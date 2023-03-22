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

static INIT: Once = Once::new();

#[test]
fn pads() {
    INIT.call_once(init);
    let arena = Arena::default_soccar();

    let statics = arena.iter_pad_static().collect::<Vec<_>>();
    assert!(statics.len() == arena.num_pads());

    let states = arena.iter_pad_state().collect::<Vec<_>>();
    assert!(states.len() == arena.num_pads());
}

#[test]
fn cars() {
    INIT.call_once(init);
    let mut arena = Arena::default_soccar();

    let car_id = arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());
    assert_eq!(arena.pin_mut().get_cars().len(), 1);

    arena.pin_mut().remove_car(car_id).unwrap();
    assert!(arena.pin_mut().get_cars().is_empty());

    let car_id = arena.pin_mut().add_car(Team::ORANGE, CarConfig::dominus());

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

    assert!(arena.pin_mut().get_car(car_id).boost < 100. / 3.);
}

#[test]
fn ball() {
    INIT.call_once(init);
    let mut arena = Arena::default_soccar();

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
