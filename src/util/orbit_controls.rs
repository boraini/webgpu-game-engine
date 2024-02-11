use glm::GenSquareMat;

use crate::{engine::{mouse_service::MouseService, service::{EnabledServices, ServiceType}}, scene::camera::Camera};

pub fn init_orbit_controls(enabled_services: &mut EnabledServices) {
    enabled_services.enable(ServiceType::MouseService);
}

pub fn orbit_controls(camera: &mut Camera, _delta: f64) {
    let mouse_service = MouseService::get();
    if !mouse_service.is_down { return; }
    let delta_azimuth = (mouse_service.delta_x / 200.0) as f32;
    let delta_pitch = (mouse_service.delta_y / 200.0) as f32;

    let world_to_local = match &camera {
        Camera::PerspectiveCamera { world_to_local, near: _, far: _, aspect: _ } => world_to_local,
    };

    let camera_pos = (world_to_local.inverse().unwrap().mul_v(&glm::vec4(0.0, 0.0, 0.0, 1.0))).truncate(3);
    let center_pos = glm::vec4(0.0, 0.0, 0.0, 0.0).truncate(3);
    let diff = camera_pos - center_pos;
    let pitch_axis = glm::normalize(glm::cross(glm::vec3(0.0, 1.0, 0.0), diff));
    
    let translate_backward = glm::ext::translate(&world_to_local, -center_pos);
    let rotate_pitch = glm::ext::rotate(&translate_backward, delta_pitch, pitch_axis );
    let rotate_azimuth = glm::ext::rotate(&rotate_pitch, delta_azimuth, glm::vec3(0.0, 1.0, 0.0));
    let translate_forward = glm::ext::translate(&rotate_azimuth, center_pos);

    let tranformation = translate_forward;

    match camera {
        Camera::PerspectiveCamera { world_to_local, near: _, far: _, aspect: _ } => {
            *world_to_local = tranformation;
        }
    }
}