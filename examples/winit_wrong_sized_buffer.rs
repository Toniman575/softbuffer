use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

const BUFFER_WIDTH: usize = 256;
const BUFFER_HEIGHT: usize = 128;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&window.canvas())
            .unwrap();
    }

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let buffer = (0..(BUFFER_WIDTH * BUFFER_HEIGHT))
                    .map(|index| {
                        let y = index / BUFFER_WIDTH;
                        let x = index % BUFFER_WIDTH;
                        let red = x % 255;
                        let green = y % 255;
                        let blue = (x * y) % 255;

                        let color = blue | (green << 8) | (red << 16);

                        color as u32
                    })
                    .collect::<Vec<_>>();

                surface.set_buffer(&buffer, BUFFER_WIDTH as u16, BUFFER_HEIGHT as u16);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
