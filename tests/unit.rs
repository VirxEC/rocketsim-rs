use std::sync::Once;

use rocketsim_rs::{
    init,
    sim::{
        arena::Arena,
        ball::Ball,
        car::{CarConfig, Team},
        math::*,
        CarControls,
    },
};

#[cfg(feature = "glam")]
use rocketsim_rs::GameStateA;

static INIT: Once = Once::new();

#[test]
fn pads() {
    INIT.call_once(init);
    let arena = Arena::default_standard();

    let statics = arena.iter_pad_static().collect::<Vec<_>>();
    assert!(statics.len() == arena.num_pads());

    let states = arena.iter_pad_state().collect::<Vec<_>>();
    assert!(states.len() == arena.num_pads());
}

#[test]
fn cars() {
    INIT.call_once(init);
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

    let cars = arena.pin_mut().get_cars();
    assert!(cars.len() == 1);

    let (_, team, car, car_config) = cars[0];

    assert!(team == Team::ORANGE);
    assert!(car.boost < 100. / 3.);

    // this differs the most between cars so we'll just this
    assert!(car_config.hitbox_size.x == dominus.hitbox_size.x);
    assert!(car_config.hitbox_size.y == dominus.hitbox_size.y);
    assert!(car_config.hitbox_size.z == dominus.hitbox_size.z);
}

#[test]
fn ball() {
    INIT.call_once(init);
    let mut arena = Arena::default_standard();

    arena.pin_mut().set_ball(Ball {
        pos: Vec3::new(1., 2., 1000.),
        vel: Vec3::new(0., 0., -1.),
        ..Default::default()
    });

    let ball = arena.pin_mut().get_ball();
    assert!(ball.pos.x == 1.);
    assert!(ball.pos.y == 2.);
    assert!(ball.pos.z == 1000.);
    assert!(ball.vel.x == 0.);
    assert!(ball.vel.y == 0.);
    assert!(ball.vel.z == -1.);
    assert!(ball.ang_vel.x == 0.);
    assert!(ball.ang_vel.y == 0.);
    assert!(ball.ang_vel.z == 0.);

    arena.pin_mut().step(30);

    let ball = arena.pin_mut().get_ball();
    assert!(ball.pos.x == 1.);
    assert!(ball.pos.y == 2.);
    assert!(ball.pos.z < 1000.);
    assert!(ball.vel.x == 0.);
    assert!(ball.vel.y == 0.);
    assert!(ball.vel.z < 0.);
}

#[test]
fn game_state() {
    INIT.call_once(init);
    let mut arena = Arena::default_standard();
    arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout());
    arena.pin_mut().add_car(Team::BLUE, CarConfig::hybrid());
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
}

#[cfg(feature = "rlbot")]
#[test]
fn rlbot() {
    INIT.call_once(init);
    let mut arena = Arena::default_standard();
    arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout());
    arena.pin_mut().add_car(Team::BLUE, CarConfig::hybrid());
    arena.pin_mut().step(120);

    let game_tick_packet = arena.pin_mut().get_game_tick_packet();

    assert_eq!(game_tick_packet.game_cars.len(), game_tick_packet.num_cars);
    assert_eq!(game_tick_packet.num_cars, 2);

    assert_eq!(game_tick_packet.game_boosts.len(), game_tick_packet.num_boosts);
    assert_eq!(game_tick_packet.num_boosts, 34);

    assert_eq!(game_tick_packet.game_ball.collision_shape.type_, 1);
    // this is actually incorrect because of a bug in rocketsim
    // assert_eq!(game_tick_packet.game_ball.collision_shape.sphere.diameter, 91.25 * 2.);

    assert!(game_tick_packet.game_info.seconds_elapsed - 1. < 0.00001);
    assert_eq!(game_tick_packet.game_info.frame_num, 120);
    assert_eq!(game_tick_packet.game_info.world_gravity_z, -650.);
}
