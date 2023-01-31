pub use autocxx;
pub use cxx;

autocxx::include_cpp! {
    #include "BulletLink.h"
    name!(bulletlink)
    safety!(unsafe)
    generate_pod!("Angle")
    generate!("Vec")
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
            extern_cpp_type!("CarConfig", crate::sim::car::carconfig::CarConfig)
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
            generate!("Ball")
            generate!("BallState")
        }

        pub use ball::{Ball, BallState};
    }

    pub mod car {
        autocxx::include_cpp! {
            #include "Sim/Car/Car.h"
            name!(car)
            safety!(unsafe)
            extern_cpp_type!("CarControls", crate::sim::CarControls)
            generate_pod!("Team")
            generate!("CarState")
            generate!("Car")
        }

        pub use car::{Car, CarState, Team};

        pub mod carconfig {
            autocxx::include_cpp! {
                #include "Sim/Car/CarConfig/CarConfig.h"
                name!(carconfig)
                safety!(unsafe)
                generate!("WheelPairConfig")
                generate!("CarConfig")
            }

            pub use carconfig::{CarConfig, WheelPairConfig};
        }
    }

    pub mod meshloader {
        autocxx::include_cpp! {
            #include "Sim/MeshLoader/MeshLoader.h"
            name!(meshloader)
            safety!(unsafe)
            extern_cpp_type!("btVector3", crate::Vec)
            generate!("MeshLoader::Mesh")
        }

        pub use meshloader::MeshLoader;
    }
}
