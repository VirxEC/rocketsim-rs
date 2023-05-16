use crate::math::Vec3;
use std::f32::consts::FRAC_1_SQRT_2;

pub const GRAVITY_Z: f32 = -650.;

pub const ARENA_EXTENT_X: f32 = 4096.;

/// Does not include inner-goal
pub const ARENA_EXTENT_Y: f32 = 5120.;

pub const ARENA_HEIGHT: f32 = 2048.;

pub const CAR_MASS_BT: f32 = 180.;

/// Ref: https://www.reddit.com/r/RocketLeague/comments/bmje9l/comment/emxkwrl/?context=3
pub const BALL_MASS_BT: f32 = CAR_MASS_BT / 6.;

pub const CAR_COLLISION_FRICTION: f32 = 0.3;

pub const CAR_COLLISION_RESTITUTION: f32 = 0.1;

pub const CARBALL_COLLISION_FRICTION: f32 = 2.0;

pub const CARBALL_COLLISION_RESTITUTION: f32 = 0.0;

pub const CARWORLD_COLLISION_FRICTION: f32 = 0.3;

pub const CARWORLD_COLLISION_RESTITUTION: f32 = 0.3;

pub const CARCAR_COLLISION_FRICTION: f32 = 0.09;

pub const CARCAR_COLLISION_RESTITUTION: f32 = 0.1;

/// Greater than ball radius because of arena mesh collision margin
pub const BALL_REST_Z: f32 = 93.15;

/// Ball can never exceed this angular velocity (radians/s)
pub const BALL_MAX_ANG_SPEED: f32 = 6.;

/// Net-velocity drag multiplier
pub const BALL_DRAG: f32 = 0.03;

pub const BALL_FRICTION: f32 = 0.35;

/// Bounce factor
pub const BALL_RESTITUTION: f32 = 0.6;

pub const CAR_MAX_SPEED: f32 = 2300.;

pub const BALL_MAX_SPEED: f32 = 6000.;

pub const BOOST_MAX: f32 = 100.;

pub const BOOST_USED_PER_SECOND: f32 = BOOST_MAX / 3.;

/// Minimum time we can be boosting for
pub const BOOST_MIN_TIME: f32 = 0.1;

pub const BOOST_ACCEL: f32 = 21.2;

pub const BOOST_SPAWN_AMOUNT: f32 = BOOST_MAX / 3.;

pub const BOOST_ACCEL_GROUND_DECAY_MIN_VEL: f32 = 600.;

pub const BOOST_ACCEL_GROUND_DECAY_AMOUNT: f32 = 0.072;

/// Car can never exceed this angular velocity (radians/s)
pub const CAR_MAX_ANG_SPEED: f32 = 5.5;

pub const SUPERSONIC_START_SPEED: f32 = 2200.;

pub const SUPERSONIC_MAINTAIN_MIN_SPEED: f32 = SUPERSONIC_START_SPEED - 100.;

pub const SUPERSONIC_MAINTAIN_MAX_TIME: f32 = 1.;

pub const POWERSLIDE_RISE_RATE: f32 = 5.;

pub const POWERSLIDE_FALL_RATE: f32 = 2.;

pub const THROTTLE_TORQUE_AMOUNT: f32 = CAR_MASS_BT * 400.;

pub const BRAKE_TORQUE_AMOUNT: f32 = CAR_MASS_BT * (14.25 + (1. / 3.));

/// If we are costing with less than this forward vel, we full-brake
pub const STOPPING_FORWARD_VEL: f32 = 25.;

/// How much the brake is applied when costing
pub const COASTING_BRAKE_FACTOR: f32 = 0.15;

/// Throttle input of less than this is ignored
pub const THROTTLE_DEADZONE: f32 = 0.001;

pub const THROTTLE_AIR_FORCE: f32 = 1. / 0.75;

pub const JUMP_ACCEL: f32 = 4375. / 3.;

pub const JUMP_IMMEDIATE_FORCE: f32 = 875. / 3.;

pub const JUMP_MIN_TIME: f32 = 0.025;

pub const JUMP_RESET_TIME_PAD: f32 = 1. / 40.;

pub const JUMP_MAX_TIME: f32 = 0.2;

/// Can be at most 1.25 seconds after the jump is finished
pub const DOUBLEJUMP_MAX_DELAY: f32 = 1.25;

pub const FLIP_Z_DAMP_120: f32 = 0.35;

pub const FLIP_Z_DAMP_START: f32 = 0.15;

pub const FLIP_Z_DAMP_END: f32 = 0.21;

pub const FLIP_TORQUE_TIME: f32 = 0.65;

