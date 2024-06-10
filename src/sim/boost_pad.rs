#[cxx::bridge]
mod boostpadstate {
    unsafe extern "C++" {
        include!("arenar.h");

        #[rust_name = "Vec3"]
        #[namespace = "RocketSim"]
        type Vec = crate::math::Vec3;

        type EBoostPadState;
        #[namespace = "RocketSim"]
        type BoostPadConfig;
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct EBoostPadState {
        is_active: bool,
        cooldown: f32,
        cur_locked_car_id: u32,
        prev_locked_car_id: u32,
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct BoostPadConfig {
        position: Vec3,
        is_big: bool,
    }
}

pub use boostpadstate::{BoostPadConfig, EBoostPadState as BoostPadState};
