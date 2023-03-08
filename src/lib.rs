use autocxx::prelude::*;
use std::{error::Error, mem, pin::Pin};

pub use autocxx;
pub use cxx;
pub use glam::Vec3A;

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

        #[rust_name = "num_cars"]
        fn numCars(arena: &Arena) -> u32;

        #[rust_name = "get_car_id"]
        fn getCarID(arena: &Arena, index: u32) -> u32;

        #[rust_name = "get_car_from_index"]
        fn getCarFromIndex(arena: Pin<&mut Arena>, index: u32) -> UniquePtr<ECarState>;

        #[rust_name = "get_car_state_from_id"]
        fn getCarState(arena: Pin<&mut Arena>, car_id: u32) -> UniquePtr<ECarState>;

        #[rust_name = "set_car_state"]
        fn setCarState(arena: Pin<&mut Arena>, car_id: u32, state: &ECarState) -> bool;

        #[rust_name = "add_car"]
        fn addCar(arena: Pin<&mut Arena>, team: Team, config: &CarConfig) -> u32;

        #[rust_name = "set_car_controls"]
        fn setCarControls(arena: Pin<&mut Arena>, car_id: u32, controls: &CarControls) -> bool;

        #[rust_name = "demolish_car"]
        fn demolishCar(arena: Pin<&mut Arena>, car_id: u32) -> bool;

        #[rust_name = "respawn_car"]
        fn respawnCar(arena: Pin<&mut Arena>, car_id: u32, seed: i32) -> bool;

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

        #[rust_name = "get_tick_count"]
        fn getTickCount(arena: &Arena) -> u64;

        #[rust_name = "get_tick_rate"]
        fn getTickRate(arena: &Arena) -> f32;
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
    pub fn num_cars(&self) -> u32 {
        extra::num_cars(self)
    }

    #[inline]
    pub fn get_car_id_from_index(&self, index: u32) -> u32 {
        debug_assert!(index < self.num_cars(), "Index out of bounds: {} >= {}", index, self.num_cars());
        extra::get_car_id(self, index)
    }

    pub fn get_car_from_index(self: Pin<&mut Self>, index: u32) -> cxx::UniquePtr<sim::car::CarState> {
        debug_assert!(index < self.num_cars(), "Index out of bounds: {} >= {}", index, self.num_cars());
        extra::get_car_from_index(self, index)
    }

    #[inline]
    pub fn demolish_car(self: Pin<&mut Self>, car_id: u32) -> Result<(), NoCarFound> {
        if extra::demolish_car(self, car_id) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    pub fn respawn_car(self: Pin<&mut Self>, car_id: u32, seed: Option<i32>) -> Result<(), NoCarFound> {
        if extra::respawn_car(self, car_id, seed.unwrap_or(-1)) {
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
        debug_assert!(index < self.num_boost_pads(), "Index out of bounds: {} >= {}", index, self.num_boost_pads());
        extra::get_boost_pad_is_big(self, index)
    }

    #[inline]
    pub fn iter_pad_is_big(&self) -> impl Iterator<Item = bool> + '_ {
        (0..self.num_boost_pads()).map(move |id| self.get_pad_is_big(id))
    }

    #[must_use]
    pub fn get_pad_pos(&self, index: u32) -> UniquePtr<btVector3> {
        debug_assert!(index < self.num_boost_pads(), "Index out of bounds: {} >= {}", index, self.num_boost_pads());
        extra::get_boost_pad_pos(self, index)
    }

    #[inline]
    pub fn iter_pad_pos(&self) -> impl Iterator<Item = UniquePtr<btVector3>> + '_ {
        (0..self.num_boost_pads()).map(move |id| self.get_pad_pos(id))
    }

    #[inline]
    #[must_use]
    pub fn get_pad_state(&self, index: u32) -> sim::boostpad::BoostPadState {
        debug_assert!(index < self.num_boost_pads(), "Index out of bounds: {} >= {}", index, self.num_boost_pads());
        extra::get_boost_pad_state(self, index)
    }

    #[inline]
    pub fn iter_pad_state(&self) -> impl Iterator<Item = sim::boostpad::BoostPadState> + '_ {
        (0..self.num_boost_pads()).map(move |id| self.get_pad_state(id))
    }

    #[inline]
    pub fn set_pad_state(self: Pin<&mut Self>, state: &sim::boostpad::BoostPadState) {
        extra::set_boost_pad_state(self, state);
    }

    #[inline]
    pub fn step(self: Pin<&mut Self>, ticks: i32) {
        self.Step(c_int(ticks));
    }

    #[inline]
    pub fn reset_to_random_kickoff(self: Pin<&mut Self>, seed: Option<i32>) {
        self.ResetToRandomKickoff(c_int(seed.unwrap_or(-1)));
    }

    #[inline]
    pub fn get_tick_count(&self) -> u64 {
        extra::get_tick_count(self)
    }
    
    #[inline]
    pub fn get_tick_rate(&self) -> f32 {
        extra::get_tick_rate(self)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct btVector3(std::arch::x86_64::__m128);
pub type Vec3 = btVector3;

unsafe impl cxx::ExternType for btVector3 {
    type Id = cxx::type_id!("btVector3");
    type Kind = cxx::kind::Trivial;
}

impl std::fmt::Display for btVector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [x, y, z] = self.to_glam().to_array();
        write!(f, "[{}, {}, {}]", x, y, z)
    }
}

