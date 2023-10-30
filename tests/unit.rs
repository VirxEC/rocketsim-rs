use std::{
    f32::consts::PI,
    sync::{
        atomic::{AtomicBool, Ordering},
        Once,
    },
};

use rocketsim_rs::{
    init,
    math::{Angle, Vec3},
    sim::{Arena, BallState, CarConfig, CarControls, CarState, Team},
};

#[cfg(feature = "glam")]
use rocketsim_rs::glam_ext::GameStateA;

static INIT: Once = Once::new();

#[test]
fn pads() {
    INIT.call_once(|| init(None));
    let arena = Arena::default_standard();

    let statics = arena.iter_pad_static().collect::<Vec<_>>();
    assert!(statics.len() == arena.num_pads());

    let states = arena.iter_pad_state().collect::<Vec<_>>();
    assert!(states.len() == arena.num_pads());
}

#[test]
fn cars() {
    INIT.call_once(|| init(None));
    let mut arena = Arena::default_standard();

    let car_id = arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());
    assert_eq!(arena.pin_mut().get_cars().len(), 1);

    arena.pin_mut().remove_car(car_id).unwrap();
    assert!(arena.pin_mut().get_cars().is_empty());

    let dominus = CarConfig::dominus();
    let car_id = arena.pin_mut().add_car(Team::ORANGE, dominus);

    arena
        .pin_mut()
        .set_car_controls(
            car_id,
            CarControls {
                boost: true,
                ..Default::default()
            },
        )
        .unwrap();

    arena.pin_mut().step(1);

    let cars = arena.pin_mut().get_car_infos();
    assert!(cars.len() == 1);

    let car_info = cars[0];

    assert_eq!(car_info.team, Team::ORANGE);
    assert!(car_info.state.boost < 100. / 3.);

    // this differs the most between cars so we'll just this
    assert_eq!(car_info.config.hitbox_size.x, dominus.hitbox_size.x);
    assert_eq!(car_info.config.hitbox_size.y, dominus.hitbox_size.y);
    assert_eq!(car_info.config.hitbox_size.z, dominus.hitbox_size.z);
}

#[test]
fn ball() {
    INIT.call_once(|| init(None));
    let mut arena = Arena::default_standard();

    arena.pin_mut().set_ball(BallState {
        pos: Vec3::new(1., 2., 1000.),
        vel: Vec3::new(0., 0., -1.),
        ..Default::default()
    });

    let ball = arena.pin_mut().get_ball();
    assert_eq!(ball.pos.x, 1.);
    assert_eq!(ball.pos.y, 2.);
    assert_eq!(ball.pos.z, 1000.);
    assert_eq!(ball.vel.x, 0.);
    assert_eq!(ball.vel.y, 0.);
    assert_eq!(ball.vel.z, -1.);
    assert_eq!(ball.ang_vel.x, 0.);
    assert_eq!(ball.ang_vel.y, 0.);
    assert_eq!(ball.ang_vel.z, 0.);

    arena.pin_mut().step(30);

    let ball = arena.pin_mut().get_ball();
    assert_eq!(ball.pos.x, 1.);
    assert_eq!(ball.pos.y, 2.);
    assert!(ball.pos.z < 1000.);
    assert_eq!(ball.vel.x, 0.);
    assert_eq!(ball.vel.y, 0.);
    assert!(ball.vel.z < 0.);

    assert_eq!(arena.get_ball_rotation(), [0., 0., 0., 1.]);
}

#[test]
fn game_state() {
    INIT.call_once(|| init(None));
    let mut arena = Arena::default_standard();
    let _ = arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout());
    let _ = arena.pin_mut().add_car(Team::BLUE, CarConfig::hybrid());
    arena.pin_mut().step(120);

    let game_state = arena.pin_mut().get_game_state();
    assert_eq!(game_state.tick_count, 120);
    assert!(game_state.ball.pos.z < 200.);
    assert_eq!(game_state.cars.len(), 2);
    assert_eq!(game_state.pads.len(), 34);

    #[cfg(feature = "glam")]
    {
        // test converison to glam
        let glam_state = GameStateA::from(game_state);
        assert_eq!(glam_state.tick_count, 120);
        assert!(glam_state.ball.pos.z < 200.);
        assert_eq!(glam_state.cars.len(), 2);
        assert_eq!(glam_state.pads.len(), 34);
    }

    arena.pin_mut().reset_tick_count();
}

