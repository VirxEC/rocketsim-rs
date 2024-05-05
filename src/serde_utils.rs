/// Re-export of `serde` crate for dependents.
#[allow(unused_imports)]
pub use serde;

use crate::{
    math::{RotMat, Vec3},
    sim::{
        BallHitInfo, BallState, BoostPadState, CarConfig, CarContact, CarControls, CarState, GameMode, HeatseekerInfo, Team,
        WheelPairConfig, WorldContact,
    },
};
use serde::{Deserialize, Serialize};

// impl Serialize for Vec3 {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//                 let mut s = serializer.serialize_struct("Vec3", 4)?;
//                 s.serialize_field("x", &self.x)?;
//                 s.serialize_field("y", &self.y)?;
//                 s.serialize_field("z", &self.z)?;
//                 s.serialize_field("_w", &self._w)?;
//                 s.end()
//     }
// }

#[derive(Serialize, Deserialize)]
#[serde(remote = "BallHitInfo")]
pub struct BallHitInfoDerive {
    is_valid: bool,
    relative_pos_on_ball: Vec3,
    ball_pos: Vec3,
    extra_hit_vel: Vec3,
    tick_count_when_hit: u64,
    tick_count_when_extra_impulse_applied: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "BoostPadState")]
pub struct BoostPadStateDerive {
    is_active: bool,
    cooldown: f32,
    cur_locked_car_id: u32,
    prev_locked_car_id: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Team")]
#[allow(clippy::upper_case_acronyms)]
pub enum TeamDerive {
    BLUE,
    ORANGE,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "GameMode")]
#[allow(clippy::upper_case_acronyms)]
pub enum GameModeDerive {
    SOCCAR,
    HOOPS,
    HEATSEEKER,
    SNOWDAY,
    #[allow(non_camel_case_types)]
    THE_VOID,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "CarControls")]
pub struct CarControlsDerive {
    pub throttle: f32,
    pub steer: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub boost: bool,
    pub jump: bool,
    pub handbrake: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "WorldContact")]
pub struct WorldContactDerive {
    pub has_contact: bool,
    pub contact_normal: Vec3,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "CarContact")]
pub struct CarContactDerive {
    pub other_car_id: u32,
    pub cooldown_timer: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "CarState")]
pub struct CarStateDerive {
    update_counter: u64,
    pos: Vec3,
    rot_mat: RotMat,
    vel: Vec3,
    ang_vel: Vec3,
    is_on_ground: bool,
    wheels_with_contact: [bool; 4],
    has_jumped: bool,
    has_double_jumped: bool,
    has_flipped: bool,
    flip_rel_torque: Vec3,
    jump_time: f32,
    flip_time: f32,
    is_flipping: bool,
    is_jumping: bool,
    air_time: f32,
    air_time_since_jump: f32,
    boost: f32,
    time_spent_boosting: f32,
    is_supersonic: bool,
    supersonic_time: f32,
    handbrake_val: f32,
    is_auto_flipping: bool,
    auto_flip_timer: f32,
    auto_flip_torque_scale: f32,
    #[serde(with = "WorldContactDerive")]
    world_contact: WorldContact,
    #[serde(with = "CarContactDerive")]
    car_contact: CarContact,
    is_demoed: bool,
    demo_respawn_timer: f32,
    #[serde(with = "BallHitInfoDerive")]
    ball_hit_info: BallHitInfo,
    #[serde(with = "CarControlsDerive")]
    last_controls: CarControls,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "WheelPairConfig")]
pub struct WheelPairConfigDerive {
    wheel_radius: f32,
    suspension_rest_length: f32,
    connection_point_offset: Vec3,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "CarConfig")]
pub struct CarConfigDerive {
    hitbox_size: Vec3,
    hitbox_pos_offset: Vec3,
    #[serde(with = "WheelPairConfigDerive")]
    front_wheels: WheelPairConfig,
    #[serde(with = "WheelPairConfigDerive")]
    back_wheels: WheelPairConfig,
    dodge_deadzone: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "HeatseekerInfo")]
pub struct HeatseekerInfoDerive {
    pub y_target_dir: f32,
    pub cur_target_speed: f32,
    pub time_since_hit: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "BallState")]
pub struct BallStateDerive {
    update_counter: u64,
    pos: Vec3,
    rot_mat: RotMat,
    vel: Vec3,
    ang_vel: Vec3,
    #[serde(with = "HeatseekerInfoDerive")]
    hs_info: HeatseekerInfo,
}
