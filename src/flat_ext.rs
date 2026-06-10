use crate::{
    BoostPad, CarInfo, DropshotTile, GameState, TileState,
    flat::rocketsim as flat,
    math::{RotMat, Vec3},
    sim::{
        BallHitInfo, BallState, BoostPadConfig, BoostPadState, CarConfig, CarContact, CarControls, CarState, DropshotInfo,
        GameMode, HeatseekerInfo, Team, WheelPairConfig, WorldContact,
    },
};

pub const PACKET_SIZE_BYTES: usize = 8;

pub trait ToFlat {
    type Flat;

    fn to_flat(&self) -> Self::Flat;
}

pub trait FromFlat<T> {
    fn from_flat(flat: T) -> Self;
}

#[derive(Clone, Debug)]
pub enum RlviserMessage {
    Connection,
    Quit,
    Speed(f32),
    Paused(bool),
    GameState(Box<GameState>),
}

pub struct PacketCodec {
    builder: planus::Builder,
    buffer: Vec<u8>,
}

impl Default for PacketCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketCodec {
    #[must_use]
    pub fn new() -> Self {
        Self::with_capacity(1024)
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            builder: planus::Builder::with_capacity(capacity),
            buffer: Vec::with_capacity(capacity + PACKET_SIZE_BYTES),
        }
    }

    pub fn encode(&mut self, message: RlviserMessage) -> &[u8] {
        self.builder.clear();

        let packet = flat::Packet {
            message: message.to_flat(),
        };
        let payload = self.builder.finish(packet, None);
        let data_len_bin = u64::try_from(payload.len()).unwrap().to_be_bytes();

        self.buffer.clear();
        self.buffer.extend_from_slice(&data_len_bin);
        self.buffer.extend_from_slice(payload);

        &self.buffer
    }

    pub fn decode_payload(payload: &[u8]) -> planus::Result<Option<RlviserMessage>> {
        let packet: flat::Packet = <flat::PacketRef<'_> as planus::ReadAsRoot>::read_as_root(payload)?.try_into()?;
        Ok(Option::<RlviserMessage>::from_flat(packet.message))
    }

    #[must_use]
    pub fn packet_len_from_header(header: [u8; PACKET_SIZE_BYTES]) -> usize {
        PACKET_SIZE_BYTES + u64::from_be_bytes(header) as usize
    }
}

impl ToFlat for RlviserMessage {
    type Flat = flat::Message;

    fn to_flat(&self) -> Self::Flat {
        match self {
            Self::Connection => flat::Message::Connection(Box::default()),
            Self::Quit => flat::Message::Quit(Box::default()),
            Self::Speed(speed) => flat::Message::Speed(Box::new(flat::Speed { speed: *speed })),
            Self::Paused(paused) => flat::Message::Paused(Box::new(flat::Paused { paused: *paused })),
            Self::GameState(game_state) => flat::Message::GameState(Box::new(game_state.to_flat())),
        }
    }
}

impl FromFlat<flat::Message> for Option<RlviserMessage> {
    fn from_flat(message: flat::Message) -> Self {
        match message {
            flat::Message::Connection(_) => Some(RlviserMessage::Connection),
            flat::Message::Quit(_) => Some(RlviserMessage::Quit),
            flat::Message::Speed(speed) => Some(RlviserMessage::Speed(speed.speed)),
            flat::Message::Paused(paused) => Some(RlviserMessage::Paused(paused.paused)),
            flat::Message::GameState(game_state) => {
                Some(RlviserMessage::GameState(Box::new(GameState::from_flat(&*game_state))))
            }
            flat::Message::AddRender(_) | flat::Message::RemoveRender(_) => None,
        }
    }
}

impl ToFlat for GameState {
    type Flat = flat::GameState;

    fn to_flat(&self) -> Self::Flat {
        flat::GameState {
            tick_rate: self.tick_rate,
            tick_count: self.tick_count,
            game_mode: self.game_mode.to_flat(),
            cars: Some(self.cars.iter().map(ToFlat::to_flat).collect()),
            ball: self.ball.to_flat(),
            pads: Some(self.pads.iter().map(ToFlat::to_flat).collect()),
            tiles: (self.game_mode == GameMode::Dropshot).then(|| {
                Box::new(flat::DropshotTilesByTeam {
                    blue_tiles: self.tiles[0].iter().map(ToFlat::to_flat).collect(),
                    orange_tiles: self.tiles[1].iter().map(ToFlat::to_flat).collect(),
                })
            }),
        }
    }
}

