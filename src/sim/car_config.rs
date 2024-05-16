#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/Car/CarConfig/CarConfig.h");

        #[rust_name = "Vec3"]
        type Vec = crate::math::Vec3;

        type WheelPairConfig;
        type CarConfig;
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct WheelPairConfig {
        wheel_radius: f32,
        suspension_rest_length: f32,
        connection_point_offset: Vec3,
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct CarConfig {
        hitbox_size: Vec3,
        hitbox_pos_offset: Vec3,
        front_wheels: WheelPairConfig,
        back_wheels: WheelPairConfig,
        dodge_deadzone: f32,
    }
}

pub use base::{CarConfig, WheelPairConfig};