#[cxx::bridge]
mod bulletlink {
    unsafe extern "C++" {
        include!("BulletLink.h");
        type btVector3 = crate::btVector3;
        type Angle;
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct Angle {
        yaw: f32,
        pitch: f32,
        roll: f32,
    }

    impl UniquePtr<btVector3> {}
}

pub use bulletlink::Angle;

impl From<btVector3> for glam::Vec3A {
    #[inline]
    fn from(value: btVector3) -> Self {
        value.to_glam()
    }
}

impl From<glam::Vec3A> for btVector3 {
    #[inline]
    fn from(value: glam::Vec3A) -> Self {
        Self::from_glam(value)
    }
}

impl btVector3 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self::from_glam(Vec3A::new(x, y, z))
    }

    #[inline]
    pub const fn from_array(a: [f32; 3]) -> Self {
        Self::from_glam(Vec3A::from_array(a))
    }

    #[inline]
    pub const fn to_glam(self) -> Vec3A {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    pub const fn from_glam(v: Vec3A) -> Self {
        unsafe { mem::transmute(v) }
    }
}

// autocxx::include_cpp! {
//     #include "CollisionMeshFile/CollisionMeshFile.h"
//     name!(meshloader)
//     safety!(unsafe)
//     block!("ReadFromFile")
//     generate!("CollisionMeshFile")
// }

// pub use meshloader::CollisionMeshFile;

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
            // extern_cpp_type!("CollisionMeshFile", crate::CollisionMeshFile)
            block!("btManifoldPoint")
            block!("btDynamicsWorld")
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
    }

    pub mod ball {
        autocxx::include_cpp! {
            #include "Sim/Ball/Ball.h"
            name!(ball)
            safety!(unsafe)
            extern_cpp_type!("btVector3", crate::Vec3)
            block!("btManifoldPoint")
            block!("btDynamicsWorld")
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

            #[derive(Debug)]
            struct EBallState {
                pos: UniquePtr<btVector3>,
                vel: UniquePtr<btVector3>,
                angvel: UniquePtr<btVector3>,
            }

            impl UniquePtr<EBallState> {}
        }

        pub use ball::Ball;
        pub use inner_bs::EBallState as BallState;
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

            #[derive(Debug)]
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
                contactNormal: UniquePtr<btVector3>,
                isContactingCar: bool,
                otherCar: u32,
                cooldownTimer: f32,
                isDemoed: bool,
                demoRespawnTimer: f32,
                lastHitBallTick: u64,
                lastControls: CarControls,
            }

            impl UniquePtr<ECarState> {}
        }

        pub use car::{Car, Team};
        pub use inner_cs::ECarState as CarState;

        impl CarState {
            #[inline]
            pub fn get_contacting_car(&self, arena: std::pin::Pin<&mut super::arena::Arena>) -> Option<cxx::UniquePtr<Self>> {
                if self.isContactingCar {
                    Some(arena.get_car_state_from_id(self.otherCar).ok()?)
                } else {
                    None
                }
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

            #[derive(Debug)]
            struct WheelPairConfig {
                wheelRadius: f32,
                suspensionRestLength: f32,
                connectionPointOffset: UniquePtr<btVector3>,
            }

            impl UniquePtr<WheelPairConfig> {}

            #[derive(Debug)]
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
