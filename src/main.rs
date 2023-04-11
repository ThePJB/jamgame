mod math;
mod game;
mod vertex_buffer;

fn main() {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        let mut game = game::Game::new(&event_loop);
        event_loop.run(move |event, _, _| game.handle_event(event));
    }
}