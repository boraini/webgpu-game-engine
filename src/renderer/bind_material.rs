use wgpu::RenderPass;

use super::material::Material;

pub trait BindMaterial<'a> {
    fn bind_material(&mut self, material: &'a Material);
}

impl<'a> BindMaterial<'a> for RenderPass<'a> {
    fn bind_material(&mut self, material: &'a Material) {
        match material {
            Material::PhongMaterial(mat) => {
                self.set_bind_group(1, mat.bind_group.as_ref().unwrap(), &[]);
            }
            Material::PhongMaterialWithTexture(_mat) => todo!(),
        }
    }
}
