#![cfg_attr(
    all(not(any(target_arch = "x86", target_arch = "x86_64")), feature = "glam"),
    feature(portable_simd)
)]

#[cfg(feature = "bin")]
pub mod bytes;
#[cfg(feature = "glam")]
pub mod glam_ext;
#[cfg(feature = "bin")]
pub mod render;
#[cfg(feature = "serde_utils")]
mod serde_utils;
#[cfg(feature = "serde_utils")]
pub use serde;

pub mod consts;

mod ext;

pub use autocxx;
pub use cxx;
pub use ext::*;

autocxx::include_cpp! {
    #include "arenar.h"
    name!(base)
    safety!(unsafe)
    generate_pod!("RocketSim::RocketSimStage")
    generate!("RocketSim::GetStage")
}

pub use base::RocketSim::{GetStage as get_stage, RocketSimStage as Stages};

#[cxx::bridge]
mod Init {
    unsafe extern "C++" {
        include!("arenar.h");

        fn init(folder: &str);

        #[namespace = "RocketSim"]
        type RotMat = crate::math::RotMat;
        #[namespace = "RocketSim"]
        type Angle = crate::math::Angle;

        #[must_use]
        #[doc(hidden)]
        fn AngleFromRotMat(mat: RotMat) -> Angle;
    }
}

#[inline]
/// Initializes the collision mesh system for `RocketSim`
pub fn init(collision_meshes_folder: Option<&str>) {
    Init::init(collision_meshes_folder.unwrap_or("collision_meshes"));
}

#[cxx::bridge]
mod extra {
    unsafe extern "C++" {
        include!("arenar.h");

        #[namespace = "RocketSim"]
        type CarConfig = crate::sim::CarConfig;

        #[rust_name = "get_octane"]
        fn getOctane() -> &'static CarConfig;
        #[rust_name = "get_dominus"]
        fn getDominus() -> &'static CarConfig;
        #[rust_name = "get_plank"]
        fn getPlank() -> &'static CarConfig;
        #[rust_name = "get_breakout"]
        fn getBreakout() -> &'static CarConfig;
        #[rust_name = "get_hybrid"]
        fn getHybrid() -> &'static CarConfig;
        #[rust_name = "get_merc"]
        fn getMerc() -> &'static CarConfig;
    }
}

impl sim::CarConfig {
    #[inline]
    #[must_use]
    pub fn octane() -> &'static Self {
        extra::get_octane()
    }

    #[inline]
    #[must_use]
    pub fn dominus() -> &'static Self {
        extra::get_dominus()
    }

    #[inline]
    #[must_use]
    pub fn plank() -> &'static Self {
        extra::get_plank()
    }

    #[inline]
    #[must_use]
    pub fn breakout() -> &'static Self {
        extra::get_breakout()
    }

    #[inline]
    #[must_use]
    pub fn hybrid() -> &'static Self {
        extra::get_hybrid()
    }

    #[inline]
    #[must_use]
    pub fn merc() -> &'static Self {
        extra::get_merc()
    }
}

pub mod sim {
    #[cxx::bridge(namespace = "RocketSim")]
    mod carcontrols {
        unsafe extern "C++" {
            include!("Sim/CarControls.h");

            type CarControls;
        }

        #[derive(Clone, Copy, Debug, Default)]
        struct CarControls {
            pub throttle: f32,
            pub steer: f32,
            pub pitch: f32,
            pub yaw: f32,
            pub roll: f32,
            pub boost: bool,
            pub jump: bool,
            pub handbrake: bool,
        }
    }

    pub use carcontrols::CarControls;

    autocxx::include_cpp! {
        #include "arenar.h"
        name!(arena)
        safety!(unsafe)
        block!("RocketSim::CarState")
        block!("RocketSim::BallState")
        block!("EBoostPadState")
        block!("RocketSim::CarConfig")
        block!("RocketSim::CarControls")
        block!("RocketSim::Vec")
        block!("RocketSim::Team")
        block!("RocketSim::Arena")
        block!("RocketSim::MutatorConfig")
        generate_pod!("RocketSim::GameMode")
        generate_pod!("RocketSim::ArenaMemWeightMode")
        generate!("Arenar")
    }

    pub use arena::RocketSim::GameMode;

