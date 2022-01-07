pub mod board;
pub mod engine;
pub mod minmax;

use engine::Engine;

fn main() {
    let mut e = Engine::new();
    e.run();
}