impl FromFlat<&flat::GameState> for GameState {
    fn from_flat(game_state: &flat::GameState) -> Self {
        Self {
            tick_rate: game_state.tick_rate,
            tick_count: game_state.tick_count,
            game_mode: GameMode::from_flat(game_state.game_mode),
            cars: game_state
                .cars
                .as_deref()
                .unwrap_or_default()
                .iter()
                .map(CarInfo::from_flat)
                .collect(),
            ball: BallState::from_flat(game_state.ball),
            pads: game_state
                .pads
                .as_deref()
                .unwrap_or_default()
                .iter()
                .map(BoostPad::from_flat)
                .collect(),
            tiles: game_state.tiles.as_ref().map_or_else(Default::default, |tiles| {
                [
                    tiles.blue_tiles.iter().map(DropshotTile::from_flat).collect(),
                    tiles.orange_tiles.iter().map(DropshotTile::from_flat).collect(),
                ]
            }),
        }
    }
}

impl From<&GameState> for flat::GameState {
    fn from(game_state: &GameState) -> Self {
        game_state.to_flat()
    }
}

impl From<&flat::GameState> for GameState {
    fn from(game_state: &flat::GameState) -> Self {
        Self::from_flat(game_state)
    }
}

impl ToFlat for CarInfo {
    type Flat = flat::CarInfo;

    fn to_flat(&self) -> Self::Flat {
        flat::CarInfo {
            id: u64::from(self.id),
            team: self.team.to_flat(),
            state: Box::new(self.state.to_flat()),
            config: self.config.to_flat(),
        }
    }
}

impl FromFlat<&flat::CarInfo> for CarInfo {
    fn from_flat(car: &flat::CarInfo) -> Self {
        Self {
            id: car.id as u32,
            team: Team::from_flat(car.team),
            state: CarState::from_flat(&car.state),
            config: CarConfig::from_flat(car.config),
        }
    }
}

impl ToFlat for CarState {
    type Flat = flat::CarState;

    fn to_flat(&self) -> Self::Flat {
        flat::CarState {
            physics: flat::PhysState {
                pos: self.pos.to_flat(),
                rot_mat: self.rot_mat.to_flat(),
                vel: self.vel.to_flat(),
                ang_vel: self.ang_vel.to_flat(),
            },
            is_on_ground: self.is_on_ground,
            wheels_with_contact: flat::WheelsWithContact {
                front_left: self.wheels_with_contact[0],
                front_right: self.wheels_with_contact[1],
                rear_left: self.wheels_with_contact[2],
                rear_right: self.wheels_with_contact[3],
            },
            has_jumped: self.has_jumped,
            has_double_jumped: self.has_double_jumped,
            has_flipped: self.has_flipped,
            flip_rel_torque: self.flip_rel_torque.to_flat(),
            jump_time: self.jump_time,
            flip_time: self.flip_time,
            is_flipping: self.is_flipping,
            is_jumping: self.is_jumping,
            air_time: self.air_time,
            air_time_since_jump: self.air_time_since_jump,
            boost: self.boost,
            time_since_boosted: self.time_since_boosted,
            is_boosting: self.is_boosting,
            boosting_time: self.boosting_time,
            is_supersonic: self.is_supersonic,
            supersonic_time: self.supersonic_time,
            handbrake_val: self.handbrake_val,
            is_auto_flipping: self.is_auto_flipping,
            auto_flip_timer: self.auto_flip_timer,
            auto_flip_torque_scale: self.auto_flip_torque_scale,
            world_contact_normal: self
                .world_contact
                .has_contact
                .then(|| self.world_contact.contact_normal.to_flat()),
            car_contact: Some(Box::new(flat::CarContact {
                other_car_id: u64::from(self.car_contact.other_car_id),
                cooldown_timer: self.car_contact.cooldown_timer,
            })),
            is_demoed: self.is_demoed,
            demo_respawn_timer: self.demo_respawn_timer,
            ball_hit_info: self.ball_hit_info.is_valid.then(|| Box::new(self.ball_hit_info.to_flat())),
            last_controls: self.last_controls.to_flat(),
        }
    }
}

