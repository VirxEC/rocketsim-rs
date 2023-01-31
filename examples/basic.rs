use autocxx::WithinUniquePtr;
use rocketsim_rs::sim::arena::{Arena, GameMode};

fn main() {
    let mut arena = Arena::new(GameMode::SOCCAR, 120.).within_unique_ptr();
    println!("Arena tick rate: {}", arena.pin_mut().GetTickRate());
}
