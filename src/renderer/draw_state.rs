use std::sync::Arc;

use super::material::{MaterialManager, MaterialType};

pub struct DrawState {
    pub matrix_stack: Vec<glm::Matrix4<f32>>,
    pub current_material: MaterialType,
    pub material_manager: Arc<MaterialManager>,
}

impl DrawState {
    pub fn new(current_material: MaterialType, material_manager: Arc<MaterialManager>) -> Self {
        DrawState {
            matrix_stack: vec![crate::util::identity_matrix()],
            current_material,
            material_manager,
        }
    }

    pub fn transform(&mut self, matrix: &glm::Matrix4<f32>) {
        let edited = self.matrix_stack.pop().unwrap();
        let multiplied = matrix.mul_m(&edited);
        self.matrix_stack.push(multiplied);
    }

    pub fn push_matrix(&mut self) {
        self.matrix_stack.push(*self.get_matrix());
    }

    pub fn pop_matrix(&mut self) {
        self.matrix_stack.pop();
    }

    pub fn get_matrix(&self) -> &glm::Matrix4<f32> {
        self.matrix_stack.last().unwrap()
    }
}