impl FromFlat<&flat::CarState> for CarState {
    fn from_flat(state: &flat::CarState) -> Self {
        Self {
            pos: Vec3::from_flat(state.physics.pos),
            rot_mat: RotMat::from_flat(state.physics.rot_mat),
            vel: Vec3::from_flat(state.physics.vel),
            ang_vel: Vec3::from_flat(state.physics.ang_vel),
            tick_count_since_update: 0,
            is_on_ground: state.is_on_ground,
            wheels_with_contact: [
                state.wheels_with_contact.front_left,
                state.wheels_with_contact.front_right,
                state.wheels_with_contact.rear_left,
                state.wheels_with_contact.rear_right,
            ],
            has_jumped: state.has_jumped,
            has_double_jumped: state.has_double_jumped,
            has_flipped: state.has_flipped,
            flip_rel_torque: Vec3::from_flat(state.flip_rel_torque),
            jump_time: state.jump_time,
            flip_time: state.flip_time,
            is_flipping: state.is_flipping,
            is_jumping: state.is_jumping,
            air_time: state.air_time,
            air_time_since_jump: state.air_time_since_jump,
            boost: state.boost,
            time_since_boosted: state.time_since_boosted,
            is_boosting: state.is_boosting,
            boosting_time: state.boosting_time,
            is_supersonic: state.is_supersonic,
            supersonic_time: state.supersonic_time,
            handbrake_val: state.handbrake_val,
            is_auto_flipping: state.is_auto_flipping,
            auto_flip_timer: state.auto_flip_timer,
            auto_flip_torque_scale: state.auto_flip_torque_scale,
            world_contact: WorldContact {
                has_contact: state.world_contact_normal.is_some(),
                contact_normal: state.world_contact_normal.map_or_else(Vec3::default, Vec3::from_flat),
            },
            car_contact: state.car_contact.as_ref().map_or(
                CarContact {
                    other_car_id: 0,
                    cooldown_timer: 0.,
                },
                |contact| CarContact {
                    other_car_id: contact.other_car_id as u32,
                    cooldown_timer: contact.cooldown_timer,
                },
            ),
            is_demoed: state.is_demoed,
            demo_respawn_timer: state.demo_respawn_timer,
            ball_hit_info: state
                .ball_hit_info
                .as_ref()
                .map_or_else(BallHitInfo::default, |hit| BallHitInfo::from_flat(&**hit)),
            last_controls: CarControls::from_flat(state.last_controls),
        }
    }
}

impl ToFlat for BallState {
    type Flat = flat::BallState;

    fn to_flat(&self) -> Self::Flat {
        flat::BallState {
            physics: flat::PhysState {
                pos: self.pos.to_flat(),
                rot_mat: self.rot_mat.to_flat(),
                vel: self.vel.to_flat(),
                ang_vel: self.ang_vel.to_flat(),
            },
            hs_info: flat::HeatseekerInfo {
                y_target_dir: self.hs_info.y_target_dir,
                cur_target_speed: self.hs_info.cur_target_speed,
                time_since_hit: self.hs_info.time_since_hit,
            },
            ds_info: flat::DropshotInfo {
                charge_level: self.ds_info.charge_level,
                accumulated_hit_force: self.ds_info.accumulated_hit_force,
                y_target_dir: self.ds_info.y_target_dir,
                has_damaged: self.ds_info.has_damaged,
                last_damage_tick: self.ds_info.last_damage_tick,
            },
        }
    }
}

impl FromFlat<flat::BallState> for BallState {
    fn from_flat(ball: flat::BallState) -> Self {
        Self {
            pos: Vec3::from_flat(ball.physics.pos),
            rot_mat: RotMat::from_flat(ball.physics.rot_mat),
            vel: Vec3::from_flat(ball.physics.vel),
            ang_vel: Vec3::from_flat(ball.physics.ang_vel),
            tick_count_since_update: 0,
            hs_info: HeatseekerInfo {
                y_target_dir: ball.hs_info.y_target_dir,
                cur_target_speed: ball.hs_info.cur_target_speed,
                time_since_hit: ball.hs_info.time_since_hit,
            },
            ds_info: DropshotInfo {
                charge_level: ball.ds_info.charge_level,
                accumulated_hit_force: ball.ds_info.accumulated_hit_force,
                y_target_dir: ball.ds_info.y_target_dir,
                has_damaged: ball.ds_info.has_damaged,
                last_damage_tick: ball.ds_info.last_damage_tick,
            },
        }
    }
}