    #[cxx::bridge]
    mod arena_extra {
        unsafe extern "C++" {
            include!("arenar.h");

            type Arenar = super::Arena;
            #[namespace = "RocketSim"]
            type CarState = crate::sim::CarState;
            #[namespace = "RocketSim"]
            type BallState = crate::sim::BallState;
            type EBoostPadState = crate::sim::BoostPadState;
            #[namespace = "RocketSim"]
            type CarConfig = crate::sim::CarConfig;
            #[namespace = "RocketSim"]
            type CarControls = crate::sim::CarControls;
            #[namespace = "RocketSim"]
            type Vec = crate::math::Vec3;
            #[namespace = "RocketSim"]
            type Team = crate::sim::Team;
            #[namespace = "RocketSim"]
            type MutatorConfig = crate::sim::MutatorConfig;

            #[must_use]
            #[doc(hidden)]
            #[rust_name = "rsc"]
            fn SetCar(self: Pin<&mut Arenar>, car_id: u32, car: CarState) -> bool;
            #[must_use]
            #[rust_name = "get_cars"]
            #[doc = "Returns all of the car ids"]
            fn GetCars(self: &Arenar) -> Vec<u32>;
            #[must_use]
            #[rust_name = "get_car"]
            fn GetCar(self: Pin<&mut Arenar>, car_id: u32) -> CarState;
            #[must_use]
            #[rust_name = "add_car"]
            fn AddCar(self: Pin<&mut Arenar>, team: Team, car_config: &CarConfig) -> u32;
            #[must_use]
            #[doc(hidden)]
            #[rust_name = "rscc"]
            fn SetCarControls(self: Pin<&mut Arenar>, car_id: u32, car_controls: CarControls) -> bool;
            #[must_use]
            #[rust_name = "get_ball"]
            fn GetBall(self: Pin<&mut Arenar>) -> BallState;
            #[rust_name = "set_ball"]
            fn SetBall(self: Pin<&mut Arenar>, ball: BallState);
            #[must_use]
            #[rust_name = "get_pad_pos"]
            fn GetPadPos(self: &Arenar, index: usize) -> Vec;
            #[rust_name = "set_pad_state"]
            fn SetPadState(self: Pin<&mut Arenar>, index: usize, pad: EBoostPadState);
            #[must_use]
            #[rust_name = "get_pad_state"]
            fn GetPadState(self: &Arenar, index: usize) -> EBoostPadState;
            #[must_use]
            #[rust_name = "get_car_config"]
            fn GetCarConfig(self: &Arenar, id: u32) -> CarConfig;
            #[must_use]
            #[rust_name = "get_car_team"]
            fn GetCarTeam(self: &Arenar, id: u32) -> Team;
            #[rust_name = "set_goal_scored_callback"]
            fn SetGoalScoreCallback(
                self: Pin<&mut Arenar>,
                callback: fn(arena: Pin<&mut Arenar>, car_team: Team, user_data: usize),
                user_data: usize,
            );
            #[rust_name = "set_car_bump_callback"]
            fn SetCarBumpCallback(
                self: Pin<&mut Arenar>,
                callback: fn(arena: Pin<&mut Arenar>, bumper: u32, victim: u32, is_demo: bool, user_data: usize),
                user_data: usize,
            );
            #[must_use]
            #[rust_name = "get_mutator_config"]
            fn GetMutatorConfig(self: &Arenar) -> MutatorConfig;
            #[rust_name = "set_mutator_config"]
            fn SetMutatorConfig(self: Pin<&mut Arenar>, config: MutatorConfig);
        }
    }

    pub use arena::{Arenar as Arena, RocketSim::ArenaMemWeightMode};

    #[cxx::bridge(namespace = "RocketSim")]
    mod ballhitinfo {
        unsafe extern "C++" {
            include!("Sim/BallHitInfo/BallHitInfo.h");

            #[rust_name = "Vec3"]
            type Vec = crate::math::Vec3;
            type BallHitInfo;
        }

        #[derive(Clone, Copy, Debug, Default)]
        struct BallHitInfo {
            is_valid: bool,
            relative_pos_on_ball: Vec3,
            ball_pos: Vec3,
            extra_hit_vel: Vec3,
            tick_count_when_hit: u64,
            tick_count_when_extra_impulse_applied: u64,
        }
    }

    pub use ballhitinfo::BallHitInfo;

    #[derive(Clone, Copy, Debug)]
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

    #[cxx::bridge(namespace = "RocketSim")]
    mod ballstate {
        unsafe extern "C++" {
            include!("Sim/Ball/Ball.h");

            #[rust_name = "Vec3"]
            type Vec = crate::math::Vec3;
            type RotMat = crate::math::RotMat;
            type BallState;
            #[namespace = "RocketSim::BallState"]
            type HeatseekerInfo = crate::sim::HeatseekerInfo;
        }

        #[derive(Clone, Copy, Debug)]
        struct BallState {
            pos: Vec3,
            rot_mat: RotMat,
            vel: Vec3,
            ang_vel: Vec3,
            update_counter: u64,
            hs_info: HeatseekerInfo,
        }
    }

    pub use ballstate::BallState;

    autocxx::include_cpp! {
        #include "Sim/Car/Car.h"
        name!(car)
        safety!(unsafe)
        generate_pod!("RocketSim::Team")
    }

    pub use car::RocketSim::Team;

    #[cxx::bridge(namespace = "RocketSim")]
    mod carstate {
        unsafe extern "C++" {
            include!("Sim/Car/Car.h");

            #[rust_name = "Vec3"]
            type Vec = crate::math::Vec3;
            type RotMat = crate::math::RotMat;
            type CarControls = crate::sim::CarControls;
            type BallHitInfo = crate::sim::BallHitInfo;

            type CarState;

            #[must_use]
            #[rust_name = "has_flip_or_jump"]
            fn HasFlipOrJump(self: &CarState) -> bool;
        }

