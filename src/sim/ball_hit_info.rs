#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/BallHitInfo/BallHitInfo.h");

        #[rust_name = "Vec3"]
        type Vec = crate::math::Vec3;
        type BallHitInfo;
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct BallHitInfo {
        is_valid: bool,
        relative_pos_on_ball: Vec3,
        ball_pos: Vec3,
        extra_hit_vel: Vec3,
        tick_count_when_hit: u64,
        tick_count_when_extra_impulse_applied: u64,
    }
}

pub use base::BallHitInfo;