impl ToFlat for BoostPad {
    type Flat = flat::BoostPadInfo;

    fn to_flat(&self) -> Self::Flat {
        flat::BoostPadInfo {
            config: flat::BoostPadConfig {
                pos: self.config.position.to_flat(),
                is_big: self.config.is_big,
            },
            state: flat::BoostPadState {
                is_active: self.state.is_active,
                cooldown: self.state.cooldown,
                cur_locked_car: u64::from(self.state.cur_locked_car_id),
                prev_locked_car_id: u64::from(self.state.prev_locked_car_id),
            },
        }
    }
}

impl FromFlat<&flat::BoostPadInfo> for BoostPad {
    fn from_flat(pad: &flat::BoostPadInfo) -> Self {
        Self {
            config: BoostPadConfig {
                position: Vec3::from_flat(pad.config.pos),
                is_big: pad.config.is_big,
            },
            state: BoostPadState {
                is_active: pad.state.is_active,
                cooldown: pad.state.cooldown,
                cur_locked_car_id: pad.state.cur_locked_car as u32,
                prev_locked_car_id: pad.state.prev_locked_car_id as u32,
            },
        }
    }
}

impl ToFlat for DropshotTile {
    type Flat = flat::DropshotTile;

    fn to_flat(&self) -> Self::Flat {
        flat::DropshotTile {
            pos: self.pos.to_flat(),
            state: self.state.to_flat(),
        }
    }
}

impl FromFlat<&flat::DropshotTile> for DropshotTile {
    fn from_flat(tile: &flat::DropshotTile) -> Self {
        Self {
            pos: Vec3::from_flat(tile.pos),
            state: TileState::from_flat(tile.state),
        }
    }
}

impl ToFlat for BallHitInfo {
    type Flat = flat::BallHitInfo;

    fn to_flat(&self) -> Self::Flat {
        flat::BallHitInfo {
            relative_pos_on_ball: self.relative_pos_on_ball.to_flat(),
            ball_pos: self.ball_pos.to_flat(),
            extra_hit_vel: self.extra_hit_vel.to_flat(),
            tick_count_when_hit: self.tick_count_when_hit,
            tick_count_when_extra_impulse_applied: self.tick_count_when_extra_impulse_applied,
        }
    }
}

impl FromFlat<&flat::BallHitInfo> for BallHitInfo {
    fn from_flat(hit: &flat::BallHitInfo) -> Self {
        Self {
            is_valid: true,
            relative_pos_on_ball: Vec3::from_flat(hit.relative_pos_on_ball),
            ball_pos: Vec3::from_flat(hit.ball_pos),
            extra_hit_vel: Vec3::from_flat(hit.extra_hit_vel),
            tick_count_when_hit: hit.tick_count_when_hit,
            tick_count_when_extra_impulse_applied: hit.tick_count_when_extra_impulse_applied,
        }
    }
}

impl ToFlat for CarConfig {
    type Flat = flat::CarConfig;

    fn to_flat(&self) -> Self::Flat {
        flat::CarConfig {
            hitbox_size: self.hitbox_size.to_flat(),
            hitbox_pos_offset: self.hitbox_pos_offset.to_flat(),
            front_wheels: self.front_wheels.to_flat(),
            back_wheels: self.back_wheels.to_flat(),
            three_wheels: self.three_wheels,
            dodge_deadzone: self.dodge_deadzone,
        }
    }
}

impl FromFlat<flat::CarConfig> for CarConfig {
    fn from_flat(config: flat::CarConfig) -> Self {
        Self {
            hitbox_size: Vec3::from_flat(config.hitbox_size),
            hitbox_pos_offset: Vec3::from_flat(config.hitbox_pos_offset),
            front_wheels: WheelPairConfig::from_flat(config.front_wheels),
            back_wheels: WheelPairConfig::from_flat(config.back_wheels),
            three_wheels: config.three_wheels,
            dodge_deadzone: config.dodge_deadzone,
        }
    }
}

impl ToFlat for WheelPairConfig {
    type Flat = flat::WheelPairConfig;

    fn to_flat(&self) -> Self::Flat {
        flat::WheelPairConfig {
            wheel_radius: self.wheel_radius,
            suspension_rest_length: self.suspension_rest_length,
            connection_point_offset: self.connection_point_offset.to_flat(),
        }
    }
}

