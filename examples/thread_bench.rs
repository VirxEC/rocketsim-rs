use std::time::Instant;

use rocketsim_rs::sim::{Arena, CarConfig, Team};

fn main() {
    const TICKS: i32 = 50000;

    // load in assets
    rocketsim_rs::init(None);

    let mut arena = Arena::default_standard();

    let _ = arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());
    let _ = arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());
    let _ = arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());

    let _ = arena.pin_mut().add_car(Team::ORANGE, CarConfig::octane());
    let _ = arena.pin_mut().add_car(Team::ORANGE, CarConfig::octane());
    let _ = arena.pin_mut().add_car(Team::ORANGE, CarConfig::octane());

    let start_time = Instant::now();
    arena.pin_mut().step(TICKS);
    let elapsed = start_time.elapsed().as_secs_f32();

    let simulated_ticks = TICKS as f32;

    println!(
        "Simulated {:.2} hours in {:.3} seconds",
        simulated_ticks / 120. / 60. / 60.,
        elapsed
    );

    println!("FPS: {}", simulated_ticks / elapsed);
}
