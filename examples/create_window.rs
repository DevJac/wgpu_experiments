//! Create a window. Close the window when escape is pressed or when the window manager requests.

fn main() {
    // Create an EventLoop first, then a Window.
    // Window requires the EventLoop.
    let event_loop = winit::event_loop::EventLoop::new();
    let window_builder = winit::window::WindowBuilder::new();
    let _window = window_builder.build(&event_loop).unwrap();

    event_loop.run(|event, _, control_flow| {
        // ControlFlow can be Poll, Wait, WaitUntil(Instant), or ExitWithCode.
        //
        // Poll is the default.
        //
        // With Poll, the EventLoop loops as fast a possible, polling for new events.
        //
        // With Wait, the EventLoop will pause until new events arrive.
        dbg!(std::time::Instant::now(), &control_flow);

        // The two most interesting Events are DeviceEvents and WindowEvents.
        //
        // DeviceEvents are specific to the device, independent of the window.
        // For example, a button being pressed. A button being pressed happens
        // regardless of what state a window is in.
        //
        // WindowEvents are speicific to a window. Moving, resizing, gaining focus, etc.
        // A button press may be specific to a window if the operating system "routes"
        // that button press to the window.
        match event {
            winit::event::Event::WindowEvent {
                window_id: _,
                event: window_event,
            } => match window_event {
                winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                winit::event::WindowEvent::KeyboardInput {
                    device_id: _,
                    input:
                        winit::event::KeyboardInput {
                            virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                            ..
                        },
                    is_synthetic: _,
                } => control_flow.set_exit(),
                _ => {}
            },
            _ => {}
        }
    });
}
