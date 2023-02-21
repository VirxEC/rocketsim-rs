use autocxx::prelude::*;
use std::{error::Error, pin::Pin};

pub use autocxx;
pub use cxx;

#[cxx::bridge]
mod extra {
    unsafe extern "C++" {
        include!("extra.h");

        type btVector3 = crate::Vec3;
        type CarConfig = crate::sim::car::CarConfig;
        type ECarState = crate::sim::car::CarState;
        type Arena = crate::sim::arena::Arena;
        type Team = crate::sim::car::Team;
        type EBallState = crate::sim::ball::BallState;
        type CarControls = crate::sim::CarControls;
        type EBoostPadState = crate::sim::boostpad::BoostPadState;

        fn btVector3ToArray(vec: &btVector3) -> [f32; 3];
        fn arrayToBtVector3(arr: &[f32; 3]) -> UniquePtr<btVector3>;
        fn cloneBtVector3(vec: &btVector3) -> UniquePtr<btVector3>;

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

        #[rust_name = "get_car_state_from_id"]
        fn getCarState(arena: Pin<&mut Arena>, car_id: u32) -> UniquePtr<ECarState>;

        #[rust_name = "set_car_state"]
        fn setCarState(arena: Pin<&mut Arena>, car_id: u32, state: &ECarState) -> bool;

        #[rust_name = "add_car"]
        fn addCar(arena: Pin<&mut Arena>, team: Team, config: &CarConfig) -> u32;

        #[rust_name = "set_car_controls"]
        fn setCarControls(arena: Pin<&mut Arena>, car_id: u32, controls: &CarControls) -> bool;

        #[rust_name = "get_ball_state"]
        fn getBallState(arena: &Arena) -> UniquePtr<EBallState>;

        #[rust_name = "set_ball_state"]
        fn setBallState(arena: Pin<&mut Arena>, state: &EBallState);

        #[rust_name = "num_boost_pads"]
        fn numBoostPads(arena: &Arena) -> u32;

        #[rust_name = "get_boost_pad_is_big"]
        fn getBoostPadIsBig(arena: &Arena, index: u32) -> bool;

        #[rust_name = "get_boost_pad_pos"]
        fn getBoostPadPos(arena: &Arena, index: u32) -> UniquePtr<btVector3>;

        #[rust_name = "get_boost_pad_state"]
        fn getBoostPadState(arena: &Arena, index: u32) -> EBoostPadState;

        #[rust_name = "set_boost_pad_state"]
        fn setBoostPadState(arena: Pin<&mut Arena>, state: &EBoostPadState);
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

impl sim::arena::Arena {
    /// Returns the ID of the car that was added.
    #[inline]
    #[must_use]
    pub fn add_car(self: Pin<&mut Self>, team: sim::car::Team, config: &sim::car::CarConfig) -> u32 {
        extra::add_car(self, team, config)
    }

    pub fn get_car_state_from_id(self: Pin<&mut Self>, car_id: u32) -> Result<cxx::UniquePtr<sim::car::CarState>, NoCarFound> {
        let car = extra::get_car_state_from_id(self, car_id);
        if car.is_null() {
            Err(NoCarFound(car_id))
        } else {
            Ok(car)
        }
    }

    #[inline]
    pub fn set_car_state(self: Pin<&mut Self>, car_id: u32, state: &sim::car::CarState) -> Result<(), NoCarFound> {
        if extra::set_car_state(self, car_id, state) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    #[must_use]
    pub fn get_ball_state(&self) -> cxx::UniquePtr<sim::ball::BallState> {
        extra::get_ball_state(self)
    }

    #[inline]
    pub fn set_ball_state(self: Pin<&mut Self>, state: &sim::ball::BallState) {
        extra::set_ball_state(self, state);
    }

    #[inline]
    pub fn set_car_controls(self: Pin<&mut Self>, car_id: u32, controls: &sim::CarControls) -> Result<(), NoCarFound> {
        if extra::set_car_controls(self, car_id, controls) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    pub fn num_boost_pads(&self) -> u32 {
        extra::num_boost_pads(self)
    }

    #[must_use]
    pub fn get_pad_is_big(&self, index: u32) -> bool {
        assert!(index < self.num_boost_pads());
        extra::get_boost_pad_is_big(self, index)
    }

    #[must_use]
    pub fn get_pad_pos(&self, index: u32) -> UniquePtr<btVector3> {
        assert!(index < self.num_boost_pads());
        extra::get_boost_pad_pos(self, index)
    }

    #[inline]
    #[must_use]
    pub fn get_pad_state(&self, index: u32) -> sim::boostpad::BoostPadState {
        assert!(index < self.num_boost_pads());
        extra::get_boost_pad_state(self, index)
    }

    #[inline]
    pub fn set_pad_state(self: Pin<&mut Self>, state: &sim::boostpad::BoostPadState) {
        extra::set_boost_pad_state(self, state);
    }

    #[inline]
    pub fn step(self: Pin<&mut Self>, ticks: i32) {
        self.Step(c_int(ticks));
    }
}

autocxx::include_cpp! {
    #include "BulletLink.h"
    name!(bulletlink)
    safety!(unsafe)
    generate_pod!("Angle")
    generate!("Vec")
    generate!("btVector3")
}

pub use bulletlink::{btVector3, Angle, Vec as Vec3};

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec3").field("x", &self.x()).field("y", &self.y()).field("z", &self.z()).finish()
    }
}

impl btVector3 {
    #[inline]
    pub fn zero() -> UniquePtr<Vec3> {
        Self::new1(&0., &0., &0.).within_unique_ptr()
    }

    #[inline]
    pub fn to_array(&self) -> [f32; 3] {
        extra::btVector3ToArray(self)
    }

    #[inline]
    pub fn from_array(arr: &[f32; 3]) -> cxx::UniquePtr<Self> {
        extra::arrayToBtVector3(arr)
    }

    #[inline]
    pub fn clone(&self) -> cxx::UniquePtr<Self> {
        extra::cloneBtVector3(self)
    }
}

#[cfg(feature = "glam")]
impl btVector3 {
    #[inline]
    pub fn to_glam(&self) -> glam::Vec3 {
        glam::Vec3::from_array(self.to_array())
    }

    #[inline]
    pub fn to_glama(&self) -> glam::Vec3A {
        glam::Vec3A::from_array(self.to_array())
    }
}

impl Clone for Angle {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            yaw: self.yaw,
            pitch: self.pitch,
            roll: self.roll,
        }
    }
}

