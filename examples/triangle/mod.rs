use webgpu_game_engine::{
    engine::{service::EnabledServices, winit::{run_winit, WinitSettings}},
    renderer::{
        light::Light,
        material::{Material, PhongMaterial},
    },
    scene::{camera::Camera, mesh::Mesh, object3d::Object3D, scene::Scene}, util::{frame_delta::get_frame_delta, orbit_controls::{init_orbit_controls, orbit_controls}, simple_axes::simple_axes},
};

const TRIANGLE_VERTICES: [f32; 36] = [
    0.0, 0.8, 0.1, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.693, -0.4, 0.1, 1.0, 0.0, 0.0,
    1.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.693, -0.4, 0.1, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.1, 0.0, 0.0,
];

const TRIANGLE_INDICES: [u32; 3] = [0, 1, 2];

// we will write it down later
static mut SCENE: Option<Scene> = None;

pub fn main() {
    println!("Hello, world!");
    let light = Light::point_light(glm::vec4(0.0, 0.0, 0.4, 1.0), glm::vec3(1.0, 1.0, 1.0));
    let vertex_array = Box::new(TRIANGLE_VERTICES);
    let index_array = Box::new(TRIANGLE_INDICES);
    let material = Material::PhongMaterial(PhongMaterial::new_without_texture(
        glm::Vec3::new(0.0, 0.0, 0.0),
        glm::Vec3::new(1.0, 0.5, 0.0),
        glm::Vec3::new(1.0, 1.0, 1.0),
        5.0,
    ));
    let object3d = Mesh::new_object_3d(None, vertex_array, index_array, material);
    let axes3d = simple_axes();
    let mut scene_root = Object3D::create_empty();
    scene_root.children.push(object3d);
    scene_root.children.push(axes3d);
    let camera = Camera::PerspectiveCamera {
        world_to_local: glm::ext::look_at(
            glm::vec3(4.0, 0.5, 0.5),
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(0.0, 1.0, 0.0),
        ),
        near: 0.5,
        far: 10.0,
        aspect: 1.5,
    };
    unsafe {
        let _ = SCENE.insert({
            let mut my_scene = Scene::new();
            my_scene.camera = camera;
            my_scene.lights.push(light);
            my_scene.root = scene_root;
            my_scene
        });
    }

    let mut enabled_services = EnabledServices::default();

    init_orbit_controls(&mut enabled_services);

    run_winit(
        &WinitSettings {
            window_width: 800,
            window_height: 600,
        },
        enabled_services,
        render_loop,
    )
}

fn render_loop(current_time: f64) -> &'static mut Scene {
    // I don't care.
    let scene = unsafe { SCENE.as_mut().unwrap() };

    let delta = get_frame_delta(current_time);
    orbit_controls(&mut scene.camera, delta);

    scene
}
