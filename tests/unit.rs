use rocketsim_rs::{
    init,
    math::{Angle, RotMat, Vec3},
    sim::{Arena, BallState, CarConfig, CarControls, CarState, Team},
};
use std::{
    f32::consts::PI,
    sync::{
        atomic::{AtomicBool, Ordering},
        Once,
    },
};

#[cfg(feature = "glam")]
use rocketsim_rs::glam_ext::GameStateA;

static INIT: Once = Once::new();

#[test]
fn pads() {
    INIT.call_once(|| init(None, true));
    let arena = Arena::default_standard();

    let statics = arena.iter_pad_config().collect::<Vec<_>>();
    assert!(statics.len() == arena.num_pads());

    let states = arena.iter_pad_state().collect::<Vec<_>>();
    assert!(states.len() == arena.num_pads());
}

#[test]
fn cars() {
    INIT.call_once(|| init(None, true));
    let mut arena = Arena::default_standard();

    let car_id = arena.pin_mut().add_car(Team::Blue, CarConfig::octane());
    assert_eq!(arena.pin_mut().get_cars().len(), 1);

    arena.pin_mut().remove_car(car_id).unwrap();
    assert!(arena.pin_mut().get_cars().is_empty());

    let dominus = CarConfig::dominus();
    let car_id = arena.pin_mut().add_car(Team::Orange, dominus);

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

    assert_eq!(car_info.team, Team::Orange);
    assert!(car_info.state.boost < 100. / 3.);

    // this differs the most between cars so we'll just this
    assert_eq!(car_info.config.hitbox_size.x, dominus.hitbox_size.x);
    assert_eq!(car_info.config.hitbox_size.y, dominus.hitbox_size.y);
    assert_eq!(car_info.config.hitbox_size.z, dominus.hitbox_size.z);

    assert!(car_info.state.last_controls.boost);
    assert!(!car_info.state.last_controls.handbrake);

    assert!(car_info.state.has_flip_or_jump());
}

#[test]
fn ball() {
    INIT.call_once(|| init(None, true));
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

    assert_eq!(ball.rot_mat, RotMat::IDENTITY);
}

#[test]
fn game_state() {
    INIT.call_once(|| init(None, true));
    let mut arena = Arena::default_standard();
    let _ = arena.pin_mut().add_car(Team::Orange, CarConfig::breakout());
    let _ = arena.pin_mut().add_car(Team::Blue, CarConfig::hybrid());
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
    INIT.call_once(|| init(None, true));
    let mut arena = Arena::default_standard();
    let ids = [
        arena.pin_mut().add_car(Team::Orange, CarConfig::breakout()),
        arena.pin_mut().add_car(Team::Orange, CarConfig::breakout()),
        arena.pin_mut().add_car(Team::Orange, CarConfig::breakout()),
        arena.pin_mut().add_car(Team::Orange, CarConfig::dominus()),
        arena.pin_mut().add_car(Team::Orange, CarConfig::dominus()),
        arena.pin_mut().add_car(Team::Orange, CarConfig::dominus()),
        arena.pin_mut().add_car(Team::Orange, CarConfig::merc()),
        arena.pin_mut().add_car(Team::Orange, CarConfig::merc()),
        arena.pin_mut().add_car(Team::Orange, CarConfig::merc()),
        arena.pin_mut().add_car(Team::Blue, CarConfig::hybrid()),
        arena.pin_mut().add_car(Team::Blue, CarConfig::hybrid()),
        arena.pin_mut().add_car(Team::Blue, CarConfig::hybrid()),
        arena.pin_mut().add_car(Team::Blue, CarConfig::octane()),
        arena.pin_mut().add_car(Team::Blue, CarConfig::octane()),
        arena.pin_mut().add_car(Team::Blue, CarConfig::octane()),
        arena.pin_mut().add_car(Team::Blue, CarConfig::plank()),
        arena.pin_mut().add_car(Team::Blue, CarConfig::plank()),
        arena.pin_mut().add_car(Team::Blue, CarConfig::plank()),
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

#[test]
fn goal_score() {
    static SCORED: AtomicBool = AtomicBool::new(false);
    INIT.call_once(|| init(None, true));

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
    INIT.call_once(|| init(None, true));

    let mut arena = Arena::default_standard();
    // set up two cars, one demoing the other
    let orange = arena.pin_mut().add_car(Team::Orange, CarConfig::breakout());
    let blue = arena.pin_mut().add_car(Team::Blue, CarConfig::hybrid());

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

// #[test]
// fn demoed_hoops() {
//     static DEMOED: AtomicBool = AtomicBool::new(false);
//     INIT.call_once(|| init(None));

//     let mut arena = Arena::default_hoops();
//     // set up two cars, one demoing the other
//     let orange = arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout());
//     let blue = arena.pin_mut().add_car(Team::BLUE, CarConfig::hybrid());

//     arena
//         .pin_mut()
//         .set_car(
//             orange,
//             CarState {
//                 pos: Vec3::new(0., 0., 17.),
//                 ..Default::default()
//             },
//         )
//         .unwrap();

//     arena
//         .pin_mut()
//         .set_car(
//             blue,
//             CarState {
//                 pos: Vec3::new(-300., 0., 17.),
//                 vel: Vec3::new(2300., 0., 0.),
//                 boost: 100.,
//                 ..Default::default()
//             },
//         )
//         .unwrap();

//     arena
//         .pin_mut()
//         .set_car_controls(
//             blue,
//             CarControls {
//                 throttle: 1.,
//                 boost: true,
//                 ..Default::default()
//             },
//         )
//         .unwrap();

//     arena.pin_mut().set_car_bump_callback(
//         |arena, bumper, victim, is_demo, _| {
//             if is_demo {
//                 assert_eq!(arena.get_tick_count(), 9);
//                 println!("CAR {bumper} DEMOED {victim}!");
//                 DEMOED.store(true, Ordering::Relaxed);
//             }
//         },
//         0,
//     );

//     arena.pin_mut().step(15);
//     assert!(DEMOED.load(Ordering::Relaxed));
// }

#[cfg(feature = "serde_utils")]
#[test]
fn game_state_serialize() {
    use serde_json;

    INIT.call_once(|| init(None, true));
    let mut arena = Arena::default_standard();
    let _ = arena.pin_mut().add_car(Team::Orange, CarConfig::breakout());
    let _ = arena.pin_mut().add_car(Team::Blue, CarConfig::hybrid());
    arena.pin_mut().step(120);

    let game_state = arena.pin_mut().get_game_state();
    let json_state = serde_json::to_string(&game_state);
    match json_state {
        Ok(val) => println!("GOT JSON GAMESTATE:\n {val}"),
        Err(e) => panic!("GOT ERROR IN GAMESTATE SERIALIZE: {e}"),
    };

    let car_info = game_state.cars[0];
    let json_car_info = serde_json::to_string(&car_info);
    match json_car_info {
        Ok(val) => println!("GOT JSON CARINFO:\n {val}"),
        Err(e) => panic!("GOT ERROR IN CARINFO SERIALIZE: {e}"),
    };

    let pad_state = game_state.pads[0];
    let json_pad_state = serde_json::to_string(&pad_state);
    match json_pad_state {
        Ok(val) => println!("GOT JSON BOOSTPAD:\n {val}"),
        Err(e) => panic!("GOT ERROR IN BOOSTPAD SERIALIZE: {e}"),
    };

    arena.pin_mut().reset_tick_count();
}
