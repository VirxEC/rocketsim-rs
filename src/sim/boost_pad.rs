use crate::math::Vec3;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BoostPadState {
    pub is_active: bool,
    pub cooldown: f32,
    pub cur_locked_car_id: u32,
    pub prev_locked_car_id: u32,
}

unsafe impl cxx::ExternType for BoostPadState {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("EBoostPadState");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BoostPadConfig {
    pub position: Vec3,
    pub is_big: bool,
}

unsafe impl cxx::ExternType for BoostPadConfig {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::BoostPadConfig");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
mod boostpadstate {
    unsafe extern "C++" {
        include!("arenar.h");

        type EBoostPadState = crate::sim::BoostPadState;
        #[namespace = "RocketSim"]
        type BoostPadConfig = crate::sim::BoostPadConfig;
    }
}