pub const FLIP_TORQUE_MIN_TIME: f32 = 0.41;

pub const FLIP_PITCHLOCK_TIME: f32 = 1.;

pub const FLIP_INITIAL_VEL_SCALE: f32 = 500.;

/// Left/Right
pub const FLIP_TORQUE_X: f32 = 260.;

/// Forward/backward
pub const FLIP_TORQUE_Y: f32 = 224.;

pub const FLIP_FORWARD_IMPULSE_MAX_SPEED_SCALE: f32 = 1.;

pub const FLIP_SIDE_IMPULSE_MAX_SPEED_SCALE: f32 = 1.9;

pub const FLIP_BACKWARD_IMPULSE_MAX_SPEED_SCALE: f32 = 2.5;

pub const FLIP_BACKWARD_IMPULSE_SCALE_X: f32 = 16. / 15.;

/// Soccar, Hoops, etc.
pub const BALL_COLLISION_RADIUS_NORMAL: f32 = 91.25;

pub const BALL_COLLISION_RADIUS_DROPSHOT: f32 = 103.6;

pub const SOCCAR_GOAL_SCORE_BASE_THRESHOLD_Y: f32 = 5121.75;

pub const SOCCAR_BALL_SCORE_THRESHOLD_Y: f32 = SOCCAR_GOAL_SCORE_BASE_THRESHOLD_Y + BALL_COLLISION_RADIUS_NORMAL;

pub const CAR_TORQUE_SCALE: f32 = 0.09587;

pub const CAR_AUTOFLIP_IMPULSE: f32 = 200.;

pub const CAR_AUTOFLIP_TORQUE: f32 = 50.;

pub const CAR_AUTOFLIP_TIME: f32 = 0.4;

pub const CAR_AUTOFLIP_NORMZ_THRESH: f32 = FRAC_1_SQRT_2;

pub const CAR_AUTOROLL_FORCE: f32 = 100.;

pub const CAR_AUTOROLL_TORQUE: f32 = 80.;

pub const BALL_CAR_EXTRA_IMPULSE_Z_SCALE: f32 = 0.35;

pub const BALL_CAR_EXTRA_IMPULSE_FORWARD_SCALE: f32 = 0.65;

pub const BALL_CAR_EXTRA_IMPULSE_MAXDELTAVEL_UU: f32 = 4600.;

pub const CAR_SPAWN_REST_Z: f32 = 17.;

pub const CAR_RESPAWN_Z: f32 = 36.;

pub const BUMP_COOLDOWN_TIME: f32 = 0.25;

pub const BUMP_MIN_FORWARD_DIST: f32 = 64.5;

pub const DEMO_RESPAWN_TIME: f32 = 3.;

pub const CAR_AIR_CONTROL_TORQUE: Vec3 = Vec3::new(130., 95., 400.);

pub const CAR_AIR_CONTROL_DAMPING: Vec3 = Vec3::new(30., 20., 50.);

pub const CAR_SPAWN_LOCATION_AMOUNT: i32 = 5;

pub const CAR_RESPAWN_LOCATION_AMOUNT: i32 = 4;

pub mod btvehicle {
    pub const SUSPENSION_FORCE_SCALE_FRONT: f32 = 36. - (1. / 4.);

    pub const SUSPENSION_FORCE_SCALE_BACK: f32 = 54. + (1. / 4.) + (1.5 / 100.);

    pub const SUSPENSION_STIFFNESS: f32 = 500.;

    pub const WHEELS_DAMPING_COMPRESSION: f32 = 25.;

    pub const WHEELS_DAMPING_RELAXATION: f32 = 40.;

    /// TODO: Are we sure this is the same for all cars?
    pub const MAX_SUSPENSION_TRAVEL: f32 = 12.;
}

pub mod boostpads {
    pub const CYL_HEIGHT: f32 = 95.;

    pub const CYL_RAD_BIG: f32 = 208.;

    pub const CYL_RAD_SMALL: f32 = 144.;

    pub const BOX_HEIGHT: f32 = 64.;

    pub const BOX_RAD_BIG: f32 = 160.;

    pub const BOX_RAD_SMALL: f32 = 120.;

    pub const COOLDOWN_BIG: f32 = 10.;

    pub const COOLDOWN_SMALL: f32 = 4.;

    pub const BOOST_AMOUNT_BIG: f32 = 100.;

    pub const BOOST_AMOUNT_SMALL: f32 = 12.;

    pub const LOCS_AMOUNT_SMALL: i32 = 28;

    pub const LOCS_AMOUNT_BIG: i32 = 6;
}
