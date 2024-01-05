use std::num::NonZeroU32;
use window_clipboard::Clipboard;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface =
        unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let clipboard = Clipboard::connect(&window).expect("Connect to clipboard");

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let (width, height) = {
                let size = window.inner_size();
                (size.width, size.height)
            };
            surface
                .resize(
                    NonZeroU32::new(width).unwrap(),
                    NonZeroU32::new(height).unwrap(),
                )
                .unwrap();
            let buffer = surface.buffer_mut().unwrap();
            buffer.present().unwrap();
        }
        Event::MainEventsCleared => {
            println!("{:?}", clipboard.read());
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => *control_flow = ControlFlow::Exit,
        _ => *control_flow = ControlFlow::Wait,
    });
}
