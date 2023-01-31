use autocxx::WithinUniquePtr;
use rocketsim_rs::sim::arena::{Arena, GameMode};

#[test]
fn init_arena() {
    Arena::new(GameMode::SOCCAR, 1. / 120.).within_unique_ptr();
}
