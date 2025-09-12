use crate::math::Vec3;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde_utils", derive(serde::Serialize, serde::Deserialize))]
pub enum ArenaMemWeightMode {
    #[default]
    Heavy,
    Light,
}

unsafe impl cxx::ExternType for ArenaMemWeightMode {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::ArenaMemWeightMode");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ArenaConfig {
    pub mem_weight_mode: ArenaMemWeightMode,
    pub min_pos: Vec3,
    pub max_pos: Vec3,
    pub max_aabb_len: f32,
    pub no_ball_rot: bool,
    pub use_custom_broadphase: bool,
    pub max_objects: u32,
}

unsafe impl cxx::ExternType for ArenaConfig {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("EArenaConfig");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
mod base {
    unsafe extern "C++" {
        include!("arenar.h");

        type EArenaConfig = crate::sim::ArenaConfig;
        #[namespace = "RocketSim"]
        type ArenaMemWeightMode = crate::sim::ArenaMemWeightMode;
    }
}
