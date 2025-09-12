#[cfg(feature = "serde_utils")]
use serde::{Deserialize, Serialize};

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde_utils", derive(Serialize, Deserialize))]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub(crate) w: f32,
}

unsafe impl cxx::ExternType for Vec3 {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::Vec");
    type Kind = cxx::kind::Trivial;
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde_utils", derive(Serialize, Deserialize))]
pub struct RotMat {
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

unsafe impl cxx::ExternType for RotMat {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::RotMat");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Angle {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

unsafe impl cxx::ExternType for Angle {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::Angle");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "RocketSim")]
mod inner_math {
    unsafe extern "C++" {
        include!("Math/MathTypes/MathTypes.h");

        type Angle = crate::math::Angle;
        type RotMat = crate::math::RotMat;

        #[must_use]
        #[cxx_name = "ToRotMat"]
        /// Converts the angle to a RocketSim rotation matrix
        fn to_rotmat(self: &Angle) -> RotMat;
    }
}
