use crate::{
    math::{Angle, RotMat, Vec3},
    sim::{Arena, BallHitInfo, BallState, BoostPadState, CarConfig, CarControls, CarState, DemoMode, GameMode, MutatorConfig, Team},
};
use autocxx::WithinUniquePtr;
use core::pin::Pin;
use cxx::UniquePtr;
use std::{error::Error, fmt};

impl Default for MutatorConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0., 0., -650.),
            car_mass: 180.,
            car_world_friction: 0.3,
            car_world_restitution: 0.3,
            ball_mass: 30.,
            ball_max_speed: 6000.,
            ball_drag: 0.3,
            ball_world_friction: 0.35,
            ball_world_restitution: 0.6,
            jump_accel: 4375. / 3.,
            jump_immediate_force: 875. / 3.,
            boost_force: 3816.,
            boost_used_per_second: 100. / 3.,
            respawn_delay: 3.,
            bump_cooldown_time: 0.25,
            boost_pad_cooldown_big: 10.,
            boost_pad_cooldown_small: 4.,
            car_spawn_boost_amount: 100. / 3.,
            ball_hit_extra_force_scale: 1.,
            bump_force_scale: 1.,
            ball_radius: 91.25,
            demo_mode: DemoMode::NORMAL,
            enable_team_demos: false,
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

impl Arena {
    #[inline]
    pub fn clone(self: Pin<&mut Self>, copy_callbacks: bool) -> UniquePtr<Self> {
        self.Clone(copy_callbacks).within_unique_ptr()
    }
}

#[derive(Debug)]
pub struct NoCarFound(u32);

impl fmt::Display for NoCarFound {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No car found in the given arena at the given ID {}.", self.0)
    }
}

impl Error for NoCarFound {}

impl Copy for Team {}

impl Default for Team {
    #[inline]
    fn default() -> Self {
        Self::BLUE
    }
}

impl fmt::Debug for Team {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BLUE => write!(f, "BLUE"),
            Self::ORANGE => write!(f, "ORANGE"),
        }
    }
}

impl Copy for DemoMode {}

