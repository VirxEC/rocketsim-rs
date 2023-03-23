#![cfg_attr(not(any(target_arch = "x86", target_arch = "x86_64")), feature(portable_simd))]

pub use glam;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
use core::simd::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type F32x4 = __m128;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
type F32x4 = f32x4;

use glam::{EulerRot, Mat3A, Quat, Vec3A, Vec4};

use crate::sim::math::{Angle, RotMat, Vec3};

impl From<RotMat> for Mat3A {
    #[inline]
    fn from(value: RotMat) -> Self {
        Self::from_cols(Vec3A::from(value.forward), Vec3A::from(value.right), Vec3A::from(value.up))
    }
}

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

impl From<Angle> for Quat {
    #[inline]
    fn from(value: Angle) -> Self {
        Self::from_euler(EulerRot::XYZ, value.roll, value.pitch, value.yaw)
    }
}

impl From<Quat> for Angle {
    #[inline]
    fn from(value: Quat) -> Self {
        let (roll, pitch, yaw) = value.to_euler(EulerRot::XYZ);
        Self { pitch, yaw, roll }
    }
}

impl From<Vec3> for Vec3A {
    #[inline]
    fn from(value: Vec3) -> Self {
        Vec3A::from(F32x4::from(value.to_glam()))
    }
}

impl From<Vec3A> for Vec3 {
    #[inline]
    fn from(value: Vec3A) -> Self {
        Self::from_glam(Vec4::from(F32x4::from(value)))
    }
}

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
