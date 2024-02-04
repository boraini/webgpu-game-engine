use glm::GenSquareMat;

use crate::renderer::light::ViewInfoData;

pub enum Camera {
    PerspectiveCamera {
        world_to_local: glm::Mat4,
        near: f32,
        far: f32,
        aspect: f32,
    },
}

impl Camera {
    pub fn get_inverse_matrix(&self) -> glm::Mat4 {
        match self {
            Camera::PerspectiveCamera {
                world_to_local,
                near,
                far,
                aspect,
            } => glm::Mat4 {
                c0: glm::Vec4::new(1.0, 0.0, 0.0, 0.0),
                c1: glm::Vec4::new(0.0, 1.0 / aspect, 0.0, 0.0),
                c2: glm::Vec4::new(0.0, 0.0, -(far + near) / (far - near), -1.0),
                c3: glm::Vec4::new(0.0, 0.0, -2.0 * near * far / (far - near), 0.0),
            }
            .mul_m(world_to_local),
        }
    }

    pub fn get_view_info(&self) -> ViewInfoData {
        match self {
            Camera::PerspectiveCamera { world_to_local, .. } => ViewInfoData {
                position: world_to_local.inverse().unwrap() * glm::vec4(0.0, 0.0, 0.0, 1.0),
            },
        }
    }
}
