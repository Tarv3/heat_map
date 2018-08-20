use glium::glutin::{ElementState, Event, VirtualKeyCode, WindowEvent};

pub fn take_input(event: &Event, contrast: &mut f32) {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => match input.state {
                ElementState::Pressed => {
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::W) => *contrast -= 5.0,
                        Some(VirtualKeyCode::S) => *contrast += 5.0,
                        _ => (),
                    };
                    println!("contrast = {}", *contrast);
                }
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
}
