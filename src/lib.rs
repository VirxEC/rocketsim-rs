#[cfg(feature = "glam")]
pub mod glam_ext;
#[cfg(feature = "rlbot")]
pub mod rlbot;

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
    }
}

#[inline]
/// Initializes the collision mesh system for RocketSim
pub fn init(collision_meshes_folder: Option<&str>) {
    Init::init(collision_meshes_folder.unwrap_or("collision_meshes"));
}

pub use base::{RocketSim::GetStage as get_stage, RocketSimStage as Stages};

#[cxx::bridge]
mod extra {
    unsafe extern "C++" {
        include!("arenar.h");

        type CarConfig = crate::sim::car::CarConfig;

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

impl sim::car::CarConfig {
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

    pub mod arena {
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
            generate_pod!("GameMode")
            generate!("Arenar")
        }

        #[cxx::bridge]
        mod arena_extra {
            unsafe extern "C++" {
                include!("arenar.h");

                type Arenar = super::Arena;
                type CarState = crate::sim::car::Car;
                type BallState = crate::sim::ball::Ball;
                type EBoostPadState = crate::sim::boostpad::BoostPadState;
                type CarConfig = crate::sim::car::CarConfig;
                type CarControls = crate::sim::CarControls;
                type Vec = crate::math::Vec3;
                type Team = crate::sim::car::Team;

                #[doc(hidden)]
                #[rust_name = "rgc"]
                fn GetCars(self: Pin<&mut Arenar>) -> UniquePtr<CxxVector<CarState>>;
                #[doc(hidden)]
                #[rust_name = "rsc"]
                fn SetCar(self: Pin<&mut Arenar>, car_id: u32, car: CarState) -> bool;
                #[rust_name = "get_car"]
                fn GetCar(self: Pin<&mut Arenar>, car_id: u32) -> CarState;
                #[rust_name = "add_car"]
                fn AddCar(self: Pin<&mut Arenar>, team: Team, car_config: &CarConfig) -> u32;
                #[doc(hidden)]
                #[rust_name = "rscc"]
                fn SetCarControls(self: Pin<&mut Arenar>, car_id: u32, car_controls: CarControls) -> bool;
                #[rust_name = "get_ball"]
                fn GetBall(self: Pin<&mut Arenar>) -> BallState;
                #[rust_name = "set_ball"]
                fn SetBall(self: Pin<&mut Arenar>, ball: BallState);
                #[rust_name = "get_pad_pos"]
                fn GetPadPos(self: &Arenar, index: usize) -> Vec;
                #[rust_name = "set_pad_state"]
                fn SetPadState(self: Pin<&mut Arenar>, index: usize, pad: EBoostPadState);
                #[rust_name = "get_pad_state"]
                fn GetPadState(self: &Arenar, index: usize) -> EBoostPadState;
                #[rust_name = "get_car_config_from_index"]
                fn GetCarConfigFromIndex(self: &Arenar, index: usize) -> CarConfig;
                #[rust_name = "get_car_team_from_index"]
                fn GetCarTeamFromIndex(self: &Arenar, index: usize) -> Team;
                #[rust_name = "set_goal_scored_callback"]
                fn SetGoalScoreCallback(self: Pin<&mut Arenar>, callback: unsafe fn(Pin<&mut Arenar>, Team, usize), user_data: usize);
            }
        }

        pub use arena::{Arenar as Arena, GameMode};
    }

    pub mod ball {
        #[cxx::bridge]
        mod inner_bs {
            unsafe extern "C++" {
                include!("Sim/Ball/Ball.h");

                #[rust_name = "Vec3"]
                type Vec = crate::math::Vec3;
                type BallState;
                type BallHitInfo;
            }

            #[derive(Clone, Copy, Debug, Default)]
            struct BallHitInfo {
                car_id: u32,
                relative_pos_on_ball: Vec3,
                ball_pos: Vec3,
                extra_hit_vel: Vec3,
                tick_count_when_hit: u64,
            }

            #[derive(Clone, Copy, Debug)]
            struct BallState {
                pos: Vec3,
                vel: Vec3,
                ang_vel: Vec3,
                hit_info: BallHitInfo,
            }
        }

        pub use inner_bs::{BallHitInfo, BallState, BallState as Ball};
    }

    pub mod car {
        autocxx::include_cpp! {
            #include "Sim/Car/Car.h"
            name!(car)
            safety!(unsafe)
            generate_pod!("Team")
        }

        #[cxx::bridge]
        mod inner_cs {
            unsafe extern "C++" {
                include!("Sim/Car/Car.h");

                #[rust_name = "Vec3"]
                type Vec = crate::math::Vec3;
                type RotMat = crate::math::RotMat;
                type CarControls = crate::sim::CarControls;

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
                last_hit_ball_tick: u64,
                last_controls: CarControls,
            }

            impl UniquePtr<CarState> {}
            impl CxxVector<CarState> {}
        }

        pub use car::Team;
        pub use inner_cs::{CarState, CarState as Car};

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
    }

    pub mod boostpad {
        #[cxx::bridge]
        mod inner_bps {
            unsafe extern "C++" {
                include!("arenar.h");

                type EBoostPadState;
            }

            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            struct EBoostPadState {
                is_active: bool,
                cooldown: f32,
                cur_locked_car_id: u32,
                prev_locked_car_id: u32,
            }
        }

        pub use inner_bps::{EBoostPadState, EBoostPadState as BoostPadState};
    }
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
            include!("arenar.h");

            type Angle;
        }

        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        struct Angle {
            yaw: f32,
            pitch: f32,
            roll: f32,
        }
    }

    pub use inner_math::Angle;
}