impl FromFlat<flat::WheelPairConfig> for WheelPairConfig {
    fn from_flat(config: flat::WheelPairConfig) -> Self {
        Self {
            wheel_radius: config.wheel_radius,
            suspension_rest_length: config.suspension_rest_length,
            connection_point_offset: Vec3::from_flat(config.connection_point_offset),
        }
    }
}

impl ToFlat for CarControls {
    type Flat = flat::CarControls;

    fn to_flat(&self) -> Self::Flat {
        flat::CarControls {
            throttle: self.throttle,
            steer: self.steer,
            pitch: self.pitch,
            yaw: self.yaw,
            roll: self.roll,
            jump: self.jump,
            boost: self.boost,
            handbrake: self.handbrake,
        }
    }
}

impl FromFlat<flat::CarControls> for CarControls {
    fn from_flat(controls: flat::CarControls) -> Self {
        Self {
            throttle: controls.throttle,
            steer: controls.steer,
            pitch: controls.pitch,
            yaw: controls.yaw,
            roll: controls.roll,
            jump: controls.jump,
            boost: controls.boost,
            handbrake: controls.handbrake,
        }
    }
}

impl ToFlat for GameMode {
    type Flat = flat::GameMode;

    fn to_flat(&self) -> Self::Flat {
        match self {
            Self::Soccar => flat::GameMode::Soccar,
            Self::Hoops => flat::GameMode::Hoops,
            Self::Heatseeker => flat::GameMode::Heatseeker,
            Self::Snowday => flat::GameMode::Snowday,
            Self::Dropshot => flat::GameMode::Dropshot,
            Self::TheVoid => flat::GameMode::TheVoid,
        }
    }
}

impl FromFlat<flat::GameMode> for GameMode {
    fn from_flat(game_mode: flat::GameMode) -> Self {
        match game_mode {
            flat::GameMode::Soccar => Self::Soccar,
            flat::GameMode::Hoops => Self::Hoops,
            flat::GameMode::Heatseeker => Self::Heatseeker,
            flat::GameMode::Snowday => Self::Snowday,
            flat::GameMode::Dropshot => Self::Dropshot,
            flat::GameMode::TheVoid => Self::TheVoid,
        }
    }
}

impl ToFlat for Team {
    type Flat = flat::Team;

    fn to_flat(&self) -> Self::Flat {
        match self {
            Self::Blue => flat::Team::Blue,
            Self::Orange => flat::Team::Orange,
        }
    }
}

impl FromFlat<flat::Team> for Team {
    fn from_flat(team: flat::Team) -> Self {
        match team {
            flat::Team::Blue => Self::Blue,
            flat::Team::Orange => Self::Orange,
        }
    }
}

impl ToFlat for TileState {
    type Flat = flat::TileState;

    fn to_flat(&self) -> Self::Flat {
        match self {
            Self::Full => flat::TileState::Full,
            Self::Damaged => flat::TileState::Damaged,
            Self::Broken => flat::TileState::Broken,
        }
    }
}

impl FromFlat<flat::TileState> for TileState {
    fn from_flat(tile_state: flat::TileState) -> Self {
        match tile_state {
            flat::TileState::Full => Self::Full,
            flat::TileState::Damaged => Self::Damaged,
            flat::TileState::Broken => Self::Broken,
        }
    }
}

impl ToFlat for RotMat {
    type Flat = flat::Mat3;

    fn to_flat(&self) -> Self::Flat {
        flat::Mat3 {
            forward: self.forward.to_flat(),
            right: self.right.to_flat(),
            up: self.up.to_flat(),
        }
    }
}

impl FromFlat<flat::Mat3> for RotMat {
    fn from_flat(rot_mat: flat::Mat3) -> Self {
        Self {
            forward: Vec3::from_flat(rot_mat.forward),
            right: Vec3::from_flat(rot_mat.right),
            up: Vec3::from_flat(rot_mat.up),
        }
    }
}

impl ToFlat for Vec3 {
    type Flat = flat::Vec3;

    fn to_flat(&self) -> Self::Flat {
        flat::Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl FromFlat<flat::Vec3> for Vec3 {
    fn from_flat(vec: flat::Vec3) -> Self {
        Self::new(vec.x, vec.y, vec.z)
    }
}
