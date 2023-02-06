pub use autocxx;
pub use cxx;

use autocxx::prelude::*;

#[cxx::bridge]
mod extra {
    unsafe extern "C++" {
        include!("extra.h");

        type btVector3 = crate::Vec3;
        type CarConfig = crate::sim::car::CarConfig;
        type Car = crate::sim::car::Car;

        fn btVector3ToArray(vec: &btVector3) -> [f32; 3];
        fn arrayToBtVector3(arr: &[f32; 3]) -> UniquePtr<btVector3>;

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

        #[rust_name = "get_car_id"]
        fn getCarID(car: &Car) -> u32;
    }
}

impl sim::car::CarConfig {
    pub fn octane() -> &'static Self {
        extra::get_octane()
    }

    pub fn dominus() -> &'static Self {
        extra::get_dominus()
    }

    pub fn plank() -> &'static Self {
        extra::get_plank()
    }

    pub fn breakout() -> &'static Self {
        extra::get_breakout()
    }

    pub fn hybrid() -> &'static Self {
        extra::get_hybrid()
    }

    pub fn merc() -> &'static Self {
        extra::get_merc()
    }
}

impl sim::car::Car {
    pub fn id(&self) -> u32 {
        extra::get_car_id(self)
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

pub use bulletlink::{Angle, Vec as Vec3};

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec3")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .finish()
    }
}

impl Vec3 {
    pub fn default() -> UniquePtr<Vec3> {
        Vec3::new1(&0., &0., &0.).within_unique_ptr()
    }

    pub fn to_array(&self) -> [f32; 3] {
        extra::btVector3ToArray(self)
    }

    pub fn from_array(arr: [f32; 3]) -> cxx::UniquePtr<Self> {
        extra::arrayToBtVector3(&arr)
    }

    pub fn clone(&self) -> cxx::UniquePtr<Self> {
        Self::from_array(self.to_array())
    }
}

impl Clone for Angle {
    fn clone(&self) -> Self {
        Self {
            yaw: self.yaw,
            pitch: self.pitch,
            roll: self.roll,
        }
    }
}

impl Default for Angle {
    fn default() -> Self {
        Self {
            pitch: 0.,
            yaw: 0.,
            roll: 0.,
        }
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
    autocxx::include_cpp! {
        #include "Sim/CarControls.h"
        name!(carcontrols)
        safety!(unsafe)
        generate_pod!("CarControls")
    }

    pub use carcontrols::CarControls;

    impl Clone for CarControls {
        fn clone(&self) -> Self {
            Self {
                throttle: self.throttle,
                steer: self.steer,
                pitch: self.pitch,
                yaw: self.yaw,
                roll: self.roll,
                boost: self.boost,
                jump: self.jump,
                handbrake: self.handbrake,
            }
        }
    }

    impl Default for CarControls {
        fn default() -> Self {
            Self {
                throttle: 0.,
                steer: 0.,
                pitch: 0.,
                yaw: 0.,
                roll: 0.,
                jump: false,
                boost: false,
                handbrake: false,
            }
        }
    }

    impl std::fmt::Debug for CarControls {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("CarControls")
                .field("throttle", &self.throttle)
                .field("steer", &self.steer)
                .field("pitch", &self.pitch)
                .field("yaw", &self.yaw)
                .field("roll", &self.roll)
                .field("jump", &self.jump)
                .field("boost", &self.boost)
                .field("handbrake", &self.handbrake)
                .finish()
        }
    }

    pub mod arena {
        autocxx::include_cpp! {
            #include "Sim/Arena/Arena.h"
            name!(arena)
            safety!(unsafe)
            extern_cpp_type!("Team", crate::sim::car::Team)
            extern_cpp_type!("Car", crate::sim::car::Car)
            extern_cpp_type!("CarConfig", crate::sim::car::CarConfig)
            extern_cpp_type!("Ball", crate::sim::ball::Ball)
            extern_cpp_type!("btVector3", crate::Vec3)
            extern_cpp_type!("MeshLoader::Mesh", crate::sim::meshloader::MeshLoader::Mesh)
            generate_pod!("GameMode")
            generate!("Arena")
        }

        pub use arena::{Arena, GameMode};
    }

    pub mod ball {
        autocxx::include_cpp! {
            #include "Sim/Ball/Ball.h"
            name!(ball)
            safety!(unsafe)
            extern_cpp_type!("BallState", super::inner_bs::BallState)
            generate!("Ball")
        }

        #[cxx::bridge]
        mod inner_bs {
            unsafe extern "C++" {
                include!("Sim/Ball/Ball.h");

                type btVector3 = crate::Vec3;

                type BallState;
            }

