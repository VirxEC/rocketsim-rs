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

#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/Car/Car.h");

        #[rust_name = "Vec3"]
        type Vec = crate::math::Vec3;
        type RotMat = crate::math::RotMat;
        type CarControls = crate::sim::CarControls;
        type BallHitInfo = crate::sim::BallHitInfo;

        type CarState;

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

    #[derive(Clone, Copy, Debug)]
    struct WorldContact {
        has_contact: bool,
        contact_normal: Vec3,
    }

    #[derive(Clone, Copy, Debug)]
    struct CarContact {
        other_car_id: u32,
        cooldown_timer: f32,
    }

    #[derive(Clone, Copy, Debug)]
    struct CarState {
        pos: Vec3,
        rot_mat: RotMat,
        vel: Vec3,
        ang_vel: Vec3,
        update_counter: u64,
        is_on_ground: bool,
        wheels_with_contact: [bool; 4],
        has_jumped: bool,
        has_double_jumped: bool,
        has_flipped: bool,
        flip_rel_torque: Vec3,
        jump_time: f32,
        flip_time: f32,
        is_flipping: bool,
        is_jumping: bool,
        air_time: f32,
        air_time_since_jump: f32,
        boost: f32,
        time_spent_boosting: f32,
        is_supersonic: bool,
        supersonic_time: f32,
        handbrake_val: f32,
        is_auto_flipping: bool,
        auto_flip_timer: f32,
        auto_flip_torque_scale: f32,
        world_contact: WorldContact,
        car_contact: CarContact,
        is_demoed: bool,
        demo_respawn_timer: f32,
        ball_hit_info: BallHitInfo,
        last_controls: CarControls,
    }

    impl UniquePtr<CarState> {}
    impl CxxVector<CarState> {}
}

pub use base::{CarContact, CarState, WorldContact};
