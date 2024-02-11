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
            } => glm::ext::perspective(
                90.0f32.to_radians(),
                aspect.to_owned(),
                near.to_owned(),
                far.to_owned(),
            )
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
