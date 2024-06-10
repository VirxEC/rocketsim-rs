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

#[cxx::bridge]
mod base {
    unsafe extern "C++" {
        include!("arenar.h");

        #[rust_name = "Vec3"]
        #[namespace = "RocketSim"]
        type Vec = crate::math::Vec3;

        type EArenaConfig;
        #[namespace = "RocketSim"]
        type ArenaMemWeightMode = crate::sim::ArenaMemWeightMode;
    }

    #[derive(Clone, Copy, Debug)]
    struct EArenaConfig {
        mem_weight_mode: ArenaMemWeightMode,
        min_pos: Vec3,
        max_pos: Vec3,
        max_aabb_len: f32,
        no_ball_rot: bool,
        use_custom_broadphase: bool,
        max_objects: u32,
    }
}

pub use base::EArenaConfig as ArenaConfig;