#[test]
fn angles() {
    INIT.call_once(|| init(None));
    let mut arena = Arena::default_standard();
    let ids = [
        arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout()),
        arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout()),
        arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout()),
        arena.pin_mut().add_car(Team::ORANGE, CarConfig::dominus()),
        arena.pin_mut().add_car(Team::ORANGE, CarConfig::dominus()),
        arena.pin_mut().add_car(Team::ORANGE, CarConfig::dominus()),
        arena.pin_mut().add_car(Team::ORANGE, CarConfig::merc()),
        arena.pin_mut().add_car(Team::ORANGE, CarConfig::merc()),
        arena.pin_mut().add_car(Team::ORANGE, CarConfig::merc()),
        arena.pin_mut().add_car(Team::BLUE, CarConfig::hybrid()),
        arena.pin_mut().add_car(Team::BLUE, CarConfig::hybrid()),
        arena.pin_mut().add_car(Team::BLUE, CarConfig::hybrid()),
        arena.pin_mut().add_car(Team::BLUE, CarConfig::octane()),
        arena.pin_mut().add_car(Team::BLUE, CarConfig::octane()),
        arena.pin_mut().add_car(Team::BLUE, CarConfig::octane()),
        arena.pin_mut().add_car(Team::BLUE, CarConfig::plank()),
        arena.pin_mut().add_car(Team::BLUE, CarConfig::plank()),
        arena.pin_mut().add_car(Team::BLUE, CarConfig::plank()),
    ];
    arena.pin_mut().reset_to_random_kickoff(None);

    for id in ids {
        let car = arena.pin_mut().get_car(id);
        let angles = Angle::from_rotmat(car.rot_mat);
        assert!(angles.yaw.abs() < PI);
        assert!(angles.pitch.abs() < 0.01);
        assert!(angles.roll.abs() < 0.01);
    }
}

#[cfg(feature = "rlbot")]
#[test]
fn rlbot() {
    INIT.call_once(|| init(None));
    let mut arena = Arena::default_standard();
    let _ = arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout());
    let _ = arena.pin_mut().add_car(Team::BLUE, CarConfig::hybrid());
    arena.pin_mut().step(120);

    let game_tick_packet = arena.pin_mut().get_game_tick_packet();

    assert_eq!(game_tick_packet.game_cars.len(), game_tick_packet.num_cars);
    assert_eq!(game_tick_packet.num_cars, 2);

    assert_eq!(game_tick_packet.game_boosts.len(), game_tick_packet.num_boosts);
    assert_eq!(game_tick_packet.num_boosts, 34);

    assert_eq!(game_tick_packet.game_ball.collision_shape.type_, 1);
    assert_eq!(game_tick_packet.game_ball.collision_shape.sphere.diameter, 91.25 * 2.);

    assert!(game_tick_packet.game_info.seconds_elapsed - 1. < 0.00001);
    assert_eq!(game_tick_packet.game_info.frame_num, 120);
    assert_eq!(game_tick_packet.game_info.world_gravity_z, -650.);
}

#[test]
fn goal_score() {
    static SCORED: AtomicBool = AtomicBool::new(false);
    INIT.call_once(|| init(None));

    let mut arena = Arena::default_standard();
    arena.pin_mut().set_ball(BallState {
        pos: Vec3::new(0., 5000., 100.),
        vel: Vec3::new(0., 2000., 0.),
        ..Default::default()
    });

    arena.pin_mut().set_goal_scored_callback(
        |arena, team, _| {
            assert_eq!(arena.get_tick_count(), 12);
            arena.reset_to_random_kickoff(None);
            println!("GOAL SCORED BY {team:?}!");
            SCORED.store(true, Ordering::Relaxed);
        },
        0,
    );

    arena.pin_mut().step(15);
    assert!(SCORED.load(Ordering::Relaxed));
}

#[test]
fn demoed() {
    static DEMOED: AtomicBool = AtomicBool::new(false);
    INIT.call_once(|| init(None));

    let mut arena = Arena::default_standard();
    // set up two cars, one demoing the other
    let orange = arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout());
    let blue = arena.pin_mut().add_car(Team::BLUE, CarConfig::hybrid());

    arena
        .pin_mut()
        .set_car(
            orange,
            CarState {
                pos: Vec3::new(0., 0., 17.),
                ..Default::default()
            },
        )
        .unwrap();

    arena
        .pin_mut()
        .set_car(
            blue,
            CarState {
                pos: Vec3::new(-300., 0., 17.),
                vel: Vec3::new(2300., 0., 0.),
                boost: 100.,
                ..Default::default()
            },
        )
        .unwrap();

    arena
        .pin_mut()
        .set_car_controls(
            blue,
            CarControls {
                throttle: 1.,
                boost: true,
                ..Default::default()
            },
        )
        .unwrap();

    arena.pin_mut().set_car_bump_callback(
        |arena, bumper, victim, is_demo, _| {
            if is_demo {
                assert_eq!(arena.get_tick_count(), 9);
                println!("CAR {bumper} DEMOED {victim}!");
                DEMOED.store(true, Ordering::Relaxed);
            }
        },
        0,
    );

    arena.pin_mut().step(15);
    assert!(DEMOED.load(Ordering::Relaxed));
}
