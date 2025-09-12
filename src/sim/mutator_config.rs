use crate::math::Vec3;

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

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MutatorConfig {
    pub gravity: Vec3,
    pub car_mass: f32,
    pub car_world_friction: f32,
    pub car_world_restitution: f32,
    pub ball_mass: f32,
    pub ball_max_speed: f32,
    pub ball_drag: f32,
    pub ball_world_friction: f32,
    pub ball_world_restitution: f32,
    pub jump_accel: f32,
    pub jump_immediate_force: f32,
    pub boost_accel_ground: f32,
    pub boost_accel_air: f32,
    pub boost_used_per_second: f32,
    pub respawn_delay: f32,
    pub bump_cooldown_time: f32,
    pub boost_pad_cooldown_big: f32,
    pub boost_pad_cooldown_small: f32,
    pub car_spawn_boost_amount: f32,
    pub ball_hit_extra_force_scale: f32,
    pub bump_force_scale: f32,
    pub ball_radius: f32,
    pub unlimited_flips: bool,
    pub unlimited_double_jumps: bool,
    pub recharge_boost_enabled: bool,
    pub recharge_boost_per_second: f32,
    pub recharge_boost_delay: f32,
    pub demo_mode: DemoMode,
    pub enable_team_demos: bool,
    /// Only used if the game mode has soccar goals (i.e. soccar, heatseeker, snowday)
    pub goal_base_threshold_y: f32,
}

unsafe impl cxx::ExternType for MutatorConfig {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::MutatorConfig");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/MutatorConfig/MutatorConfig.h");

        type DemoMode = crate::sim::DemoMode;
        type MutatorConfig = crate::sim::MutatorConfig;
    }
}
