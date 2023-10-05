//! Create a Window and give it a uniform background color.
fn clear_screen(
    device: &wgpu::Device,
    window_texture_view: &wgpu::TextureView,
    queue: &wgpu::Queue,
) {
    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        // RenderPass is a relatively high level rendering concept in WGPU.
        // You wont get much rendering done without a RenderingPass.
        // A RenderPass consists of color attachments and depth stencil attachments.
        let _render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            // RenderPassColorAttachments focus on rendering to specific textures.
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: window_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 0.0,
                    }),
                    // We have to store what we render to the texture so we can present it.
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
    }
    queue.submit([command_encoder.finish()]);
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window_builder = winit::window::WindowBuilder::new();
    let window = window_builder.build(&event_loop).unwrap();

    // For WGPU:
    // Create Instance
    // Create Adapter
    // Create Surface
    // Create Device / Queue
    // Create CommandEncoder
    // Create RenderPass
    // Configure RenderPass to clear screen

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter =
        pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
            .unwrap();
    let (device, queue) =
        pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
            .unwrap();
    // create_surface is unsafe because the window must remain valid as long as the surface lives.
    // We have to ensure this ourselves.
    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    // surface will be moved into the loop.
    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::NewEvents(_) => {
                // Emitted when a new batch of Events is beginning to be processed.
            }
            winit::event::Event::WindowEvent {
                window_id: _,
                event: window_event,
            } => {
                // Events related to this specific Window.
                use winit::event::{KeyboardInput, VirtualKeyCode, WindowEvent};
                match window_event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => control_flow.set_exit(),
                    _ => {}
                }
            }
            winit::event::Event::DeviceEvent {
                device_id: _,
                event: _,
            } => {
                // Events related to this device, independent of any Window.
            }
            winit::event::Event::UserEvent(_) => {
                // User defined Events.
                panic!("Received an unexpected user defined event");
            }
            winit::event::Event::Suspended => {
                // Suspend Event.
            }
            winit::event::Event::Resumed => {
                // Resumed Event.
            }
            winit::event::Event::MainEventsCleared => {
                // Emitted after input events are cleared.
                // Should maybe be called InputEventsCleared.
                // This is a good place to call into a game loop iteration.

                // We must configure the surface before we can do much with it.
                // surface.get_current_texture() will fail if we do not configure first.
                surface.configure(
                    &device,
                    &wgpu::SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        width: window.inner_size().width,
                        height: window.inner_size().height,
                        present_mode: wgpu::PresentMode::AutoVsync,
                        alpha_mode: wgpu::CompositeAlphaMode::Auto,
                        view_formats: vec![wgpu::TextureFormat::Bgra8UnormSrgb],
                    },
                );
                // The surface.get_current_texture() docs say it
                // "returns the next texture to be presented by the swapchain".
                // At the highest level, I suppose we simply render to this texture
                // and then present it.
                let surface_texture = surface.get_current_texture().unwrap();
                let texture_view = surface_texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                clear_screen(&device, &texture_view, &queue);
                surface_texture.present();
            }
            winit::event::Event::RedrawRequested(_window_id) => {
                // Emitted when the OS requests a redraw, or the application calls Window::request_redraw.
                // The OS will request redraw for window resizing, etc.
                // There might be multiple Windows and multiple RedrawRequests.
                // Note that each RedrawRequest contains a WindowId.
            }
            winit::event::Event::RedrawEventsCleared => {
                // Emitted after RedrawRequests.
                // This is the last event in a normal event loop iteration.
            }
            winit::event::Event::LoopDestroyed => {
                // Emitted when the program is ending.
                // This is the last event emitted by the application.
            }
        }
    });
}