impl Default for Angle {
    #[inline]
    fn default() -> Self {
        Self { pitch: 0., yaw: 0., roll: 0. }
    }
}

impl std::fmt::Debug for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Angle")
            .field("pitch", &self.pitch)
            .field("yaw", &self.yaw)
            .field("roll", &self.roll)
            .finish()
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

        impl UniquePtr<CarControls> {}
    }

    use autocxx::WithinUniquePtr;
    pub use carcontrols::CarControls;

    pub mod arena {
        autocxx::include_cpp! {
            #include "Sim/Arena/Arena.h"
            name!(arena)
            safety!(unsafe)
            extern_cpp_type!("Ball", crate::sim::ball::Ball)
            extern_cpp_type!("btVector3", crate::Vec3)
            extern_cpp_type!("MeshLoader::Mesh", crate::sim::meshloader::MeshLoader::Mesh)
            block!("btManifoldPoint")
            block!("Car")
            generate_pod!("GameMode")
            generate!("Arena")
        }

        pub use arena::{Arena, GameMode};
    }

    impl arena::Arena {
        #[inline]
        pub fn default_soccar() -> cxx::UniquePtr<Self> {
            Self::new(arena::GameMode::SOCCAR, 120.).within_unique_ptr()
        }

        #[inline]
        pub fn get_tick_rate(self: crate::Pin<&mut Self>) -> f32 {
            self.GetTickRate()
        }
    }

    pub mod ball {
        autocxx::include_cpp! {
            #include "Sim/Ball/Ball.h"
            name!(ball)
            safety!(unsafe)
            extern_cpp_type!("btVector3", crate::Vec3)
            block!("btManifoldPoint")
            block!("BallState")
            generate!("Ball")
        }

        #[cxx::bridge]
        mod inner_bs {
            unsafe extern "C++" {
                include!("extra.h");

                type btVector3 = crate::Vec3;
                type EBallState;
            }

            struct EBallState {
                pos: UniquePtr<btVector3>,
                vel: UniquePtr<btVector3>,
                angvel: UniquePtr<btVector3>,
            }

            impl UniquePtr<EBallState> {}
        }

        pub use ball::Ball;
        pub use inner_bs::EBallState as BallState;

        impl std::fmt::Debug for BallState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("BallState")
                    .field("pos", &self.pos)
                    .field("vel", &self.vel)
                    .field("angvel", &self.angvel)
                    .finish()
            }
        }
    }

    pub mod car {
        autocxx::include_cpp! {
            #include "Sim/Car/Car.h"
            name!(car)
            safety!(unsafe)
            block!("CarState")
            block!("btDynamicsWorld")
            generate_pod!("Team")
            generate!("Car")
        }

        #[cxx::bridge]
        mod inner_cs {
            unsafe extern "C++" {
                include!("extra.h");

                type Angle = crate::Angle;
                type CarControls = crate::sim::CarControls;
                type btVector3 = crate::Vec3;

                type ECarState;
            }

            struct ECarState {
                pos: UniquePtr<btVector3>,
                angles: Angle,
                vel: UniquePtr<btVector3>,
                angvel: UniquePtr<btVector3>,
                isOnGround: bool,
                hasJumped: bool,
                hasDoubleJumped: bool,
                hasFlipped: bool,
                lastRelDodgeTorque: UniquePtr<btVector3>,
                jumpTimer: f32,
                flipTimer: f32,
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
                contactNormal: UniquePtr<btVector3>,
                lastControls: CarControls,
            }

            impl UniquePtr<ECarState> {}
        }

        pub use car::{Car, Team};
        pub use inner_cs::ECarState as CarState;

        impl std::fmt::Debug for CarState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("CarState")
                    .field("pos", &self.pos)
                    .field("vel", &self.vel)
                    .field("angles", &self.angles)
                    .field("angvel", &self.angvel)
                    .field("isOnGround", &self.isOnGround)
                    .field("hasJumped", &self.hasJumped)
                    .field("hasDoubleJumped", &self.hasDoubleJumped)
                    .field("hasFlipped", &self.hasFlipped)
                    .field("lastRelDodgeTorque", &self.lastRelDodgeTorque)
                    .field("jumpTimer", &self.jumpTimer)
                    .field("flipTimer", &self.flipTimer)
                    .field("isJumping", &self.isJumping)
                    .field("airTimeSinceJump", &self.airTimeSinceJump)
                    .field("boost", &self.boost)
                    .field("timeSpentBoosting", &self.timeSpentBoosting)
                    .field("isSupersonic", &self.isSupersonic)
                    .field("supersonicTime", &self.supersonicTime)
                    .field("handbrakeVal", &self.handbrakeVal)
                    .field("isAutoFlipping", &self.isAutoFlipping)
                    .field("autoFlipTimer", &self.autoFlipTimer)
                    .field("autoFlipTorqueScale", &self.autoFlipTorqueScale)
                    .field("hasContact", &self.hasContact)
                    .field("contactNormal", &self.contactNormal)
                    .field("lastControls", &self.lastControls)
                    .finish()
            }
        }

        #[cxx::bridge]
        mod carconfig {
            unsafe extern "C++" {
                include!("Sim/Car/CarConfig/CarConfig.h");

                type btVector3 = crate::Vec3;

                type WheelPairConfig;
                type CarConfig;
            }

            struct WheelPairConfig {
                wheelRadius: f32,
                suspensionRestLength: f32,
                connectionPointOffset: UniquePtr<btVector3>,
            }

            impl UniquePtr<WheelPairConfig> {}

            struct CarConfig {
                hitboxSize: UniquePtr<btVector3>,
                hitboxPosOffset: UniquePtr<btVector3>,
                frontWheels: WheelPairConfig,
                backWheels: WheelPairConfig,
                dodgeDeadzone: f32,
            }

            impl UniquePtr<CarConfig> {}
        }

        pub use carconfig::{CarConfig, WheelPairConfig};

        impl std::fmt::Debug for WheelPairConfig {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("WheelPairConfig")
                    .field("wheelRadius", &self.wheelRadius)
                    .field("suspensionRestLength", &self.suspensionRestLength)
                    .field("connectionPointOffset", &self.connectionPointOffset)
                    .finish()
            }
        }

        impl std::fmt::Debug for CarConfig {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("CarConfig")
                    .field("hitboxSize", &self.hitboxSize)
                    .field("hitboxPosOffset", &self.hitboxPosOffset)
                    .field("frontWheels", &self.frontWheels)
                    .field("backWheels", &self.backWheels)
                    .field("dodgeDeadzone", &self.dodgeDeadzone)
                    .finish()
            }
        }
    }

    pub mod meshloader {
        autocxx::include_cpp! {
            #include "Sim/MeshLoader/MeshLoader.h"
            name!(meshloader)
            safety!(unsafe)
            extern_cpp_type!("btVector3", crate::Vec3)
            generate!("MeshLoader::Mesh")
            generate!("MeshLoader::TriIndices")
        }

        pub use meshloader::MeshLoader;
    }

    pub mod boostpad {
        autocxx::include_cpp! {
            #include "Sim/BoostPad/BoostPad.h"
            name!(boostpad)
            safety!(unsafe)
            extern_cpp_type!("btVector3", crate::Vec3)
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
                id: u32,
                isActive: bool,
                cooldown: f32,
            }
        }

        pub use inner_bps::EBoostPadState as BoostPadState;
    }
}
