//! The built-in gomp renderer.
//!
//! # For clients
//! Clients should never need to interact with this. Your application instance should call the
//! renderer whenever nessisary.
//!
//! # For developers
//! Most of these functions are pretty self explanitory. To illustrate this, below are some
//! functions and their purpose. You will see the API is pretty intuitive.
//!
//! | Function | Purpose |
//! |---|---|
//! | `new` | Creates a new renderer. |
//! | `resize` | Handles window resize events. |
//! | `render` | Renders a frame. |

pub mod vertex;
pub mod shaders;
pub mod render;

use shaders::ShaderBuilder;
use vertex::Vertex;

use winit::window::Window;
use log::debug;

#[derive(Debug)]
pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    /// Creates a new renderer. Compiles shaders, gets devices,
    /// make queues, and *all that jazz*.
    pub async fn new(window: &Window) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        // Get the physical size of the window.
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        debug!("Getting a GPU handle (instance)");
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        // Get a device and a command queue.
        debug!("Grabbing a device");
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::POLYGON_MODE_LINE,
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        // Configure our surface
        debug!("Configuring surface");
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        // Lets compile our shaders
        debug!("Compiling default vertex shader");
        let vertex_shader = ShaderBuilder::new()
            .with_label("Default vertex shader")
            .with_source(shaders::ShaderSourceType::Wgsl, shaders::vertex::DEFAULT_VERTEX_SOURCE)
            .with_entry_point("main") // this is technically the default
            .with_device(&device)
            .compile()?;

        debug!("Compiling default fragment shader");
        let fragment_shader = ShaderBuilder::new()
            .with_label("Default fragment shader")
            .with_source(shaders::ShaderSourceType::Wgsl, shaders::fragment::DEFAULT_FRAGMENT_SOURCE)
            .with_entry_point("main")
            .with_device(&device)
            .compile()?;

        debug!("Creating render pipeline layout");
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Default render pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        debug!("Creating render pipeline with the just-now-created layout");
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            // Specify vertex instructions.
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "main",
                buffers: &[
                    Vertex::desc(),
                ],
            },
            // Specify fragment instructions.
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false, // 4.
            },
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
        })
    }

    /// Window resize event handler.
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}

