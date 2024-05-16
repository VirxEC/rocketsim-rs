#![doc = include_str!("../README.md")]
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

// dummy gen so that the build script doesn't fail
autocxx::include_cpp! {
    #include "arenar.h"
    name!(base)
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Stages {
    Uninitialized,
    Initializing,
    Initialized,
}

unsafe impl cxx::ExternType for Stages {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::RocketSimStage");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
mod extra {
    unsafe extern "C++" {
        include!("arenar.h");

        #[namespace = "RocketSim"]
        type RocketSimStage = crate::Stages;
        #[namespace = "RocketSim"]
        type CarConfig = crate::sim::CarConfig;
        #[namespace = "RocketSim"]
        type RotMat = crate::math::RotMat;
        #[namespace = "RocketSim"]
        type Angle = crate::math::Angle;
        #[namespace = "RocketSim"]
        type GameMode = crate::sim::GameMode;
        #[namespace = "RocketSim"]
        type ArenaConfig = crate::sim::ArenaConfig;
        #[rust_name = "Arena"]
        type Arenar = crate::sim::Arena;

        #[namespace = "RocketSim"]
        #[cxx_name = "GetStage"]
        pub fn get_stage() -> RocketSimStage;

        fn Init(folder: &str);

        #[cxx_name = "InitFromMem"]
        /// Initializes the collision mesh system for `RocketSim` from memory
        fn init_from_mem(soccar: &[&[u8]], hoops: &[&[u8]]);

        #[must_use]
        #[doc(hidden)]
        fn AngleFromRotMat(mat: RotMat) -> Angle;

        #[must_use]
        #[doc(hidden)]
        fn CreateArena(game_mode: GameMode, arena_config: ArenaConfig, tick_rate: u8) -> UniquePtr<Arena>;

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

pub use extra::{get_stage, init_from_mem};

#[inline]
/// Initializes the collision mesh system for `RocketSim`
pub fn init(collision_meshes_folder: Option<&str>) {
    extra::Init(collision_meshes_folder.unwrap_or("collision_meshes"));
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

    #[cxx::bridge(namespace = "RocketSim")]
    mod arenaconfig {
        unsafe extern "C++" {
            include!("Sim/Arena/ArenaConfig/ArenaConfig.h");

            type ArenaConfig;
            #[rust_name = "Vec3"]
            type Vec = crate::math::Vec3;
            type ArenaMemWeightMode = crate::sim::ArenaMemWeightMode;
        }

        #[derive(Clone, Copy, Debug)]
        struct ArenaConfig {
            mem_weight_mode: ArenaMemWeightMode,
            min_pos: Vec3,
            max_pos: Vec3,
            max_aabb_len: f32,
            no_ball_rot: bool,
            use_custom_broadphase: bool,
            max_objects: u32,
        }
    }

    pub use arenaconfig::ArenaConfig;

    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
    pub enum GameMode {
        #[default]
        Soccar,
        Hoops,
        Heatseeker,
        Snowday,
        TheVoid,
    }

    unsafe impl cxx::ExternType for GameMode {
        #[allow(unused_attributes)]
        #[doc(hidden)]
        type Id = cxx::type_id!("RocketSim::GameMode");
        type Kind = cxx::kind::Trivial;
    }

    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
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
    mod arena_cxx {
        unsafe extern "C++" {
            include!("arenar.h");

            #[rust_name = "Arena"]
            type Arenar;

            #[namespace = "RocketSim"]
            type CarState = crate::sim::CarState;
            #[namespace = "RocketSim"]
            type BallState = crate::sim::BallState;
            #[cxx_name = "EBoostPadState"]
            type BoostPadState = crate::sim::BoostPadState;
            #[namespace = "RocketSim"]
            type CarConfig = crate::sim::CarConfig;
            #[namespace = "RocketSim"]
            type CarControls = crate::sim::CarControls;
            #[namespace = "RocketSim"]
            #[rust_name = "Vec3"]
            type Vec = crate::math::Vec3;
            #[namespace = "RocketSim"]
            type Team = crate::sim::Team;
            #[namespace = "RocketSim"]
            type MutatorConfig = crate::sim::MutatorConfig;
            #[namespace = "RocketSim"]
            type GameMode = crate::sim::GameMode;
            #[allow(dead_code)]
            #[namespace = "RocketSim"]
            type ArenaMemWeightMode = crate::sim::ArenaMemWeightMode;

            #[must_use]
            #[doc(hidden)]
            #[rust_name = "rsc"]
            fn SetCar(self: Pin<&mut Arena>, car_id: u32, car: CarState) -> bool;

            #[must_use]
            #[doc(hidden)]
            #[rust_name = "rscc"]
            fn SetCarControls(self: Pin<&mut Arena>, car_id: u32, car_controls: CarControls) -> bool;

            #[doc(hidden)]
            #[rust_name = "rmvc"]
            fn RemoveCar(self: Pin<&mut Arena>, car_id: u32) -> bool;

            #[doc(hidden)]
            #[rust_name = "rtrk"]
            fn ResetToRandomKickoff(self: Pin<&mut Arena>, seed: i32);

            #[doc(hidden)]
            #[rust_name = "dc"]
            fn DemolishCar(self: Pin<&mut Arena>, car_id: u32) -> bool;

            #[doc(hidden)]
            #[rust_name = "rspc"]
            fn RespawnCar(self: Pin<&mut Arena>, car_id: u32, seed: i32, boost_amount: f32) -> bool;

            #[doc(hidden)]
            #[rust_name = "ibpgi"]
            fn IsBallProbablyGoingIn(self: &Arena, max_time: f32, extra_margin: f32) -> bool;

            /// Returns all of the car ids"
            #[must_use]
            #[cxx_name = "GetCars"]
            fn get_cars(self: &Arena) -> Vec<u32>;

            /// Returns the car state of the car with the given id
            #[must_use]
            #[cxx_name = "GetCar"]
            fn get_car(self: Pin<&mut Arena>, car_id: u32) -> CarState;

            /// Adds a car to the arena with the given team and car config
            #[must_use]
            #[cxx_name = "AddCar"]
            fn add_car(self: Pin<&mut Arena>, team: Team, car_config: &CarConfig) -> u32;

            /// Returns the ball state
            #[must_use]
            #[cxx_name = "GetBall"]
            fn get_ball(self: Pin<&mut Arena>) -> BallState;
            /// Sets the ball state

            #[cxx_name = "SetBall"]
            fn set_ball(self: Pin<&mut Arena>, ball: BallState);

            /// Returns the position of the pad with the given index
            #[must_use]
            #[cxx_name = "GetPadPos"]
            fn get_pad_pos(self: &Arena, index: usize) -> Vec3;
            /// Sets the state of the pad with the given index

            #[cxx_name = "SetPadState"]
            fn set_pad_state(self: Pin<&mut Arena>, index: usize, pad: BoostPadState);

            /// Returns the state of the pad with the given index
            #[must_use]
            #[cxx_name = "GetPadState"]
            fn get_pad_state(self: &Arena, index: usize) -> BoostPadState;

            /// Returns the car config of the car with the given id
            #[must_use]
            #[cxx_name = "GetCarConfig"]
            fn get_car_config(self: &Arena, id: u32) -> CarConfig;

            /// Returns the team of the car with the given id
            #[must_use]
            #[cxx_name = "GetCarTeam"]
            fn get_car_team(self: &Arena, id: u32) -> Team;
            /// Sets the goal scored callback

            #[cxx_name = "SetGoalScoreCallback"]
            fn set_goal_scored_callback(
                self: Pin<&mut Arena>,
                callback: fn(arena: Pin<&mut Arena>, car_team: Team, user_data: usize),
                user_data: usize,
            );
            /// Sets the car bump callback

            #[cxx_name = "SetCarBumpCallback"]
            fn set_car_bump_callback(
                self: Pin<&mut Arena>,
                callback: fn(arena: Pin<&mut Arena>, bumper: u32, victim: u32, is_demo: bool, user_data: usize),
                user_data: usize,
            );

            /// Returns the mutator config
            #[must_use]
            #[cxx_name = "GetMutatorConfig"]
            fn get_mutator_config(self: &Arena) -> MutatorConfig;

            /// Sets the mutator config
            #[cxx_name = "SetMutatorConfig"]
            fn set_mutator_config(self: Pin<&mut Arena>, config: MutatorConfig);

            /// Deep clone the arena, optionally copying the callbacks
            ///
            /// If `copy_callbacks` is true, the callbacks will be copied,
            /// otherwise the new arena will have no callbacks
            #[must_use]
            #[cxx_name = "Clone"]
            fn clone(self: &Arena, copy_callbacks: bool) -> UniquePtr<Arena>;

            /// Returns the number of cars in the arena
            #[cxx_name = "NumCars"]
            fn num_cars(self: &Arena) -> usize;

            /// Returns the radius of the ball
            #[cxx_name = "GetBallRadius"]
            fn get_ball_radius(self: &Arena) -> f32;

            /// Returns the number of pads in the arena
            #[cxx_name = "NumPads"]
            fn num_pads(self: &Arena) -> usize;

            /// Returns if the pad with the given index is big (gives 100 boost instead of 12)
            #[cxx_name = "GetPadIsBig"]
            fn get_pad_is_big(self: &Arena, index: usize) -> bool;

            /// Resets the tick count
            #[cxx_name = "ResetTickCount"]
            fn reset_tick_count(self: Pin<&mut Arena>);

            /// Returns the tick count
            #[cxx_name = "GetTickCount"]
            fn get_tick_count(self: &Arena) -> u64;

            /// Returns the tick rate (i.e. `0.008333` aka `1 / 120`)
            #[cxx_name = "GetTickRate"]
            fn get_tick_rate(self: &Arena) -> f32;

            /// Returns the game mode
            #[cxx_name = "GetGameMode"]
            fn get_game_mode(self: &Arena) -> GameMode;

            /// Steps the simulation by the given number of ticks
            #[cxx_name = "Step"]
            fn step(self: Pin<&mut Arena>, num_ticks: u32);

            /// Returns if the ball is within a goal
            #[cxx_name = "IsBallScored"]
            fn is_ball_scored(self: &Arena) -> bool;
        }

        impl UniquePtr<Arena> {}
    }

    pub use arena_cxx::Arena;

    unsafe impl Send for Arena {}

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

    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
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
    mod carstate {
        unsafe extern "C++" {
            include!("Sim/Car/Car.h");

            #[rust_name = "Vec3"]
            type Vec = crate::math::Vec3;
            type RotMat = crate::math::RotMat;
            type CarControls = crate::sim::CarControls;
            type BallHitInfo = crate::sim::BallHitInfo;

            type CarState;

            #[cxx_name = "HasFlipOrJump"]
            /// Returns if the car has flipped or jumped
            fn has_flip_or_jump(self: &CarState) -> bool;

            #[cxx_name = "HasFlipReset"]
            /// Returns if the car has a flip reset
            fn has_flip_reset(self: &CarState) -> bool;

            #[cxx_name = "GotFlipReset"]
            /// Returns if the car got a flip reset
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

    pub use carstate::{CarContact, CarState, WorldContact};

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

    pub use boostpadstate::EBoostPadState as BoostPadState;

    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
    pub enum DemoMode {
        #[default]
        Normal = 0,
        OnContact = 1,
        Disabled = 2,
    }

    unsafe impl cxx::ExternType for DemoMode {
        #[allow(unused_attributes)]
        #[doc(hidden)]
        type Id = cxx::type_id!("RocketSim::DemoMode");
        type Kind = cxx::kind::Trivial;
    }

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
            #[cxx_name = "ToRotMat"]
            /// Converts the angle to a RocketSim rotation matrix
            fn to_rotmat(self: &Angle) -> RotMat;
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
