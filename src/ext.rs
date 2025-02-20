use crate::{
    base, consts,
    math::{Angle, RotMat, Vec3},
    sim::{
        Arena, ArenaConfig, ArenaMemWeightMode, BallHitInfo, BallState, BoostPadConfig, BoostPadState, CarConfig,
        CarContact, CarControls, CarState, DemoMode, GameMode, HeatseekerInfo, MutatorConfig, Team, WorldContact,
    },
};
use core::pin::Pin;
use cxx::UniquePtr;
use std::{error::Error, fmt};

#[cfg(feature = "serde_utils")]
use crate::serde_utils;
#[cfg(feature = "serde_utils")]
use serde::{Deserialize, Serialize};

impl CarConfig {
    #[inline]
    #[must_use]
    pub fn octane() -> &'static Self {
        base::get_octane()
    }

    #[inline]
    #[must_use]
    pub fn dominus() -> &'static Self {
        base::get_dominus()
    }

    #[inline]
    #[must_use]
    pub fn plank() -> &'static Self {
        base::get_plank()
    }

    #[inline]
    #[must_use]
    pub fn breakout() -> &'static Self {
        base::get_breakout()
    }

    #[inline]
    #[must_use]
    pub fn hybrid() -> &'static Self {
        base::get_hybrid()
    }

    #[inline]
    #[must_use]
    pub fn merc() -> &'static Self {
        base::get_merc()
    }
}

impl Default for ArenaConfig {
    fn default() -> Self {
        Self {
            mem_weight_mode: ArenaMemWeightMode::Heavy,
            min_pos: Vec3::new(-4500., -6000., 0.),
            max_pos: Vec3::new(4500., 6000., 2500.),
            max_aabb_len: 370.,
            no_ball_rot: true,
            use_custom_broadphase: true,
            max_objects: 512,
        }
    }
}

impl MutatorConfig {
    #[must_use]
    pub fn default(game_mode: GameMode) -> Self {
        Self {
            gravity: Vec3::new(0., 0., consts::GRAVITY_Z),
            car_mass: consts::CAR_MASS_BT,
            car_world_friction: consts::CARWORLD_COLLISION_FRICTION,
            car_world_restitution: consts::CARWORLD_COLLISION_RESTITUTION,
            ball_mass: if game_mode == GameMode::Snowday {
                consts::snowday::PUCK_MASS_BT
            } else {
                consts::BALL_MASS_BT
            },
            ball_max_speed: consts::BALL_MAX_SPEED,
            ball_drag: consts::BALL_DRAG,
            ball_world_friction: if game_mode == GameMode::Snowday {
                consts::snowday::PUCK_FRICTION
            } else {
                consts::BALL_FRICTION
            },
            ball_world_restitution: if game_mode == GameMode::Snowday {
                consts::snowday::PUCK_RESTITUTION
            } else {
                consts::BALL_RESTITUTION
            },
            jump_accel: consts::JUMP_ACCEL,
            jump_immediate_force: consts::JUMP_IMMEDIATE_FORCE,
            boost_accel_ground: consts::BOOST_ACCEL_GROUND,
            boost_accel_air: consts::BOOST_ACCEL_AIR,
            boost_used_per_second: consts::BOOST_USED_PER_SECOND,
            respawn_delay: consts::DEMO_RESPAWN_TIME,
            bump_cooldown_time: consts::BUMP_COOLDOWN_TIME,
            boost_pad_cooldown_big: consts::boostpads::COOLDOWN_BIG,
            boost_pad_cooldown_small: consts::boostpads::COOLDOWN_SMALL,
            car_spawn_boost_amount: consts::BOOST_SPAWN_AMOUNT,
            ball_hit_extra_force_scale: 1.,
            bump_force_scale: 1.,
            ball_radius: match game_mode {
                GameMode::Hoops => consts::BALL_COLLISION_RADIUS_HOOPS,
                GameMode::Snowday => consts::snowday::PUCK_RADIUS,
                GameMode::Dropshot => consts::BALL_COLLISION_RADIUS_DROPSHOT,
                _ => consts::BALL_COLLISION_RADIUS_SOCCAR,
            },
            unlimited_flips: false,
            unlimited_double_jumps: false,
            demo_mode: DemoMode::Normal,
            enable_team_demos: false,
            goal_base_threshold_y: consts::SOCCAR_GOAL_SCORE_BASE_THRESHOLD_Y,
        }
    }
}

impl PartialEq for Angle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.pitch == other.pitch && self.yaw == other.yaw && self.roll == other.roll
    }
}

