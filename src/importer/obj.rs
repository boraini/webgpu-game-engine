use std::collections::HashMap;
use std::{path::Path, error::Error, result::Result, fmt};
use obj::{self, IndexTuple, Obj, ObjMaterial, SimplePolygon};
use crate::renderer::material::{Material, PhongMaterial};
use crate::importer::defaults::default_material;
use crate::scene::mesh::{Mesh, VertexBufferObject};
use crate::scene::object3d::Object3D;
use crate::util::boxed_slice;
use glm::Vector3;

type LoadObjErrorBranchType = Box<dyn Error>;
type LoadObjResult = Result<Object3D, LoadObjErrorBranchType>;

const DEFAULT_UV: [f32; 2] = [0.0, 0.0];

#[derive(Debug)]
struct LoadObjError(String);

impl fmt::Display for LoadObjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error loading the obj file: {}", self.0)
    }
}

impl Error for LoadObjError {}

fn obj_material_to_renderer_material(mat : &obj::Material) -> Material {
    let ka = mat.ka.map_or(Vector3 {x: 0., y: 0., z: 0.}, |a| Vector3 {x: a[0], y: a[1], z: a[2]} );
    let ks = mat.ka.map_or(Vector3 {x: 1., y: 1., z: 1.}, |a| Vector3 {x: a[0], y: a[1], z: a[2]} );
    
    let kd = mat.kd.map_or(Vector3 {x: 1.0, y: 0.2, z: 0.2}, |a| Vector3 {x: a[0], y: a[1], z: a[2]} );
    let map_kd = &mat.map_kd;
    let shininess = mat.ns.unwrap_or(1.0);
    match &map_kd {
        Some(name) => Material::PhongMaterialWithTexture(PhongMaterial::new_with_texture(ka, kd, ks, shininess, name)),
        None => Material::PhongMaterial(PhongMaterial::new_without_texture(ka, kd, ks, shininess)),
    }
}

fn split_poly(poly: &SimplePolygon) -> Result<Vec<[IndexTuple; 3]>, LoadObjErrorBranchType> {
    if poly.0.len() < 3 {
        return Err(Box::new(LoadObjError("there were supposed to be at least 3 vertices.".to_string())))
    }

    let start_vertex = poly.0[0];

    let mut result: Vec<[IndexTuple; 3]> = vec!();

    for idx in 1..poly.0.len() - 1 {
        result.push([start_vertex, poly.0[idx], poly.0[idx + 1]]);
    }

    Ok(result)
}

fn create_vertex_and_index_buffer(loaded: &obj::Obj, polygons: &[SimplePolygon]) -> (Box<[f32]>, Box<[u32]>) {
    // TODO: Achieve more efficient loading.
    struct VertexMapItem(usize, VertexBufferObject);
    let mut vertex_map: HashMap<IndexTuple, VertexMapItem> = HashMap::new();

    let mut indices: Vec<u32> = Vec::with_capacity(polygons.len() * 3);

    let mut index_counter = 1;

    for poly in polygons.iter().flat_map(split_poly).flat_map(|a| a) {
        let calculated_normal = {
            let v0a = loaded.data.position[poly[0].0];
            let v1a = loaded.data.position[poly[1].0];
            let v2a = loaded.data.position[poly[2].0];

            let v0 = glm::vec3(v0a[0], v0a[1], v0a[2]);
            let v1 = glm::vec3(v1a[0], v1a[1], v1a[2]);
            let v2 = glm::vec3(v2a[0], v2a[1], v2a[2]);

            let n = glm::cross(v1 - v0, v2 - v0);
            
            [n.x, n.y, n.z]
        };

        for i in 0..3 {
            let idx = match vertex_map.entry(poly[i]) {
                std::collections::hash_map::Entry::Occupied(entry) => entry.get().0,
                std::collections::hash_map::Entry::Vacant(_) => {
                    let my_index = index_counter;
                    index_counter += 1;

                    let position = &loaded.data.position[poly[i].0];

                    let normal = match poly[i].2 {
                        Some(idx) => &loaded.data.normal[idx],
                        None => &calculated_normal,
                    };

                    let uv = match poly[i].1 {
                        Some(idx) => &loaded.data.texture[idx],
                        None => &DEFAULT_UV,
                    };

                    vertex_map.insert(poly[i], VertexMapItem(my_index, VertexBufferObject::new_from_slices(position, normal, uv)));

                    my_index
                },
            };

            indices.push(idx as u32);
        }
    }

    let mut vbos: Box<[f32]> = boxed_slice::alloc_box_buffer(index_counter * std::mem::size_of::<VertexBufferObject>());

    let vbos_view: &mut [VertexBufferObject] = bytemuck::cast_slice_mut(&mut vbos);

    for (_, pair) in vertex_map.iter() {
        vbos_view[pair.0] = pair.1;
    }

    (vbos, indices.into_boxed_slice())
}

pub fn load_obj<P>(filename: P) -> LoadObjResult where P: AsRef<Path> {
    let mut loaded = Obj::load(filename)?;
    dbg!(&loaded);

    // this will replace the material entries INSIDE OBJECT GROUPS automatically.
    let assign_materials = loaded.load_mtls().is_ok();

    let mut objects = Object3D::create_empty();

    // Do whatever you want
    for object in &loaded.data.objects {
        // Each group is compiled as a child of a representative.
        let mut groups = Object3D::create_empty();

        for group in &object.groups {
            let material = if !assign_materials {default_material()} else {match &group.material {
                Some(obj_material) => match obj_material {
                    ObjMaterial::Mtl(mat) => obj_material_to_renderer_material(&mat),
                    ObjMaterial::Ref(x) => return Err(Box::new(LoadObjError(format!("Material {} was supposed to be loaded.", x)))),
                },
                None => default_material(),
            }};

            let (vertex_array, index_array) = create_vertex_and_index_buffer(&loaded, &group.polys);

            let object3d = Mesh::new_object_3d(Some(group.name.clone()), vertex_array, index_array, material);

            groups.children.push(object3d);
        }

        objects.children.push(groups);
    }

    Ok(objects)
}
