#![warn(clippy::all)]
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
pub mod math;
pub mod sim;

mod ext;

pub use cxx;
pub use ext::*;

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
mod base {
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
        #[cxx_name = "EArenaConfig"]
        type ArenaConfig = crate::sim::ArenaConfig;
        #[rust_name = "Arena"]
        type Arenar = crate::sim::Arena;

        #[must_use]
        #[namespace = "RocketSim"]
        #[cxx_name = "GetStage"]
        pub fn get_stage() -> RocketSimStage;

        fn Init(folder: &str);

        /// Initializes the collision mesh system for `RocketSim` from memory
        #[cxx_name = "InitFromMem"]
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

pub use base::{get_stage, init_from_mem};

#[inline]
/// Initializes the collision mesh system for `RocketSim`
pub fn init(collision_meshes_folder: Option<&str>) {
    base::Init(collision_meshes_folder.unwrap_or("collision_meshes"));
}
