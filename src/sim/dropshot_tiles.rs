#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/Arena/DropshotTiles/DropshotTiles.h");

        type DropshotTileState;
        type DropshotTilesState;
    }

    #[derive(Clone, Copy, Debug)]
    struct DropshotTileState {
        damage_state: u8,
    }

    #[derive(Clone, Copy, Debug)]
    struct DropshotTilesState {
        states: [[DropshotTileState; 70]; 2],
    }
}

pub use base::{DropshotTileState, DropshotTilesState};
