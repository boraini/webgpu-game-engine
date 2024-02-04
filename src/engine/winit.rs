use crate::{
    renderer::{
        material::MaterialManager, render::render_to_texture_view, wgpu_handles::WgpuHandles,
    },
    scene::scene::Scene,
};
use std::{collections::HashMap, sync::Arc, time::Instant};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowId},
};

pub struct WinitSettings {
    pub window_width: u32,
    pub window_height: u32,
}

pub fn run_winit<'a, F>(settings: &WinitSettings, render_loop: F)
where
    F: Fn(f64) -> &'a mut Scene,
{
    pollster::block_on(run_winit_async(settings, render_loop));
}

pub async fn run_winit_async<'a, F>(settings: &WinitSettings, render_loop: F)
where
    F: Fn(f64) -> &'a mut Scene,
{
    let event_loop = EventLoop::new().unwrap();
    let mut viewports = Vec::with_capacity(1usize);

    let window = winit::window::WindowBuilder::new()
        .with_title("Scene".to_string())
        .with_inner_size(winit::dpi::PhysicalSize::new(
            settings.window_width,
            settings.window_height,
        ))
        .build(&event_loop)
        .unwrap();
    let window = Arc::new(window);

    viewports.push((
        window,
        wgpu::Color {
            r: 1.0,
            g: 0.6,
            b: 0.8,
            a: 1.0,
        },
    ));

    let instance = wgpu::Instance::default();
    let viewports: Vec<_> = viewports
        .into_iter()
        .map(|(window, color)| ViewportDesc::new(window, color, &instance))
        .collect();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            // Request an adapter which can render to our surface
            compatible_surface: viewports.first().map(|desc| &desc.surface),
            ..Default::default()
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    println!("Max Bind Groups: {}", device.limits().max_bind_groups);
    let device = Arc::new(device);

    let mut viewports: HashMap<WindowId, Viewport> = viewports
        .into_iter()
        .map(|desc| (desc.window.id(), desc.build(&adapter, &device)))
        .collect();

    let mut material_manager = MaterialManager::new();
    let surface_capabilities = viewports
        .iter_mut()
        .next()
        .unwrap()
        .1
        .get_surface_capabilities(&adapter);
    material_manager.populate(&device, &surface_capabilities);
    let mut material_manager = Arc::new(material_manager);

    let mut wgpu_handles = WgpuHandles {
        adapter,
        instance,
        device: device.clone(),
        queue,
        material_manager: material_manager.clone(),
    };

    let start_instant = Instant::now();

    env_logger::init();
    event_loop
        .run(move |event, target| {
            // Have the closure take ownership of the resources.
            // `event_loop.run` never returns, therefore we must do this to ensure
            // the resources are properly cleaned up.
            let _ = (&wgpu_handles, &device, &material_manager);

            if let Event::WindowEvent { window_id, event } = event {
                match event {
                    WindowEvent::Resized(new_size) => {
                        // Recreate the swap chain with the new size
                        if let Some(viewport) = viewports.get_mut(&window_id) {
                            viewport.resize(&device, new_size);
                            // On macos the window needs to be redrawn manually after resizing
                            viewport.desc.window.request_redraw();
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        if let Some(viewport) = viewports.get_mut(&window_id) {
                            let frame = viewport.get_current_texture();
                            let texture_view = frame
                                .texture
                                .create_view(&wgpu::TextureViewDescriptor::default());
                            let scene = render_loop(start_instant.elapsed().as_secs_f64());
                            render_to_texture_view(
                                scene,
                                &mut wgpu_handles,
                                &texture_view,
                                &mut material_manager,
                            );
                            frame.present();
                        }
                    }
                    WindowEvent::CloseRequested => {
                        viewports.remove(&window_id);
                        if viewports.is_empty() {
                            target.exit();
                        }
                    }
                    _ => {}
                }
            }
        })
        .unwrap();
}

pub struct ViewportDesc {
    pub window: Arc<Window>,
    pub background: wgpu::Color,
    pub surface: wgpu::Surface<'static>,
}

pub struct Viewport {
    pub desc: ViewportDesc,
    pub config: wgpu::SurfaceConfiguration,
}

impl ViewportDesc {
    pub fn new(window: Arc<Window>, background: wgpu::Color, instance: &wgpu::Instance) -> Self {
        let surface = instance.create_surface(window.clone()).unwrap();
        Self {
            window,
            background,
            surface,
        }
    }

    pub fn build(self, adapter: &wgpu::Adapter, device: &wgpu::Device) -> Viewport {
        let size = self.window.inner_size();
        let config = self
            .surface
            .get_default_config(adapter, size.width, size.height)
            .unwrap();
        self.surface.configure(device, &config);
        Viewport { desc: self, config }
    }
}

impl Viewport {
    pub fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.desc.surface.configure(device, &self.config);
    }
    pub fn get_current_texture(&mut self) -> wgpu::SurfaceTexture {
        self.desc
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture")
    }
    pub fn get_surface_capabilities(&self, adapter: &wgpu::Adapter) -> wgpu::SurfaceCapabilities {
        self.desc.surface.get_capabilities(adapter)
    }
}

pub struct WinitHandles {
    pub viewports: HashMap<WindowId, Viewport>,
}
