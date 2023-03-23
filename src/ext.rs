use crate::sim::{
    arena::{Arena, GameMode},
    ball::Ball,
    boostpad::BoostPadState,
    car::{Car, CarConfig},
    math::{Angle, RotMat, Vec3},
    CarControls,
};
use autocxx::WithinUniquePtr;
use std::{error::Error, fmt, pin::Pin};

#[derive(Debug)]
pub struct NoCarFound(u32);

impl fmt::Display for NoCarFound {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No car found in the given arena at the given ID {}.", self.0)
    }
}

impl Error for NoCarFound {}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoostPad {
    pub is_big: bool,
    pub position: Vec3,
    pub state: BoostPadState,
}

#[derive(Clone, Debug, Default)]
pub struct GameState {
    pub tick_count: u64,
    pub cars: Vec<(u32, Car, CarConfig)>,
    pub ball: Ball,
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
    pub fn set_car(self: Pin<&mut Self>, car_id: u32, car_state: Car) -> Result<(), NoCarFound> {
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
    /// Respawns the car with the given ID with the given seed for the random spawn
    pub fn respawn_car(self: Pin<&mut Self>, car_id: u32, seed: Option<i32>) -> Result<(), NoCarFound> {
        if self.RespawnCar(car_id, seed.unwrap_or(-1)) {
            Ok(())
        } else {
            Err(NoCarFound(car_id))
        }
    }

    #[inline]
    /// Returns all of the `(id, Car, CarConfig)`s in the arena
    pub fn get_cars(mut self: Pin<&mut Self>) -> Vec<(u32, Car, CarConfig)> {
        self.as_mut()
            .rgc()
            .iter()
            .enumerate()
            .map(|(i, &state)| (self.get_car_id(i), state, self.get_car_config_from_index(i)))
            .collect()
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
    pub fn get_game_state(self: Pin<&mut Self>) -> GameState {
        GameState {
            tick_count: self.get_tick_count(),
            ball: self.get_ball(),
            pads: self.iter_pads().collect(),
            cars: self.get_cars(),
        }
    }
}

impl Default for Ball {
    #[inline]
    fn default() -> Self {
        Self {
            pos: Vec3::new(0., 0., 93.15),
            vel: Vec3::default(),
            ang_vel: Vec3::default(),
        }
    }
}

impl Default for Car {
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
            last_hit_ball_tick: 0,
            last_controls: CarControls::default(),
        }
    }
}

impl Car {
    #[inline]
    /// Returns the other Car that this Car is currently contacting, if any
    pub fn get_contacting_car(&self, arena: std::pin::Pin<&mut Arena>) -> Option<Self> {
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
