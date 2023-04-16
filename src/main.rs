mod math;
mod game;
mod vertex_buffer;
mod gems;
mod enemies;
mod player;

fn main() {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        let mut game = game::Game::new(&event_loop);
        game.populate_with_gems();
        event_loop.run(move |event, _, _| game.handle_event(event));
    }
}