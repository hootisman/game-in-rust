
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run(){
   env_logger::init(); 
   let event_loop = EventLoop::new();
   let window = WindowBuilder::new().build(&event_loop).unwrap();

   event_loop.run(move |event, _, control_flow| match event {
       Event::WindowEvent { window_id, ref event } if window_id == window.id() => event_handler(event, control_flow),
       _ => {},
   });
}

fn event_handler(event: &WindowEvent<'_>, control_flow: &mut ControlFlow){
    match event{
        WindowEvent::CloseRequested | 
        WindowEvent::KeyboardInput { 
            input: KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::Escape), .. },
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => {},
    }
}