            struct BallState {
                pos: UniquePtr<btVector3>,
                vel: UniquePtr<btVector3>,
                angVel: UniquePtr<btVector3>,
            }

            impl UniquePtr<BallState> {}
        }

        pub use ball::{Ball, BallState};
        use inner_bs::*;

        impl Default for BallState {
            fn default() -> Self {
                Self {
                    pos: btVector3::default(),
                    vel: btVector3::default(),
                    angVel: btVector3::default(),
                }
            }
        }

        impl std::fmt::Debug for BallState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("BallState")
                    .field("pos", &self.pos)
                    .field("vel", &self.vel)
                    .field("angVel", &self.angVel)
                    .finish()
            }
        }
    }

    pub mod car {
        autocxx::include_cpp! {
            #include "Sim/Car/Car.h"
            name!(car)
            safety!(unsafe)
            extern_cpp_type!("CarControls", crate::sim::CarControls)
            extern_cpp_type!("CarState", super::inner_cs::CarState)
            generate_pod!("Team")
            generate!("Car")
        }

        #[cxx::bridge]
        mod inner_cs {
            unsafe extern "C++" {
                include!("Sim/Car/Car.h");

                type btVector3 = crate::Vec3;
                type Angle = crate::Angle;
                type CarControls = crate::sim::CarControls;
                type Car = super::car::Car;

                type CarState;

                #[cxx_name = "GetState"]
                fn get_state(self: Pin<&mut Car>) -> CarState;
            }

            struct CarState {
                pos: UniquePtr<btVector3>,
                angles: Angle,
                vel: UniquePtr<btVector3>,
                angVel: UniquePtr<btVector3>,
                isOnGround: bool,
                hasJumped: bool,
                hasDoubleJumped: bool,
                hasFlipped: bool,
                lastRelDodgeTorque: UniquePtr<btVector3>,
                jumpTimer: f32,
                flipTimer: f32,
                isJumping: bool,
                airTimeSpaceJump: f32,
                boost: f32,
                isSupersonic: bool,
                handbrakeVal: f32,
                lastControls: CarControls,
            }

            impl UniquePtr<CarState> {}
        }

        pub use car::{Car, CarState, Team};
        use inner_cs::*;

        impl Clone for CarState {
            fn clone(&self) -> Self {
                Self {
                    pos: self.pos.clone(),
                    vel: self.vel.clone(),
                    angles: self.angles.clone(),
                    angVel: self.angVel.clone(),
                    isOnGround: self.isOnGround,
                    hasJumped: self.hasJumped,
                    hasDoubleJumped: self.hasDoubleJumped,
                    hasFlipped: self.hasFlipped,
                    lastRelDodgeTorque: self.lastRelDodgeTorque.clone(),
                    jumpTimer: self.jumpTimer,
                    flipTimer: self.flipTimer,
                    isJumping: self.isJumping,
                    airTimeSpaceJump: self.airTimeSpaceJump,
                    boost: self.boost,
                    isSupersonic: self.isSupersonic,
                    handbrakeVal: self.handbrakeVal,
                    lastControls: self.lastControls.clone(),
                }
            }
        }

        impl Default for CarState {
            fn default() -> Self {
                Self {
                    pos: btVector3::default(),
                    vel: btVector3::default(),
                    angles: Angle::default(),
                    angVel: btVector3::default(),
                    isOnGround: false,
                    hasJumped: false,
                    hasDoubleJumped: false,
                    hasFlipped: false,
                    lastRelDodgeTorque: btVector3::default(),
                    jumpTimer: 0.,
                    flipTimer: 0.,
                    isJumping: false,
                    airTimeSpaceJump: 0.,
                    boost: 0.,
                    isSupersonic: false,
                    handbrakeVal: 0.,
                    lastControls: CarControls::default(),
                }
            }
        }

        impl std::fmt::Debug for CarState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("CarState")
                    .field("pos", &self.pos)
                    .field("vel", &self.vel)
                    .field("angles", &self.angles)
                    .field("angVel", &self.angVel)
                    .field("isOnGround", &self.isOnGround)
                    .field("hasJumped", &self.hasJumped)
                    .field("hasDoubleJumped", &self.hasDoubleJumped)
                    .field("hasFlipped", &self.hasFlipped)
                    .field("lastRelDodgeTorque", &self.lastRelDodgeTorque)
                    .field("jumpTimer", &self.jumpTimer)
                    .field("flipTimer", &self.flipTimer)
                    .field("isJumping", &self.isJumping)
                    .field("airTimeSpaceJump", &self.airTimeSpaceJump)
                    .field("boost", &self.boost)
                    .field("isSupersonic", &self.isSupersonic)
                    .field("handbrakeVal", &self.handbrakeVal)
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
}
