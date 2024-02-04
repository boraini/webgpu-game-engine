use std::{path::Path, error::Error, result::Result, fmt};
use obj::{self, Obj, ObjMaterial};
use crate::renderer::material::{Material, PhongMaterial, MaterialManager};
use crate::importer::defaults::default_material;
use crate::scene::object3d::{Object3D};
use glm::Vector3;

type LoadObjResult = Result<Object3D, Box<dyn Error>>;

#[derive(Debug)]
struct LoadObjError(String);

impl fmt::Display for LoadObjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error loading the obj file: {}", self.0)
    }
}

impl Error for LoadObjError {}

fn obj_material_to_renderer_material(mat : &obj::Material, material_manager: &mut MaterialManager) -> Material {
    let Ka = mat.ka.map_or(Vector3 {x: 0., y: 0., z: 0.}, |a| Vector3 {x: a[0], y: a[1], z: a[2]} );
    let Ks = mat.ka.map_or(Vector3 {x: 1., y: 1., z: 1.}, |a| Vector3 {x: a[0], y: a[1], z: a[2]} );
    
    let Kd = mat.kd.map_or(Vector3 {x: 1.0, y: 0.2, z: 0.2}, |a| Vector3 {x: a[0], y: a[1], z: a[2]} );
    let shininess = mat.ns.unwrap_or(1.0);
    Material::PhongMaterial(PhongMaterial::new_without_texture(Ka, Kd, Ks, shininess))
}

pub fn load_obj<P>(filename: P, material_manager: &mut MaterialManager) -> LoadObjResult where P: AsRef<Path> {
    let mut loaded = Obj::load(filename)?;
    let mut objects: Vec<Object3D> = vec!();
    // this will replace the material entries INSIDE OBJECT GROUPS automatically.
    let assign_materials = loaded.load_mtls().is_ok();

    // Do whatever you want
    for object in &loaded.data.objects {
        dbg!(object);

        // Each group is compiled as a child of a representative.
        let mut groups: Vec<Object3D> = vec!();
        for group in &object.groups {
            let material = if !assign_materials {default_material()} else {match &group.material {
                Some(obj_material) => match obj_material {
                    ObjMaterial::Mtl(mat) => obj_material_to_renderer_material(&mat, material_manager),
                    ObjMaterial::Ref(x) => return Err(Box::new(LoadObjError(format!("Material {} was supposed to be loaded.", x)))),
                },
                None => default_material(),
            }};
        }
    }

    todo!();
}