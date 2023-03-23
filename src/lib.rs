#![cfg_attr(all(not(any(target_arch = "x86", target_arch = "x86_64")), feature = "glam"), feature(portable_simd))]

mod ext;

use std::error::Error;

pub use autocxx;
pub use cxx;
pub use ext::*;

autocxx::include_cpp! {
    #include "arenar.h"
    name!(base)
    safety!(unsafe)
    generate_pod!("RocketSimStage")
    generate!("RocketSim::Init")
    generate!("RocketSim::GetStage")
}

pub use base::{
    RocketSim::{GetStage as get_stage, Init as init},
    RocketSimStage as Stages,
};

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

#[derive(Debug)]
pub struct NoCarFound(u32);

impl std::fmt::Display for NoCarFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No car found in the given arena at the given ID {}.", self.0)
    }
}

impl Error for NoCarFound {}

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
                type Vec = crate::sim::math::Vec3;
                type Team = crate::sim::car::Team;

                #[doc(hidden)]
                #[rust_name = "rgc"]
                fn GetCars(self: Pin<&mut Arenar>) -> UniquePtr<CxxVector<CarState>>;
                #[doc(hidden)]
                #[rust_name = "rsc"]
                fn SetCar(self: Pin<&mut Arenar>, car_id: u32, car_state: CarState) -> bool;
                #[rust_name = "get_car"]
                fn GetCar(self: Pin<&mut Arenar>, car_id: u32) -> CarState;
                #[rust_name = "add_car"]
                fn AddCar(self: Pin<&mut Arenar>, team: Team, car_config: &CarConfig) -> u32;
                #[doc(hidden)]
                #[rust_name = "rscc"]
                fn SetCarControls(self: Pin<&mut Arenar>, car_id: u32, car_controls: CarControls) -> bool;
                #[rust_name = "get_ball"]
                fn GetBall(self: &Arenar) -> BallState;
                #[rust_name = "set_ball"]
                fn SetBall(self: Pin<&mut Arenar>, ball_state: BallState);
                #[rust_name = "get_pad_pos"]
                fn GetPadPos(self: &Arenar, index: usize) -> Vec;
                #[rust_name = "set_pad_state"]
                fn SetPadState(self: Pin<&mut Arenar>, index: usize, state: EBoostPadState);
                #[rust_name = "get_pad_state"]
                fn GetPadState(self: &Arenar, index: usize) -> EBoostPadState;
            }
        }

        use crate::NoCarFound;
        use arena_extra::*;
        use autocxx::WithinUniquePtr;
        use std::pin::Pin;

        pub use arena::{Arenar as Arena, GameMode};

        impl Arena {
            #[inline]
            pub fn default_soccar() -> cxx::UniquePtr<Self> {
                Self::new(arena::GameMode::SOCCAR, 120.).within_unique_ptr()
            }

            #[inline]
            pub fn reset_to_random_kickoff(self: Pin<&mut Self>, seed: Option<i32>) {
                self.ResetToRandomKickoff(seed.unwrap_or(-1));
            }

            #[inline]
            pub fn remove_car(self: Pin<&mut Self>, car_id: u32) -> Result<(), NoCarFound> {
                if self.RemoveCar(car_id) {
                    Ok(())
                } else {
                    Err(NoCarFound(car_id))
                }
            }

            #[inline]
            pub fn set_car(self: Pin<&mut Self>, car_id: u32, car_state: CarState) -> Result<(), NoCarFound> {
                if self.rsc(car_id, car_state) {
                    Ok(())
                } else {
                    Err(NoCarFound(car_id))
                }
            }

            #[inline]
            pub fn set_car_controls(self: Pin<&mut Self>, car_id: u32, car_controls: CarControls) -> Result<(), NoCarFound> {
                if self.rscc(car_id, car_controls) {
                    Ok(())
                } else {
                    Err(NoCarFound(car_id))
                }
            }

            #[inline]
            pub fn demolish_car(self: Pin<&mut Self>, car_id: u32) -> Result<(), NoCarFound> {
                if self.DemolishCar(car_id) {
                    Ok(())
                } else {
                    Err(NoCarFound(car_id))
                }
            }

            #[inline]
            pub fn respawn_car(self: Pin<&mut Self>, car_id: u32, seed: Option<i32>) -> Result<(), NoCarFound> {
                if self.RespawnCar(car_id, seed.unwrap_or(-1)) {
                    Ok(())
                } else {
                    Err(NoCarFound(car_id))
                }
            }

            #[inline]
            /// Returns all of the `(id, car_state)`s in the arena
            pub fn get_cars(mut self: Pin<&mut Self>) -> std::vec::Vec<(u32, CarState)> {
                self.as_mut().rgc().iter().enumerate().map(|(i, &state)| (self.get_car_id(i), state)).collect()
            }

            #[inline]
            /// Iterates over the static `(position, is_big)` info of boost pads in the Arena
            pub fn iter_pad_static(&self) -> impl Iterator<Item = (bool, Vec)> + '_ {
                (0..self.num_pads()).map(|i| (self.get_pad_is_big(i), self.get_pad_pos(i)))
            }

            #[inline]
            /// Iterates over the dynamic `(isActive, cooldown)` info of the boost pads in the arena
            pub fn iter_pad_state(&self) -> impl Iterator<Item = EBoostPadState> + '_ {
                (0..self.num_pads()).map(|i| self.get_pad_state(i))
            }
        }
    }

    pub mod ball {
        #[cxx::bridge]
        mod inner_bs {
            unsafe extern "C++" {
                include!("Sim/Ball/Ball.h");

                #[rust_name = "Vec3"]
                type Vec = crate::sim::math::Vec3;
                type BallState;
            }

            #[derive(Clone, Copy, Debug, Default)]
            struct BallState {
                pos: Vec3,
                vel: Vec3,
                ang_vel: Vec3,
            }
        }

        pub use inner_bs::{BallState, BallState as Ball};
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
                type Vec = crate::sim::math::Vec3;
                type RotMat = crate::sim::math::RotMat;
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

        use inner_cs::*;

        impl Default for Car {
            #[inline]
            fn default() -> Self {
                Self {
                    pos: Vec3::new(0., 0., 17.),
                    rot_mat: RotMat::get_identity(),
                    vel: Vec3::default(),
                    ang_vel: Vec3::default(),
                    is_on_ground: true,
                    has_jumped: false,
                    has_double_jumped: false,
                    has_flipped: false,
                    last_rel_dodge_torque: Vec3::default(),
                    jump_time: 0.,
                    flip_time: 0.,
                    is_jumping: false,
                    air_time_since_jump: 0.,
                    boost: 100. / 3.,
                    time_spent_boosting: 0.,
                    is_supersonic: false,
                    supersonic_time: 0.,
                    handbrake_val: 0.,
                    is_auto_flipping: false,
                    auto_flip_timer: 0.,
                    auto_flip_torque_scale: 0.,
                    has_contact: false,
                    contact_normal: Vec3::default(),
                    other_car_id: 0,
                    cooldown_timer: 0.,
                    is_demoed: false,
                    demo_respawn_timer: 0.,
                    last_hit_ball_tick: 0,
                    last_controls: CarControls::default(),
                }
            }
        }

        pub use car::Team;
        pub use inner_cs::{CarState, CarState as Car};

        impl Car {
            #[inline]
            pub fn get_contacting_car(&self, arena: std::pin::Pin<&mut super::arena::Arena>) -> Option<Self> {
                if self.other_car_id != 0 {
                    Some(arena.get_car(self.other_car_id))
                } else {
                    None
                }
            }
        }

        #[cxx::bridge]
        mod carconfig {
            unsafe extern "C++" {
                include!("Sim/Car/CarConfig/CarConfig.h");

                #[rust_name = "Vec3"]
                type Vec = crate::sim::math::Vec3;

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
        autocxx::include_cpp! {
            #include "Sim/BoostPad/BoostPad.h"
            name!(boostpad)
            safety!(unsafe)
            extern_cpp_type!("Vec", crate::sim::math::Vec3)
            block!("BoostPadState")
            block!("btDynamicsWorld")
            generate!("BoostPad")
        }

        pub use boostpad::BoostPad;

        #[cxx::bridge]
        mod inner_bps {
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

        pub use inner_bps::{EBoostPadState, EBoostPadState as BoostPadState};
    }

    pub mod math {
        #[repr(C, align(16))]
        #[derive(Clone, Copy, Debug, Default)]
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
        #[derive(Clone, Copy, Debug, Default)]
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

            #[derive(Clone, Copy, Debug, Default)]
            struct Angle {
                yaw: f32,
                pitch: f32,
                roll: f32,
            }
        }

        pub use inner_math::Angle;
    }
}