        #[derive(Clone, Copy, Debug)]
        struct CarState {
            pos: Vec3,
            rot_mat: RotMat,
            vel: Vec3,
            ang_vel: Vec3,
            update_counter: u64,
            is_on_ground: bool,
            has_jumped: bool,
            has_double_jumped: bool,
            has_flipped: bool,
            flip_rel_torque: Vec3,
            jump_time: f32,
            flip_time: f32,
            is_flipping: bool,
            is_jumping: bool,
            air_time_since_jump: f32,
            boost: f32,
            time_spent_boosting: f32,
            is_supersonic: bool,
            supersonic_time: f32,
            handbrake_val: f32,
            is_auto_flipping: bool,
            auto_flip_timer: f32,
            auto_flip_torque_scale: f32,
            has_contact: bool,
            contact_normal: Vec3,
            other_car_id: u32,
            cooldown_timer: f32,
            is_demoed: bool,
            demo_respawn_timer: f32,
            ball_hit_info: BallHitInfo,
            last_controls: CarControls,
        }

        impl UniquePtr<CarState> {}
        impl CxxVector<CarState> {}
    }

    pub use carstate::CarState;

    #[cxx::bridge(namespace = "RocketSim")]
    mod carconfig {
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

    pub use carconfig::{CarConfig, WheelPairConfig};

    #[cxx::bridge]
    mod boostpadstate {
        unsafe extern "C++" {
            include!("arenar.h");

            type EBoostPadState;
        }

        #[derive(Clone, Copy, Debug, Default)]
        struct EBoostPadState {
            is_active: bool,
            cooldown: f32,
            cur_locked_car_id: u32,
            prev_locked_car_id: u32,
        }
    }

    pub use boostpadstate::{EBoostPadState, EBoostPadState as BoostPadState};

    autocxx::include_cpp! {
        #include "Sim/MutatorConfig/MutatorConfig.h"
        name!(demo)
        safety!(unsafe)
        generate_pod!("RocketSim::DemoMode")
    }

    pub use demo::RocketSim::DemoMode;

    #[cxx::bridge(namespace = "RocketSim")]
    mod mutators {
        unsafe extern "C++" {
            include!("Sim/MutatorConfig/MutatorConfig.h");

            #[rust_name = "Vec3"]
            type Vec = crate::math::Vec3;
            type DemoMode = crate::sim::DemoMode;

            type MutatorConfig;
        }

        #[derive(Clone, Copy, Debug)]
        struct MutatorConfig {
            gravity: Vec3,
            car_mass: f32,
            car_world_friction: f32,
            car_world_restitution: f32,
            ball_mass: f32,
            ball_max_speed: f32,
            ball_drag: f32,
            ball_world_friction: f32,
            ball_world_restitution: f32,
            jump_accel: f32,
            jump_immediate_force: f32,
            boost_accel: f32,
            boost_used_per_second: f32,
            respawn_delay: f32,
            bump_cooldown_time: f32,
            boost_pad_cooldown_big: f32,
            boost_pad_cooldown_small: f32,
            car_spawn_boost_amount: f32,
            ball_hit_extra_force_scale: f32,
            bump_force_scale: f32,
            ball_radius: f32,
            unlimited_flips: bool,
            unlimited_double_jumps: bool,
            demo_mode: DemoMode,
            enable_team_demos: bool,
        }
    }

    pub use mutators::MutatorConfig;
}

pub mod math {
    #[cfg(feature = "serde_utils")]
    use serde::{Deserialize, Serialize};

    #[repr(C, align(16))]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "serde_utils", derive(Serialize, Deserialize))]
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub _w: f32,
    }

    unsafe impl cxx::ExternType for Vec3 {
        #[allow(unused_attributes)]
        #[doc(hidden)]
        type Id = cxx::type_id!("RocketSim::Vec");
        type Kind = cxx::kind::Trivial;
    }

    #[repr(C, align(16))]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "serde_utils", derive(Serialize, Deserialize))]
    pub struct RotMat {
        pub forward: Vec3,
        pub right: Vec3,
        pub up: Vec3,
    }

    unsafe impl cxx::ExternType for RotMat {
        #[allow(unused_attributes)]
        #[doc(hidden)]
        type Id = cxx::type_id!("RocketSim::RotMat");
        type Kind = cxx::kind::Trivial;
    }

    #[cxx::bridge(namespace = "RocketSim")]
    mod inner_math {
        unsafe extern "C++" {
            include!("Math/MathTypes/MathTypes.h");

            type Angle;
            type RotMat = crate::math::RotMat;

            #[must_use]
            #[rust_name = "to_rotmat"]
            fn ToRotMat(self: &Angle) -> RotMat;
        }

        #[derive(Clone, Copy, Debug, Default)]
        struct Angle {
            yaw: f32,
            pitch: f32,
            roll: f32,
        }
    }

    pub use inner_math::Angle;
}
