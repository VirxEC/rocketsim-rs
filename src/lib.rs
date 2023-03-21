use std::error::Error;

pub use autocxx;
pub use cxx;
pub use glam::Vec3A;

autocxx::include_cpp! {
    #include "extra.h"
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
        include!("extra.h");

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
                type BallState = crate::sim::ball::BallState;
                type EBoostPadState = crate::sim::boostpad::BoostPadState;
                type CarConfig = crate::sim::car::CarConfig;
                type CarControls = crate::sim::CarControls;
                type Vec = crate::sim::math::Vec3;
                type Team = crate::sim::car::Team;

                #[rust_name = "get_car_from_index"]
                fn GetCarFromIndex(self: Pin<&mut Arenar>, index: u32) -> CarState;
                #[rust_name = "rsc"]
                fn SetCar(self: Pin<&mut Arenar>, car_id: u32, car_state: &CarState) -> bool;
                #[rust_name = "get_car"]
                fn GetCar(self: Pin<&mut Arenar>, car_id: u32) -> CarState;
                #[rust_name = "add_car"]
                fn AddCar(self: Pin<&mut Arenar>, team: Team, car_config: &CarConfig) -> u32;
                #[rust_name = "scc"]
                fn SetCarControls(self: Pin<&mut Arenar>, car_id: u32, car_controls: &CarControls) -> bool;
                #[rust_name = "get_ball"]
                fn GetBall(self: &Arenar) -> BallState;
                #[rust_name = "set_ball"]
                fn SetBall(self: Pin<&mut Arenar>, ball_state: &BallState);
                #[rust_name = "get_pad_pos"]
                fn GetPadPos(self: &Arenar, index: u32) -> Vec;
                #[rust_name = "set_pad_state"]
                fn SetPadState(self: Pin<&mut Arenar>, state: &EBoostPadState);
                #[rust_name = "get_pad_state"]
                fn GetPadState(self: &Arenar, index: u32) -> EBoostPadState;
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
            pub fn set_car(self: Pin<&mut Self>, car_id: u32, car_state: &CarState) -> Result<(), NoCarFound> {
                if self.rsc(car_id, car_state) {
                    Ok(())
                } else {
                    Err(NoCarFound(car_id))
                }
            }

            #[inline]
            pub fn set_car_controls(self: Pin<&mut Self>, car_id: u32, car_controls: &CarControls) -> Result<(), NoCarFound> {
                if self.scc(car_id, car_controls) {
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
            pub fn iter_pad_pos(&self) -> impl Iterator<Item = Vec> + '_ {
                (0..self.num_boost_pads()).map(move |id| self.get_pad_pos(id))
            }

            #[inline]
            pub fn iter_pad_state(&self) -> impl Iterator<Item = EBoostPadState> + '_ {
                (0..self.num_boost_pads()).map(move |id| self.get_pad_state(id))
            }
        }
    }

    pub mod ball {
        autocxx::include_cpp! {
            #include "Sim/Ball/Ball.h"
            name!(ball)
            safety!(unsafe)
            block!("btManifoldPoint")
            block!("btDynamicsWorld")
            block!("BallState")
            generate!("Ball")
        }

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
                angVel: Vec3,
            }
        }

        pub use ball::Ball;
        pub use inner_bs::BallState;
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

            #[derive(Clone, Copy, Debug, Default)]
            struct CarState {
                pos: Vec3,
                rotMat: RotMat,
                vel: Vec3,
                angvel: Vec3,
                isOnGround: bool,
                hasJumped: bool,
                hasDoubleJumped: bool,
                hasFlipped: bool,
                lastRelDodgeTorque: Vec3,
                jumpTime: f32,
                flipTime: f32,
                isJumping: bool,
                airTimeSinceJump: f32,
                boost: f32,
                timeSpentBoosting: f32,
                isSupersonic: bool,
                supersonicTime: f32,
                handbrakeVal: f32,
                isAutoFlipping: bool,
                autoFlipTimer: f32,
                autoFlipTorqueScale: f32,
                hasContact: bool,
                contactNormal: Vec3,
                otherCarID: u32,
                cooldownTimer: f32,
                isDemoed: bool,
                demoRespawnTimer: f32,
                lastHitBallTick: u64,
                lastControls: CarControls,
            }
        }

        pub use car::Team;
        pub use inner_cs::CarState as Car;

        impl Car {
            #[inline]
            pub fn get_contacting_car(&self, arena: std::pin::Pin<&mut super::arena::Arena>) -> Option<Self> {
                if self.otherCarID != 0 {
                    Some(arena.get_car(self.otherCarID))
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

            #[derive(Debug)]
            struct WheelPairConfig {
                wheelRadius: f32,
                suspensionRestLength: f32,
                connectionPointOffset: Vec3,
            }

            #[derive(Debug)]
            struct CarConfig {
                hitboxSize: Vec3,
                hitboxPosOffset: Vec3,
                frontWheels: WheelPairConfig,
                backWheels: WheelPairConfig,
                dodgeDeadzone: f32,
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
                include!("extra.h");

                type EBoostPadState;
            }

            #[derive(Clone, Copy, Debug, Default)]
            struct EBoostPadState {
                index: u32,
                isActive: bool,
                cooldown: f32,
            }
        }

        pub use inner_bps::EBoostPadState as BoostPadState;
    }

    pub mod math {
        #[cfg(all(target_arch = "x86", feature = "glam"))]
        use core::arch::x86::*;
        #[cfg(all(target_arch = "x86_64", feature = "glam"))]
        use core::arch::x86_64::*;

        #[cfg(feature = "glam")]
        use glam::{EulerRot, Quat, Vec3A, Vec4};

        #[repr(C)]
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
            type Id = (::cxx::V, ::cxx::e, ::cxx::c);
            type Kind = ::cxx::kind::Trivial;
        }

        #[cxx::bridge]
        mod inner_math {
            unsafe extern "C++" {
                include!("extra.h");

                #[rust_name = "Vec3"]
                type Vec = super::Vec3;
                type RotMat;
                type Angle;
            }

            #[derive(Clone, Copy, Debug, Default)]
            struct RotMat {
                forward: Vec3,
                right: Vec3,
                up: Vec3,
            }

            #[derive(Clone, Copy, Debug, Default)]
            struct Angle {
                pitch: f32,
                yaw: f32,
                roll: f32,
            }
        }

        impl std::fmt::Display for RotMat {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "f: {}, r: {}, u: {}", self.forward, self.right, self.up)
            }
        }

        impl std::fmt::Display for Angle {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "(p: {}, y: {}, r: {})", self.pitch, self.yaw, self.roll)
            }
        }

        impl std::fmt::Display for Vec3 {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
            }
        }

        #[cfg(feature = "glam")]
        impl From<Angle> for Quat {
            #[inline]
            fn from(value: Angle) -> Self {
                Self::from_euler(EulerRot::XYZ, value.roll, roll.pitch, roll.yaw)
            }
        }

        #[cfg(feature = "glam")]
        impl From<Quat> for Angle {
            #[inline]
            fn from(value: Quat) -> Self {
                let (roll, pitch, yaw) = value.to_euler(EulerRot::XYZ);
                Self { pitch, yaw, roll }
            }
        }

        #[cfg(feature = "glam")]
        impl From<Vec3> for Vec3A {
            #[inline]
            fn from(value: Vec3) -> Self {
                Vec3A::from(__m128::from(value.to_glam()))
            }
        }

        #[cfg(feature = "glam")]
        impl From<Vec3A> for Vec3 {
            #[inline]
            fn from(value: Vec3A) -> Self {
                Self::from_glam(Vec4::from(__m128::from(value)))
            }
        }

        impl Vec3 {
            #[inline]
            pub const fn new(x: f32, y: f32, z: f32) -> Self {
                Self { x, y, z, _w: 0. }
            }
        }

        #[cfg(feature = "glam")]
        impl Vec3 {
            #[inline]
            pub const fn to_glam(self) -> Vec4 {
                Vec4::new(self.x, self.y, self.z, self._w)
            }

            #[inline]
            pub const fn from_glam(vec: Vec4) -> Self {
                let [x, y, z, w] = vec.to_array();
                Self { x, y, z, _w: w }
            }
        }

        pub use inner_math::{Angle, RotMat};
    }
}
