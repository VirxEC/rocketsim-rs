#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/Arena/DropshotTiles/DropshotTiles.h");

        #[rust_name = "Vec3"]
        type Vec = crate::math::Vec3;

        type DropshotTileState;
        type DropshotTilesState;

        #[namespace = "RocketSim::DropshotTiles"]
        fn GetTilePos(team: i32, index: i32) -> Vec3;
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

pub use base::{DropshotTileState, DropshotTilesState, GetTilePos};
