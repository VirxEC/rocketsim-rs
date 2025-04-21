#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde_utils", derive(serde::Serialize, serde::Deserialize))]
pub enum DemoMode {
    #[default]
    Normal = 0,
    OnContact = 1,
    Disabled = 2,
}

unsafe impl cxx::ExternType for DemoMode {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::DemoMode");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/MutatorConfig/MutatorConfig.h");

        #[rust_name = "Vec3"]
        type Vec = crate::math::Vec3;
        type DemoMode = crate::sim::DemoMode;

        type MutatorConfig;
    }

    #[derive(Clone, Copy, Debug)]
    struct MutatorConfig {
        gravity: Vec3,
        car_mass: f32,
        car_world_friction: f32,
        car_world_restitution: f32,
        ball_mass: f32,
        ball_max_speed: f32,
        ball_drag: f32,
        ball_world_friction: f32,
        ball_world_restitution: f32,
        jump_accel: f32,
        jump_immediate_force: f32,
        boost_accel_ground: f32,
        boost_accel_air: f32,
        boost_used_per_second: f32,
        respawn_delay: f32,
        bump_cooldown_time: f32,
        boost_pad_cooldown_big: f32,
        boost_pad_cooldown_small: f32,
        car_spawn_boost_amount: f32,
        ball_hit_extra_force_scale: f32,
        bump_force_scale: f32,
        ball_radius: f32,
        unlimited_flips: bool,
        unlimited_double_jumps: bool,
        recharge_boost_enabled: bool,
        recharge_boost_per_second: f32,
        recharge_boost_delay: f32,
        demo_mode: DemoMode,
        enable_team_demos: bool,
        /// Only used if the game mode has soccar goals (i.e. soccar, heatseeker, snowday)
        goal_base_threshold_y: f32,
    }
}

pub use base::MutatorConfig;
