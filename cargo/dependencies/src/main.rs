use rand::Rng;
use winit::{window, event, event_loop::EventLoop};
fn main() {
    let mut eventLoop = EventLoop::new();
    let mut window = window::WindowBuilder::new()
    .with_title("Cargo Dependencies")
    .with_resizable(true)
    .build(&eventLoop);

    eventLoop.run(move |a, b, c| {

    });
}
