use crate::{
    math::{RotMat, Vec3},
    sim::{BallHitInfo, CarControls},
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde_utils", derive(serde::Serialize, serde::Deserialize))]
pub enum Team {
    #[default]
    Blue,
    Orange,
}

unsafe impl cxx::ExternType for Team {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::Team");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WorldContact {
    pub has_contact: bool,
    pub contact_normal: Vec3,
}

unsafe impl cxx::ExternType for WorldContact {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::WorldContact");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CarContact {
    pub other_car_id: u32,
    pub cooldown_timer: f32,
}

unsafe impl cxx::ExternType for CarContact {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::CarContact");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CarState {
    pub pos: Vec3,
    pub rot_mat: RotMat,
    pub vel: Vec3,
    pub ang_vel: Vec3,
    pub tick_count_since_update: u64,
    pub is_on_ground: bool,
    pub wheels_with_contact: [bool; 4],
    pub has_jumped: bool,
    pub has_double_jumped: bool,
    pub has_flipped: bool,
    pub flip_rel_torque: Vec3,
    pub jump_time: f32,
    pub flip_time: f32,
    pub is_flipping: bool,
    pub is_jumping: bool,
    pub air_time: f32,
    pub air_time_since_jump: f32,
    pub boost: f32,
    pub time_since_boosted: f32,
    pub is_boosting: bool,
    pub boosting_time: f32,
    pub is_supersonic: bool,
    pub supersonic_time: f32,
    pub handbrake_val: f32,
    pub is_auto_flipping: bool,
    pub auto_flip_timer: f32,
    pub auto_flip_torque_scale: f32,
    pub world_contact: WorldContact,
    pub car_contact: CarContact,
    pub is_demoed: bool,
    pub demo_respawn_timer: f32,
    pub ball_hit_info: BallHitInfo,
    pub last_controls: CarControls,
}

unsafe impl cxx::ExternType for CarState {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::CarState");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/Car/Car.h");

        type CarState = crate::sim::CarState;

        /// Returns if the car has flipped or jumped
        #[must_use]
        #[cxx_name = "HasFlipOrJump"]
        fn has_flip_or_jump(self: &CarState) -> bool;

        /// Returns if the car has a flip reset
        #[must_use]
        #[cxx_name = "HasFlipReset"]
        fn has_flip_reset(self: &CarState) -> bool;

        /// Returns if the car got a flip reset
        #[must_use]
        #[cxx_name = "GotFlipReset"]
        fn got_flip_reset(self: &CarState) -> bool;
    }

    impl UniquePtr<CarState> {}
    impl CxxVector<CarState> {}
}
