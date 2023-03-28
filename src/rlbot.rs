use core::pin::Pin;

use glam::{Mat3A, Quat, Vec3A};

use crate::{
    math::Angle,
    sim::{
        arena::Arena,
        ball::{BallHitInfo, BallState},
    },
};

#[derive(Clone, Copy, Debug, Default)]
pub struct BoostPadInfo {
    pub location: Vec3A,
    pub is_full_boost: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoostPad {
    pub is_active: bool,
    pub timer: f32,
}

#[derive(Clone, Debug, Default)]
pub struct FieldInfo {
    pub boost_pads: Vec<BoostPadInfo>,
    pub num_boosts: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Physics {
    pub location: Vec3A,
    pub rotation: Angle,
    pub velocity: Vec3A,
    pub angular_velocity: Vec3A,
}

impl From<BallState> for Physics {
    fn from(ball: BallState) -> Self {
        Self {
            location: ball.pos.into(),
            rotation: Angle::default(),
            velocity: ball.vel.into(),
            angular_velocity: ball.ang_vel.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Cuboid {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Car {
    pub physics: Physics,
    pub is_demolished: bool,
    pub has_wheel_contact: bool,
    pub is_super_sonic: bool,
    pub jumped: bool,
    pub double_jumped: bool,
    pub team: u8,
    pub boost: f32,
    pub hitbox: Cuboid,
    pub hitbox_offset: Vec3A,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Sphere {
    pub diameter: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Cylinder {
    pub diameter: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CollisionShape {
    pub type_: u8,
    pub box_: Cuboid,
    pub sphere: Sphere,
    pub cylinder: Cylinder,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LastTouch {
    pub time_seconds: f32,
    pub hit_location: Vec3A,
    pub team: u8,
    pub player_index: usize,
}

impl LastTouch {
    fn from(arena: &Arena, last_touch: BallHitInfo) -> Self {
        if last_touch.car_id == 0 {
            return Self::default();
        }

        let player_index = arena.get_car_index(last_touch.car_id);
        Self {
            time_seconds: last_touch.tick_count_when_hit as f32 / arena.get_tick_rate(),
            hit_location: last_touch.ball_pos.into(),
            team: arena.get_car_team_from_index(player_index) as u8,
            player_index,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Ball {
    pub physics: Physics,
    pub collision_shape: CollisionShape,
    pub last_touch: LastTouch,
}

impl Ball {
    fn from(mut arena: Pin<&mut Arena>) -> Self {
        let ball = arena.as_mut().get_ball();
        Ball {
            physics: ball.into(),
            collision_shape: CollisionShape {
                type_: 1,
                sphere: Sphere {
                    diameter: arena.get_ball_radius() * 2.,
                },
                ..Default::default()
            },
            last_touch: LastTouch::from(&arena, ball.hit_info),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GameInfo {
    pub seconds_elapsed: f32,
    pub world_gravity_z: f32,
    pub frame_num: u64,
}

#[derive(Clone, Debug, Default)]
pub struct GameTickPacket {
    pub game_cars: Vec<Car>,
    pub num_cars: usize,
    pub game_boosts: Vec<BoostPad>,
    pub num_boosts: usize,
    pub game_ball: Ball,
    pub game_info: GameInfo,
}

impl Arena {
    #[inline]
    pub fn get_game_tick_packet(mut self: Pin<&mut Self>) -> GameTickPacket {
        GameTickPacket {
            game_cars: self
                .as_mut()
                .rgc()
                .iter()
                .enumerate()
                .map(|(i, car)| {
                    let car_config = self.get_car_config_from_index(i);
                    Car {
                        physics: Physics {
                            location: car.pos.into(),
                            rotation: Angle::from(Quat::from_mat3a(&Mat3A::from(car.rot_mat))),
                            velocity: car.vel.into(),
                            angular_velocity: car.ang_vel.into(),
                        },
                        is_demolished: car.is_demoed,
                        has_wheel_contact: car.has_contact,
                        is_super_sonic: car.is_supersonic,
                        jumped: car.has_jumped,
                        double_jumped: car.has_double_jumped,
                        team: self.get_car_team_from_index(i) as u8,
                        boost: car.boost,
                        hitbox: Cuboid {
                            length: car_config.hitbox_size.x,
                            width: car_config.hitbox_size.y,
                            height: car_config.hitbox_size.z,
                        },
                        hitbox_offset: car_config.hitbox_pos_offset.into(),
                    }
                })
                .collect(),
            num_cars: self.num_cars(),
            game_boosts: self
                .iter_pad_state()
                .map(|pad| BoostPad {
                    is_active: pad.is_active,
                    timer: pad.cooldown,
                })
                .collect(),
            num_boosts: self.num_pads(),
            game_ball: Ball::from(self.as_mut()),
            game_info: GameInfo {
                seconds_elapsed: self.get_tick_count() as f32 / self.get_tick_rate(),
                world_gravity_z: -650.,
                frame_num: self.get_tick_count(),
            },
        }
    }
}

impl Arena {
    #[inline]
    pub fn get_field_info(&self) -> FieldInfo {
        FieldInfo {
            boost_pads: self
                .iter_pad_static()
                .map(|(is_full_boost, pos)| BoostPadInfo {
                    location: pos.into(),
                    is_full_boost,
                })
                .collect(),
            num_boosts: self.num_pads(),
        }
    }
}
