#![cfg_attr(
    all(not(any(target_arch = "x86", target_arch = "x86_64")), feature = "glam"),
    feature(portable_simd)
)]

#[cfg(feature = "bin")]
pub mod bytes;
#[cfg(feature = "glam")]
pub mod glam_ext;
#[cfg(feature = "rlbot")]
pub mod rlbot;

pub mod consts;

mod ext;

pub use autocxx;
pub use cxx;
pub use ext::*;

autocxx::include_cpp! {
    #include "arenar.h"
    name!(base)
    safety!(unsafe)
    generate_pod!("RocketSimStage")
    generate!("RocketSim::GetStage")
}

#[cxx::bridge]
mod Init {
    unsafe extern "C++" {
        include!("arenar.h");

        fn init(folder: &str);

        type RotMat = crate::math::RotMat;
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

pub use base::{RocketSim::GetStage as get_stage, RocketSimStage as Stages};

#[cxx::bridge]
mod extra {
    unsafe extern "C++" {
        include!("arenar.h");

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
    #[cxx::bridge]
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
        block!("CarState")
        block!("BallState")
        block!("EBoostPadState")
        block!("CarConfig")
        block!("CarControls")
        block!("Vec")
        block!("Team")
        block!("Arena")
        block!("MutatorConfig")
        generate_pod!("GameMode")
        generate!("Arenar")
    }

    #[cxx::bridge]
    mod arena_extra {
        unsafe extern "C++" {
            include!("arenar.h");

            type Arenar = super::Arena;
            type CarState = crate::sim::CarState;
            type BallState = crate::sim::BallState;
            type EBoostPadState = crate::sim::BoostPadState;
            type CarConfig = crate::sim::CarConfig;
            type CarControls = crate::sim::CarControls;
            type Vec = crate::math::Vec3;
            type Team = crate::sim::Team;
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
            #[must_use]
            #[rust_name = "get_ball_rotation"]
            fn GetBallRotation(self: &Arenar) -> [f32; 4];
        }
    }

    pub use arena::{Arenar as Arena, GameMode};

    #[cxx::bridge]
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

    #[cxx::bridge]
    mod ballstate {
        unsafe extern "C++" {
            include!("Sim/Ball/Ball.h");

            #[rust_name = "Vec3"]
            type Vec = crate::math::Vec3;
            type BallState;
        }

        #[derive(Clone, Copy, Debug)]
        struct BallState {
            pos: Vec3,
            vel: Vec3,
            ang_vel: Vec3,
        }
    }

    pub use ballstate::BallState;

    autocxx::include_cpp! {
        #include "Sim/Car/Car.h"
        name!(car)
        safety!(unsafe)
        generate_pod!("Team")
    }

    #[cxx::bridge]
    mod carstate {
        unsafe extern "C++" {
            include!("Sim/Car/Car.h");

            #[rust_name = "Vec3"]
            type Vec = crate::math::Vec3;
            type RotMat = crate::math::RotMat;
            type CarControls = crate::sim::CarControls;
            type BallHitInfo = crate::sim::BallHitInfo;

            type CarState;
        }

        #[derive(Clone, Copy, Debug)]
        struct CarState {
            pos: Vec3,
            rot_mat: RotMat,
            vel: Vec3,
            ang_vel: Vec3,
            is_on_ground: bool,
            has_jumped: bool,
            has_double_jumped: bool,
            has_flipped: bool,
            last_rel_dodge_torque: Vec3,
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

    pub use car::Team;
    pub use carstate::CarState;

    #[cxx::bridge]
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
        generate_pod!("DemoMode")
    }

    pub use demo::DemoMode;

    #[cxx::bridge]
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
            boost_force: f32,
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
            enable_physics_rounding: bool,
        }
    }

    pub use mutators::MutatorConfig;
}

pub mod math {
    #[repr(C, align(16))]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub _w: f32,
    }

    unsafe impl cxx::ExternType for Vec3 {
        #[allow(unused_attributes)]
        #[doc(hidden)]
        type Id = (cxx::V, cxx::e, cxx::c);
        type Kind = cxx::kind::Trivial;
    }

    #[repr(C, align(16))]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct RotMat {
        pub forward: Vec3,
        pub right: Vec3,
        pub up: Vec3,
    }

    unsafe impl cxx::ExternType for RotMat {
        #[allow(unused_attributes)]
        #[doc(hidden)]
        type Id = (cxx::R, cxx::o, cxx::t, cxx::M, cxx::a, cxx::t);
        type Kind = cxx::kind::Trivial;
    }

    #[cxx::bridge]
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
