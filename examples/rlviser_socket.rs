mod rlviser;

use rlviser::RLViserSocketHandler;
use rocketsim_rs::{
    cxx::UniquePtr,
    math::Vec3,
    sim::{Arena, ArenaConfig, BallState, CarConfig, CarControls, GameMode, Team},
};
use std::io;

fn main() -> io::Result<()> {
    rocketsim_rs::init(None, true);

    let mut args = std::env::args();
    let _ = args.next();
    let arena_type = match args.next().as_deref() {
        Some("hoops") => GameMode::Hoops,
        Some("dropshot") => GameMode::Dropshot,
        _ => GameMode::Soccar,
    };

    RLViserSocketHandler::new()?.run(arena_type, setup_arena)?;

    Ok(())
}

fn setup_arena(arena_type: GameMode) -> UniquePtr<Arena> {
    let mut arena = Arena::new(arena_type, ArenaConfig::default(), 120);

    let _ = arena.pin_mut().add_car(Team::Blue, CarConfig::octane());
    let _ = arena.pin_mut().add_car(Team::Blue, CarConfig::dominus());
    let _ = arena.pin_mut().add_car(Team::Blue, CarConfig::merc());
    let _ = arena.pin_mut().add_car(Team::Orange, CarConfig::breakout());
    let _ = arena.pin_mut().add_car(Team::Orange, CarConfig::hybrid());
    let _ = arena.pin_mut().add_car(Team::Orange, CarConfig::plank());

    arena.pin_mut().set_ball(BallState {
        pos: Vec3::new(3236.619, 4695.641, 789.734),
        vel: Vec3::new(742.26917, 1717.2388, -1419.7668),
        ang_vel: Vec3::new(-0.2784555, 2.6806574, 0.9157419),
        ..Default::default()
    });

    arena.pin_mut().set_goal_scored_callback(
        |arena, _, _| {
            arena.reset_to_random_kickoff(None);
        },
        0,
    );

    arena
        .pin_mut()
        .set_all_controls(
            (1..=6u32)
                .map(|i| {
                    (
                        i,
                        CarControls {
                            steer: 0.2,
                            throttle: 1.,
                            pitch: -0.1,
                            boost: true,
                            ..Default::default()
                        },
                    )
                })
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .unwrap();

    arena
}
