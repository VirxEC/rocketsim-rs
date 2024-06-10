#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde_utils", derive(serde::Serialize, serde::Deserialize))]
pub struct HeatseekerInfo {
    /// Which net the ball should seek towards;
    /// When 0, no net
    pub y_target_dir: f32,
    pub cur_target_speed: f32,
    pub time_since_hit: f32,
}

unsafe impl cxx::ExternType for HeatseekerInfo {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::BallState::HeatseekerInfo");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/Ball/Ball.h");

        #[rust_name = "Vec3"]
        type Vec = crate::math::Vec3;
        type RotMat = crate::math::RotMat;
        type BallState;
        #[namespace = "RocketSim::BallState"]
        type HeatseekerInfo = crate::sim::HeatseekerInfo;
    }

    #[derive(Clone, Copy, Debug)]
    struct BallState {
        pos: Vec3,
        rot_mat: RotMat,
        vel: Vec3,
        ang_vel: Vec3,
        update_counter: u64,
        hs_info: HeatseekerInfo,
    }
}

pub use base::BallState;