impl fmt::Debug for DemoMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DISABLED => write!(f, "DISABLED"),
            Self::NORMAL => write!(f, "NORMAL"),
            Self::ON_CONTACT => write!(f, "ON_CONTACT"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoostPad {
    pub is_big: bool,
    pub position: Vec3,
    pub state: BoostPadState,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CarInfo {
    pub id: u32,
    pub team: Team,
    pub state: CarState,
    pub config: CarConfig,
}

#[derive(Clone, Debug, Default)]
pub struct GameState {
    pub tick_rate: f32,
    pub tick_count: u64,
    pub cars: Vec<CarInfo>,
    pub ball: BallState,
    pub ball_rot: [f32; 4],
    pub pads: Vec<BoostPad>,
}

impl Arena {
    #[inline]
    /// Create a new standard arena running at the max TPS
    pub fn default_standard() -> cxx::UniquePtr<Self> {
        Self::new(GameMode::SOCCAR, 120.).within_unique_ptr()
    }

    #[inline]
    /// Start ball and cars from random valid kickoff positions
    pub fn reset_to_random_kickoff(self: Pin<&mut Self>, seed: Option<i32>) {
        self.ResetToRandomKickoff(seed.unwrap_or(-1));
    }

    #[inline]
    /// Remove the car at the given ID from the arena
    pub fn remove_car(self: Pin<&mut Self>, car_id: u32) -> Result<(), NoCarFound> {
        if self.RemoveCar(car_id) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    /// Sets the state of the car at the given ID
    pub fn set_car(self: Pin<&mut Self>, car_id: u32, car_state: CarState) -> Result<(), NoCarFound> {
        if self.rsc(car_id, car_state) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    /// Sets the controls of the car at the given ID
    pub fn set_car_controls(self: Pin<&mut Self>, car_id: u32, car_controls: CarControls) -> Result<(), NoCarFound> {
        if self.rscc(car_id, car_controls) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    /// Demolishes the car with the given ID
    pub fn demolish_car(self: Pin<&mut Self>, car_id: u32) -> Result<(), NoCarFound> {
        if self.DemolishCar(car_id) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    /// Respawns the car with the given ID with the given seed for the random spawn and the amount of boost to spawn the car with
    ///
    /// - If the seed is None, the seed will be random
    /// - If the boost amount is None, the boost amount will be 33.333
    pub fn respawn_car(self: Pin<&mut Self>, car_id: u32, seed: Option<i32>, boost_amount: Option<f32>) -> Result<(), NoCarFound> {
        if self.RespawnCar(car_id, seed.unwrap_or(-1), boost_amount.unwrap_or(100. / 3.)) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    // Returns all of the car ids
    pub fn get_cars(&self) -> Vec<u32> {
        self.GetCars().iter().copied().collect()
    }

    #[inline]
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
    /// Returns all of the `CarInfo`s in the arena
    pub fn get_car_infos(mut self: Pin<&mut Self>) -> Vec<CarInfo> {
        self.GetCars().iter().map(|&car_id| self.as_mut().get_car_info(car_id)).collect()
    }

    #[inline]
    /// Iterates over the static `(position, is_big)` info of boost pads in the Arena
    pub fn iter_pad_static(&self) -> impl Iterator<Item = (bool, Vec3)> + '_ {
        (0..self.num_pads()).map(|i| (self.get_pad_is_big(i), self.get_pad_pos(i)))
    }

    #[inline]
    /// Iterates over the dynamic `(is_active, cooldown)` info of the boost pads in the arena
    pub fn iter_pad_state(&self) -> impl Iterator<Item = BoostPadState> + '_ {
        (0..self.num_pads()).map(|i| self.get_pad_state(i))
    }

    #[inline]
    /// Returns an iterator over the all BoostPad information in the arena
    pub fn iter_pads(&self) -> impl Iterator<Item = BoostPad> + '_ {
        (0..self.num_pads()).map(|i| BoostPad {
            is_big: self.get_pad_is_big(i),
            position: self.get_pad_pos(i),
            state: self.get_pad_state(i),
        })
    }

    #[inline]
    /// Set the all of the car id <-> car control pairs in the arena
    pub fn set_all_controls(mut self: Pin<&mut Self>, controls: &[(u32, CarControls)]) -> Result<(), NoCarFound> {
        controls.iter().try_for_each(|&(car_id, car_controls)| self.as_mut().set_car_controls(car_id, car_controls))
    }

    #[inline]
    /// Get all game state information in one struct
    pub fn get_game_state(mut self: Pin<&mut Self>) -> GameState {
        GameState {
            tick_rate: self.get_tick_rate(),
            tick_count: self.get_tick_count(),
            pads: self.iter_pads().collect(),
            ball: self.as_mut().get_ball(),
            ball_rot: self.get_ball_rotation(),
            cars: self.get_car_infos(),
        }
    }

    /// Full game state setter
    ///
    /// Note: Some things cannot be state set, such game tick count/tick rate - these will be ignored
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
    /// Returns true if the ball is probably going in, does not account for wall or ceiling bounces
    /// NOTE: Purposefully overestimates, just like the real RL's shot prediction
    /// To check which goal it will score in, use the ball's velocity
    ///
    /// # Arguments
    ///
    /// * `max_time` - The maximum time to check for, if None, will default to 0.2s
    pub fn is_ball_probably_going_in(&self, max_time: Option<f32>) -> bool {
        self.IsBallProbablyGoingIn(max_time.unwrap_or(0.2))
    }
}

impl Default for BallState {
    #[inline]
    fn default() -> Self {
        Self {
            pos: Vec3::new(0., 0., 93.15),
            vel: Vec3::default(),
            ang_vel: Vec3::default(),
        }
    }
}

impl Default for CarState {
    #[inline]
    fn default() -> Self {
        Self {
            pos: Vec3::new(0., 0., 17.),
            rot_mat: RotMat::get_identity(),
            vel: Vec3::default(),
            ang_vel: Vec3::default(),
            is_on_ground: true,
            has_jumped: false,
            has_double_jumped: false,
            has_flipped: false,
            last_rel_dodge_torque: Vec3::default(),
            jump_time: 0.,
            flip_time: 0.,
            is_jumping: false,
            air_time_since_jump: 0.,
            boost: 100. / 3.,
            time_spent_boosting: 0.,
            is_supersonic: false,
            supersonic_time: 0.,
            handbrake_val: 0.,
            is_auto_flipping: false,
            auto_flip_timer: 0.,
            auto_flip_torque_scale: 0.,
            has_contact: false,
            contact_normal: Vec3::default(),
            other_car_id: 0,
            cooldown_timer: 0.,
            is_demoed: false,
            demo_respawn_timer: 0.,
            ball_hit_info: BallHitInfo::default(),
            last_controls: CarControls::default(),
        }
    }
}

impl CarState {
    #[inline]
    /// Returns the other Car that this Car is currently contacting, if any
    pub fn get_contacting_car(&self, arena: Pin<&mut Arena>) -> Option<Self> {
        if self.other_car_id != 0 {
            Some(arena.get_car(self.other_car_id))
        } else {
            None
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
    #[inline]
    /// Returns the identity rotation matrix
    pub fn get_identity() -> Self {
        Self {
            forward: Vec3::new(1., 0., 0.),
            right: Vec3::new(0., 1., 0.),
            up: Vec3::new(0., 0., 1.),
        }
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
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, _w: 0. }
    }
}
