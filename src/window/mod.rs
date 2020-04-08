mod support;

use glutin::event::{Event, WindowEvent};
use glutin::ContextBuilder;

pub struct Window {
    width: u32,
    heigth: u32
}

impl Window {
    pub fn new(w: u32, h: u32) -> Window {
        Window{width: w, heigth: h}
    }
    pub fn main_loop(&self) {
        let el = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title("RustyRays")
            .with_inner_size(glutin::dpi::LogicalSize::from((self.width, self.heigth)));

        let windowed_context =
            ContextBuilder::new().build_windowed(wb, &el).unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        println!(
            "Pixel format of the window's GL context: {:?}",
            windowed_context.get_pixel_format()
        );

        let gl = support::load(&windowed_context.context());

        el.run(move |event, _, control_flow| {
            println!("{:?}", event);
            *control_flow = glutin::event_loop::ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(logical_size) => {
                        let dpi_factor =
                            windowed_context.window().hidpi_factor();
                        windowed_context
                            .resize(logical_size.to_physical(dpi_factor));
                    }
                    WindowEvent::RedrawRequested => {
                        gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                        windowed_context.swap_buffers().unwrap();
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}