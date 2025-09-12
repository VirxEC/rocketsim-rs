use crate::math::Vec3;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BallHitInfo {
    pub is_valid: bool,
    pub relative_pos_on_ball: Vec3,
    pub ball_pos: Vec3,
    pub extra_hit_vel: Vec3,
    pub tick_count_when_hit: u64,
    pub tick_count_when_extra_impulse_applied: u64,
}

unsafe impl cxx::ExternType for BallHitInfo {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::BallHitInfo");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/BallHitInfo/BallHitInfo.h");

        type BallHitInfo = crate::sim::BallHitInfo;
    }
}