impl PartialEq for BoostPadState {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.is_active == other.is_active
            && self.cooldown == other.cooldown
            && self.cur_locked_car_id == other.cur_locked_car_id
            && self.prev_locked_car_id == other.prev_locked_car_id
    }
}

#[derive(Clone, Copy, Debug)]
/// Error for when a car with a given ID cannot be found
pub struct NoCarFound(pub u32);

impl Error for NoCarFound {}
impl fmt::Display for NoCarFound {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No car found in the given arena at the given ID {}.", self.0)
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde_utils", derive(Serialize, Deserialize))]
pub struct BoostPad {
    #[cfg_attr(feature = "serde_utils", serde(with = "serde_utils::BoostPadConfigDerive"))]
    pub config: BoostPadConfig,
    #[cfg_attr(feature = "serde_utils", serde(with = "serde_utils::BoostPadStateDerive"))]
    pub state: BoostPadState,
}

#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde_utils", derive(Serialize, Deserialize))]
pub struct CarInfo {
    pub id: u32,
    pub team: Team,
    #[cfg_attr(feature = "serde_utils", serde(with = "serde_utils::CarStateDerive"))]
    pub state: CarState,
    #[cfg_attr(feature = "serde_utils", serde(with = "serde_utils::CarConfigDerive"))]
    pub config: CarConfig,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde_utils", derive(Serialize, Deserialize))]
pub struct GameState {
    pub tick_rate: f32,
    pub tick_count: u64,
    pub game_mode: GameMode,
    pub cars: Vec<CarInfo>,
    #[cfg_attr(feature = "serde_utils", serde(with = "serde_utils::BallStateDerive"))]
    pub ball: BallState,
    pub pads: Vec<BoostPad>,
}

impl Arena {
    #[inline]
    #[must_use]
    /// Create a new arena with the given game mode, arena config, and tick rate
    ///
    /// Tick rate MUST be equal to or between 15 and 120
    pub fn new(game_mode: GameMode, config: ArenaConfig, tick_rate: u8) -> UniquePtr<Self> {
        base::CreateArena(game_mode, config, tick_rate)
    }

    #[inline]
    #[must_use]
    /// Create a new standard arena running at the max TPS
    pub fn default_standard() -> cxx::UniquePtr<Self> {
        Self::new(GameMode::Soccar, ArenaConfig::default(), 120)
    }

    #[inline]
    #[must_use]
    /// Create a new hoops arena running at the max TPS
    pub fn default_hoops() -> cxx::UniquePtr<Self> {
        Self::new(GameMode::Hoops, ArenaConfig::default(), 120)
    }

    #[inline]
    #[must_use]
    /// Create a new heatseeker arena running at the max TPS
    pub fn default_heatseeker() -> cxx::UniquePtr<Self> {
        Self::new(GameMode::Heatseeker, ArenaConfig::default(), 120)
    }

    #[inline]
    #[must_use]
    /// Create a new snowday arena running at the max TPS
    pub fn default_snowday() -> cxx::UniquePtr<Self> {
        Self::new(GameMode::Snowday, ArenaConfig::default(), 120)
    }

    #[inline]
    /// Start ball and cars from random valid kickoff positions
    pub fn reset_to_random_kickoff(self: Pin<&mut Self>, seed: Option<i32>) {
        self.rtrk(seed.unwrap_or(-1));
    }

    #[inline]
    /// Remove the car at the given ID from the arena
    ///
    /// # Errors
    ///
    /// If there is no car with the given ID, this will return an error
    pub fn remove_car(self: Pin<&mut Self>, car_id: u32) -> Result<(), NoCarFound> {
        if self.rmvc(car_id) { Ok(()) } else { Err(NoCarFound(car_id)) }
    }

