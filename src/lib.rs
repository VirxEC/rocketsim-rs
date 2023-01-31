pub use autocxx;
pub use cxx;

#[cxx::bridge]
mod extra {
    unsafe extern "C++" {
        include!("extra.h");

        type btVector3 = crate::Vec;
        type CarConfig = crate::sim::car::CarConfig;

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

autocxx::include_cpp! {
    #include "BulletLink.h"
    name!(bulletlink)
    safety!(unsafe)
    generate_pod!("Angle")
    generate!("Vec")
    generate!("btVector3")
}

impl Vec {
    pub fn to_array(&self) -> [f32; 3] {
        extra::btVector3ToArray(self)
    }

    pub fn from_array(arr: [f32; 3]) -> cxx::UniquePtr<Self> {
        extra::arrayToBtVector3(&arr)
    }
}

pub use bulletlink::{Angle, Vec};

pub mod sim {
    autocxx::include_cpp! {
        #include "Sim/CarControls.h"
        name!(carcontrols)
        safety!(unsafe)
        generate_pod!("CarControls")
    }

    pub use carcontrols::CarControls;

    pub mod arena {
        autocxx::include_cpp! {
            #include "Sim/Arena/Arena.h"
            name!(arena)
            safety!(unsafe)
            extern_cpp_type!("Team", crate::sim::car::Team)
            extern_cpp_type!("Car", crate::sim::car::Car)
            extern_cpp_type!("CarConfig", crate::sim::car::CarConfig)
            extern_cpp_type!("Ball", crate::sim::ball::Ball)
            extern_cpp_type!("btVector3", crate::Vec)
            extern_cpp_type!("Mesh", crate::sim::meshloader::MeshLoader::Mesh)
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

                type btVector3 = crate::Vec;

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

                type btVector3 = crate::Vec;
                type Angle = crate::Angle;
                type CarControls = crate::sim::CarControls;

                type CarState;
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

        #[cxx::bridge]
        mod carconfig {
            unsafe extern "C++" {
                include!("Sim/Car/CarConfig/CarConfig.h");

                type btVector3 = crate::Vec;

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
    }

    pub mod meshloader {
        autocxx::include_cpp! {
            #include "Sim/MeshLoader/MeshLoader.h"
            name!(meshloader)
            safety!(unsafe)
            extern_cpp_type!("btVector3", crate::Vec)
            generate!("MeshLoader::Mesh")
            generate!("MeshLoader::TriIndices")
        }

        pub use meshloader::MeshLoader;
    }
}
