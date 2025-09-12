#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DropshotTileState {
    pub damage_state: u8,
}

unsafe impl cxx::ExternType for DropshotTileState {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::DropshotTileState");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DropshotTilesState {
    pub states: [[DropshotTileState; 70]; 2],
}

unsafe impl cxx::ExternType for DropshotTilesState {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::DropshotTilesState");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/Arena/DropshotTiles/DropshotTiles.h");

        #[rust_name = "Vec3"]
        type Vec = crate::math::Vec3;

        type DropshotTileState = crate::sim::DropshotTileState;
        type DropshotTilesState = crate::sim::DropshotTilesState;

        #[namespace = "RocketSim::DropshotTiles"]
        fn GetTilePos(team: i32, index: i32) -> Vec3;
    }
}

pub use base::GetTilePos;
