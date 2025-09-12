use crate::math::Vec3;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WheelPairConfig {
    pub wheel_radius: f32,
    pub suspension_rest_length: f32,
    pub connection_point_offset: Vec3,
}

unsafe impl cxx::ExternType for WheelPairConfig {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::WheelPairConfig");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CarConfig {
    pub hitbox_size: Vec3,
    pub hitbox_pos_offset: Vec3,
    pub front_wheels: WheelPairConfig,
    pub back_wheels: WheelPairConfig,
    pub three_wheels: bool,
    pub dodge_deadzone: f32,
}

unsafe impl cxx::ExternType for CarConfig {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::CarConfig");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/Car/CarConfig/CarConfig.h");

        type WheelPairConfig = crate::sim::WheelPairConfig;
        type CarConfig = crate::sim::CarConfig;
    }
}