    #[inline]
    /// Sets the state of the car at the given ID
    ///
    /// # Errors
    ///
    /// If there is no car with the given ID, this will return an error
    pub fn set_car(self: Pin<&mut Self>, car_id: u32, car_state: CarState) -> Result<(), NoCarFound> {
        if self.rsc(car_id, car_state) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    /// Sets the controls of the car at the given ID
    ///
    /// # Errors
    ///
    /// If there is no car with the given ID, this will return an error
    pub fn set_car_controls(self: Pin<&mut Self>, car_id: u32, car_controls: CarControls) -> Result<(), NoCarFound> {
        if self.rscc(car_id, car_controls) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    /// Demolishes the car with the given ID
    ///
    /// # Errors
    ///
    /// If there is no car with the given ID, this will return an error
    pub fn demolish_car(self: Pin<&mut Self>, car_id: u32) -> Result<(), NoCarFound> {
        if self.dc(car_id) { Ok(()) } else { Err(NoCarFound(car_id)) }
    }

    #[inline]
    /// Respawns the car with the given ID with the given seed for the random spawn and the amount of boost to spawn the car with
    ///
    /// - If the seed is None, the seed will be random
    /// - If the boost amount is None, the boost amount will be 33.333
    ///
    /// # Errors
    ///
    /// If there is no car with the given ID, this will return an error
    pub fn respawn_car(
        self: Pin<&mut Self>,
        car_id: u32,
        seed: Option<i32>,
        boost_amount: Option<f32>,
    ) -> Result<(), NoCarFound> {
        if self.rspc(car_id, seed.unwrap_or(-1), boost_amount.unwrap_or(100. / 3.)) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    #[must_use]
    /// Get all the avalible information on a car
    pub fn get_car_info(self: Pin<&mut Self>, car_id: u32) -> CarInfo {
        CarInfo {
            id: car_id,
            team: self.get_car_team(car_id),
            config: self.get_car_config(car_id),
            state: self.get_car(car_id),
        }
    }

    #[inline]
    #[must_use]
    /// Returns all of the `CarInfo`s in the arena
    pub fn get_car_infos(mut self: Pin<&mut Self>) -> Vec<CarInfo> {
        self.get_cars()
            .into_iter()
            .map(|car_id| self.as_mut().get_car_info(car_id))
            .collect()
    }

    #[inline]
    /// Iterates over the static `(position, is_big)` info of boost pads in the Arena
    pub fn iter_pad_config(&self) -> impl Iterator<Item = BoostPadConfig> + '_ {
        (0..self.num_pads()).map(|i| (self.get_pad_config(i)))
    }

    #[inline]
    /// Iterates over the dynamic `(is_active, cooldown)` info of the boost pads in the arena
    pub fn iter_pad_state(&self) -> impl Iterator<Item = BoostPadState> + '_ {
        (0..self.num_pads()).map(|i| self.get_pad_state(i))
    }

    #[inline]
    /// Returns an iterator over the all `BoostPad` information in the arena
    pub fn iter_pads(&self) -> impl Iterator<Item = BoostPad> + '_ {
        (0..self.num_pads()).map(|i| BoostPad {
            config: self.get_pad_config(i),
            state: self.get_pad_state(i),
        })
    }

    #[inline]
    /// Set the all of the car id <-> car control pairs in the arena
    ///
    /// # Errors
    ///
    /// Returns `NoCarFound` upon the first car that cannot be found from a given ID
    pub fn set_all_controls(mut self: Pin<&mut Self>, controls: &[(u32, CarControls)]) -> Result<(), NoCarFound> {
        controls
            .iter()
            .try_for_each(|&(car_id, car_controls)| self.as_mut().set_car_controls(car_id, car_controls))
    }

    #[inline]
    #[must_use]
    /// Get all game state information in one struct
    pub fn get_game_state(mut self: Pin<&mut Self>) -> GameState {
        GameState {
            tick_rate: self.get_tick_rate(),
            tick_count: self.get_tick_count(),
            game_mode: self.get_game_mode(),
            pads: self.iter_pads().collect(),
            ball: self.as_mut().get_ball(),
            cars: self.get_car_infos(),
        }
    }

    /// Full game state setter
    ///
    /// Note: Some things cannot be state set, such game tick count/tick rate - these will be ignored
    ///
    /// # Errors
    ///
    /// Returns `NoCarFound` upon the first car that cannot be found from a given ID
    pub fn set_game_state(mut self: Pin<&mut Self>, game_state: &GameState) -> Result<(), NoCarFound> {
        for car in &game_state.cars {
            self.as_mut().set_car(car.id, car.state)?;
        }

        for (i, pad) in game_state.pads.iter().enumerate() {
            self.as_mut().set_pad_state(i, pad.state);
        }

        self.set_ball(game_state.ball);

        Ok(())
    }

