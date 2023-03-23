#[cfg(all(target_arch = "x86", feature = "glam"))]
use core::arch::x86::*;
#[cfg(all(target_arch = "x86_64", feature = "glam"))]
use core::arch::x86_64::*;
#[cfg(all(not(any(target_arch = "x86", target_arch = "x86_64")), feature = "glam"))]
use core::simd::*;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "glam"))]
type F32x4 = __m128;
#[cfg(all(not(any(target_arch = "x86", target_arch = "x86_64")), feature = "glam"))]
type F32x4 = f32x4;

#[cfg(feature = "glam")]
pub use glam::{EulerRot, Mat3A, Quat, Vec3A, Vec4};

use crate::{
    sim::{
        arena::Arena,
        ball::Ball,
        boostpad::BoostPadState,
        car::{Car, CarConfig},
        math::{Angle, RotMat, Vec3},
        CarControls,
    },
    NoCarFound,
};
use std::pin::Pin;

#[derive(Clone, Copy, Debug, Default)]
pub struct BoostPad {
    pub is_big: bool,
    pub position: Vec3,
    pub state: BoostPadState,
}

#[derive(Clone, Debug, Default)]
pub struct GameState {
    pub tick_count: u64,
    pub cars: Vec<(u32, Car, CarConfig)>,
    pub ball: Ball,
    pub pads: Vec<BoostPad>,
}

impl Arena {
    #[inline]
    /// Returns all of the `(id, Car, CarConfig)`s in the arena
    pub fn get_cars(mut self: Pin<&mut Self>) -> std::vec::Vec<(u32, Car, CarConfig)> {
        self.as_mut()
            .rgc()
            .iter()
            .enumerate()
            .map(|(i, &state)| (self.get_car_id(i), state, self.get_car_config_from_index(i)))
            .collect()
    }

    #[inline]
    /// Iterates over the static `(position, is_big)` info of boost pads in the Arena
    pub fn iter_pad_static(&self) -> impl Iterator<Item = (bool, Vec3)> + '_ {
        (0..self.num_pads()).map(|i| (self.get_pad_is_big(i), self.get_pad_pos(i)))
    }

    #[inline]
    /// Iterates over the dynamic `(is_active, cooldown)` info of the boost pads in the arena
    pub fn iter_pad_state(&self) -> impl Iterator<Item = BoostPadState> + '_ {
        (0..self.num_pads()).map(|i| self.get_pad_state(i))
    }

    #[inline]
    /// Returns an iterator over the all BoostPad information in the arena
    pub fn iter_pads(&self) -> impl Iterator<Item = BoostPad> + '_ {
        (0..self.num_pads()).map(|i| BoostPad {
            is_big: self.get_pad_is_big(i),
            position: self.get_pad_pos(i),
            state: self.get_pad_state(i),
        })
    }

    #[inline]
    /// Set the all of the car id <-> car control pairs in the arena
    pub fn set_all_controls(mut self: Pin<&mut Self>, controls: &[(u32, CarControls)]) -> Result<(), NoCarFound> {
        controls.iter().try_for_each(|&(car_id, car_controls)| self.as_mut().set_car_controls(car_id, car_controls))
    }

    #[inline]
    /// Get all game state information in one struct
    pub fn get_game_state(self: Pin<&mut Self>) -> GameState {
        GameState {
            tick_count: self.get_tick_count(),
            ball: self.get_ball(),
            pads: self.iter_pads().collect(),
            cars: self.get_cars(),
        }
    }
}

impl std::fmt::Display for RotMat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "f: {}, r: {}, u: {}", self.forward, self.right, self.up)
    }
}

#[cfg(feature = "glam")]
impl From<RotMat> for Mat3A {
    #[inline]
    fn from(value: RotMat) -> Self {
        Self::from_cols(Vec3A::from(value.forward), Vec3A::from(value.right), Vec3A::from(value.up))
    }
}

#[cfg(feature = "glam")]
impl From<Mat3A> for RotMat {
    #[inline]
    fn from(value: Mat3A) -> Self {
        Self {
            forward: Vec3::from(value.x_axis),
            right: Vec3::from(value.y_axis),
            up: Vec3::from(value.z_axis),
        }
    }
}

impl RotMat {
    pub fn get_identity() -> Self {
        Self {
            forward: Vec3::new(1., 0., 0.),
            right: Vec3::new(0., 1., 0.),
            up: Vec3::new(0., 0., 1.),
        }
    }
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(p: {}, y: {}, r: {})", self.pitch, self.yaw, self.roll)
    }
}

#[cfg(feature = "glam")]
impl From<Angle> for Quat {
    #[inline]
    fn from(value: Angle) -> Self {
        Self::from_euler(EulerRot::XYZ, value.roll, value.pitch, value.yaw)
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

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

#[cfg(feature = "glam")]
impl From<Vec3> for Vec3A {
    #[inline]
    fn from(value: Vec3) -> Self {
        Vec3A::from(F32x4::from(value.to_glam()))
    }
}

#[cfg(feature = "glam")]
impl From<Vec3A> for Vec3 {
    #[inline]
    fn from(value: Vec3A) -> Self {
        Self::from_glam(Vec4::from(F32x4::from(value)))
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
