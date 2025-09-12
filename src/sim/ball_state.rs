use crate::math::{RotMat, Vec3};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde_utils", derive(serde::Serialize, serde::Deserialize))]
pub struct HeatseekerInfo {
    /// Which net the ball should seek towards;
    /// When 0, no net
    pub y_target_dir: f32,
    pub cur_target_speed: f32,
    pub time_since_hit: f32,
}

unsafe impl cxx::ExternType for HeatseekerInfo {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::BallState::HeatseekerInfo");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde_utils", derive(serde::Serialize, serde::Deserialize))]
pub struct DropshotInfo {
    /// Charge level number, which controls the radius of damage when hitting tiles
    /// 1 = damages r=1 -> 1 tile
    /// 2 = damages r=2 -> 7 tiles
    /// 3 = damages r=3 -> 19 tiles
    pub charge_level: i32,
    /// Resets when a tile is damaged
    pub accumulated_hit_force: f32,
    /// Which side of the field the ball can damage (0=none, -1=blue, 1=orange)
    pub y_target_dir: f32,
    pub has_damaged: bool,
    /// Only valid if `has_damaged`
    pub last_damage_tick: u64,
}

unsafe impl cxx::ExternType for DropshotInfo {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::BallState::DropshotInfo");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BallState {
    pub pos: Vec3,
    pub rot_mat: RotMat,
    pub vel: Vec3,
    pub ang_vel: Vec3,
    pub tick_count_since_update: u64,
    pub hs_info: HeatseekerInfo,
    pub ds_info: DropshotInfo,
}

unsafe impl cxx::ExternType for BallState {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::BallState");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/Ball/Ball.h");

        type BallState = crate::sim::BallState;
        #[namespace = "RocketSim::BallState"]
        type HeatseekerInfo = crate::sim::HeatseekerInfo;
        #[namespace = "RocketSim::BallState"]
        type DropshotInfo = crate::sim::DropshotInfo;
    }
}
