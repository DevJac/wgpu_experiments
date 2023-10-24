use rand::{seq::SliceRandom, Rng};

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct Vec3f {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct Vec2f {
    x: f32,
    y: f32,
}

impl Vec2f {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct OurStruct {
    color: Vec3f,
    _padding: [u8; 4],
    scale: Vec2f,
    offset: Vec2f,
}

struct WgpuStuff {
    window: winit::window::Window,
    surface: wgpu::Surface,
    preferred_format: wgpu::TextureFormat,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl WgpuStuff {
    fn new(window: winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::None,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let preferred_format = surface_capabilities.formats[0];
        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
                .unwrap();
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/uniforms.wgsl").into()),
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vertex_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fragment_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: preferred_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });
        const OBJECT_COUNT: usize = 100;
        const OUR_STRUCT_SIZE: usize = std::mem::size_of::<OurStruct>();
        let mut bind_groups = Vec::with_capacity(OBJECT_COUNT);
        let mut rng = rand::thread_rng();
        for _object_index in 0..OBJECT_COUNT {
            let our_struct = OurStruct {
                color: Vec3f::new(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                ),
                scale: Vec2f::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)),
                offset: Vec2f::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
                _padding: [0; 4],
            };
            let our_struct_bytes: &[u8; OUR_STRUCT_SIZE] = bytemuck::cast_ref(&our_struct);
            let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: OUR_STRUCT_SIZE as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: true,
            });
            buffer
                .slice(..)
                .get_mapped_range_mut()
                .copy_from_slice(our_struct_bytes);
            buffer.unmap();
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &render_pipeline.get_bind_group_layout(0),
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &buffer,
                        offset: 0,
                        size: None,
                    }),
                }],
            });
            bind_groups.push(bind_group);
        }
        let result = WgpuStuff {
            window,
            surface,
            preferred_format,
            device,
            queue,
            render_pipeline,
            bind_groups,
        };
        result.configure_surface();
        result
    }

    fn configure_surface(&self) {
        let window_size = self.window.inner_size();
        self.surface.configure(
            &self.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.preferred_format,
                width: window_size.width,
                height: window_size.height,
                present_mode: wgpu::PresentMode::AutoNoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![self.preferred_format],
            },
        );
    }

    fn render(&self) {
        let surface_texture: wgpu::SurfaceTexture = self.surface.get_current_texture().unwrap();
        let texture_view: wgpu::TextureView = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.3,
                            g: 0.3,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            let mut rng = rand::thread_rng();
            for bind_group in self
                .bind_groups
                .choose_multiple(&mut rng, self.bind_groups.len())
            {
                render_pass.set_bind_group(0, bind_group, &[]);
                render_pass.draw(0..3, 0..1);
            }
        }
        self.queue.submit([command_encoder.finish()]);
        surface_texture.present();
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    let wgpu_stuff = WgpuStuff::new(window);
    let start = std::time::Instant::now();
    let mut completed_renders: u64 = 0;

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent {
            window_id: _,
            event: window_event,
        } => match window_event {
            winit::event::WindowEvent::CloseRequested
            | winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => control_flow.set_exit(),
            winit::event::WindowEvent::Resized(_) => {
                wgpu_stuff.configure_surface();
            }
            _ => {}
        },
        winit::event::Event::MainEventsCleared => {
            wgpu_stuff.render();
            completed_renders += 1;
            if completed_renders % 100 == 0 {
                println!(
                    "FPS: {:.0}",
                    1.0 / ((std::time::Instant::now() - start).as_secs_f32()
                        / completed_renders as f32)
                );
            }
        }
        _ => {}
    });
}
