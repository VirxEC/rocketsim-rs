use std::sync::Once;

use rocketsim_rs::{
    init,
    sim::{
        arena::Arena,
        car::{Car, CarConfig, Team},
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
        .set_car(
            car_id,
            Car {
                pos: Vec3::new(1., 2., 20.),
                rotMat: RotMat::get_identity(),
                vel: Vec3::new(4., 5., 100.),
                angVel: Vec3::new(7., 8., 9.),
                isOnGround: true,
                hasJumped: true,
                hasDoubleJumped: false,
                hasFlipped: false,
                lastRelDodgeTorque: Vec3::new(0., 0., 0.),
                jumpTime: 0.1,
                flipTime: 0.,
                isJumping: true,
                airTimeSinceJump: 0.1,
                boost: 100. / 3.,
                ..Default::default()
            },
        )
        .unwrap();
    dbg!(arena.pin_mut().get_car(car_id));
    arena
        .pin_mut()
        .set_car_controls(
            car_id,
            dbg!(CarControls {
                boost: true,
                ..Default::default()
            }),
        )
        .unwrap();

    arena.pin_mut().step(1);
    dbg!(arena.pin_mut().get_car(car_id));

    assert!(dbg!(arena.pin_mut().get_car(car_id).boost) < 100. / 3.);
}