    #[inline]
    #[must_use]
    /// Returns true if the ball is probably going in, does not account for wall or ceiling bounces
    ///
    /// NOTE: Purposefully overestimates, just like the real RL's shot prediction
    ///
    /// To check which goal it will score in, use the ball's velocity
    ///
    /// # Arguments
    ///
    /// * `max_time` - The maximum time to check for, if None, will default to 0.2s
    /// * `extra_margin` - Adjust the score margin (negative to prevent overestimating), will default to 0 if None
    pub fn is_ball_probably_going_in(&self, max_time: Option<f32>, extra_margin: Option<f32>) -> bool {
        self.ibpgi(max_time.unwrap_or(0.2), extra_margin.unwrap_or_default())
    }
}

impl Default for HeatseekerInfo {
    #[inline]
    fn default() -> Self {
        Self {
            y_target_dir: 0.,
            cur_target_speed: consts::heatseeker::INITIAL_TARGET_SPEED,
            time_since_hit: 0.,
        }
    }
}

impl Default for BallState {
    #[inline]
    fn default() -> Self {
        Self {
            update_counter: 0,
            pos: Vec3::new(0., 0., 93.15),
            rot_mat: RotMat::IDENTITY,
            vel: Vec3::ZERO,
            ang_vel: Vec3::ZERO,
            hs_info: HeatseekerInfo::default(),
        }
    }
}

impl Default for CarState {
    #[inline]
    fn default() -> Self {
        Self {
            update_counter: 0,
            pos: Vec3::new(0., 0., 17.),
            rot_mat: RotMat::IDENTITY,
            vel: Vec3::ZERO,
            ang_vel: Vec3::ZERO,
            is_on_ground: true,
            wheels_with_contact: [true; 4],
            has_jumped: false,
            has_double_jumped: false,
            has_flipped: false,
            flip_rel_torque: Vec3::ZERO,
            jump_time: 0.,
            flip_time: 0.,
            is_flipping: false,
            is_jumping: false,
            air_time: 0.,
            air_time_since_jump: 0.,
            boost: 100. / 3.,
            time_spent_boosting: 0.,
            is_supersonic: false,
            supersonic_time: 0.,
            handbrake_val: 0.,
            is_auto_flipping: false,
            auto_flip_timer: 0.,
            auto_flip_torque_scale: 0.,
            world_contact: WorldContact {
                has_contact: false,
                contact_normal: Vec3::ZERO,
            },
            car_contact: CarContact {
                other_car_id: 0,
                cooldown_timer: 0.,
            },
            is_demoed: false,
            demo_respawn_timer: 0.,
            ball_hit_info: BallHitInfo::default(),
            last_controls: CarControls::default(),
        }
    }
}

impl CarState {
    #[inline]
    #[must_use]
    /// Returns the other Car that this Car is currently contacting, if any
    pub fn get_contacting_car(&self, arena: Pin<&mut Arena>) -> Option<Self> {
        if self.car_contact.other_car_id == 0 {
            None
        } else {
            Some(arena.get_car(self.car_contact.other_car_id))
        }
    }
}

impl fmt::Display for RotMat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "f: {}, r: {}, u: {}", self.forward, self.right, self.up)
    }
}

impl RotMat {
    pub const IDENTITY: Self = Self {
        forward: Vec3::X,
        right: Vec3::Y,
        up: Vec3::Z,
    };

    #[inline]
    #[must_use]
    pub const fn new(forward: Vec3, right: Vec3, up: Vec3) -> Self {
        Self { forward, right, up }
    }
}

impl Angle {
    #[inline]
    #[must_use]
    pub fn from_rotmat(rot_mat: RotMat) -> Self {
        base::AngleFromRotMat(rot_mat)
    }
}

impl fmt::Display for Angle {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(p: {}, y: {}, r: {})", self.pitch, self.yaw, self.roll)
    }
}

impl fmt::Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0., 0., 0.);
    pub const X: Self = Self::new(1., 0., 0.);
    pub const Y: Self = Self::new(0., 1., 0.);
    pub const Z: Self = Self::new(0., 0., 1.);

    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0. }
    }
}

pub struct LinearPieceCurve<const N: usize> {
    pub value_mappings: [(f32, f32); N],
}

impl<const N: usize> LinearPieceCurve<N> {
    /// Returns the output of the curve
    ///
    /// # Arguments
    ///
    /// * `input` - The input to the curve
    /// * `default_output` - The default output if N is 0
    #[must_use]
    pub fn get_output(&self, input: f32, default_output: Option<f32>) -> f32 {
        if N == 0 {
            return default_output.unwrap_or(1.);
        }

        let first_val_pair = self.value_mappings[0];

        if input <= first_val_pair.0 {
            return first_val_pair.1;
        }

        for i in 1..N {
            let after_pair = self.value_mappings[i];
            let before_pair = self.value_mappings[i - 1];

            if after_pair.0 > input {
                let range_between = after_pair.0 - before_pair.0;
                let val_diff_between = after_pair.1 - before_pair.1;
                let linear_interp_factor = (input - before_pair.0) / range_between;
                return before_pair.1 + val_diff_between * linear_interp_factor;
            }
        }

        self.value_mappings[N - 1].1
    }
}

#[cfg_attr(feature = "serde_utils", derive(Serialize, Deserialize))]
pub struct CarSpawnPos {
    pub x: f32,
    pub y: f32,
    pub yaw_ang: f32,
}

impl CarSpawnPos {
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32, yaw_ang: f32) -> Self {
        Self { x, y, yaw_ang }
    }
}
