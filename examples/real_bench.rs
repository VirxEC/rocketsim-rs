use std::{
    thread::{available_parallelism, spawn},
    time::Instant,
};

use rocketsim_rs::sim::{arena::Arena, car::{Team, CarConfig}};

fn main() {
    const TICKS: i32 = 25000;

    // load in assets
    Arena::default_soccar();

    let num_cpu = available_parallelism().unwrap().get();

    println!("Running on {num_cpu} threads");

    let start_time = Instant::now();
    let threads = (0..num_cpu).map(|_| spawn(|| {
        let mut arena = Arena::default_soccar();

        arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());
        arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());
        arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());

        arena.pin_mut().add_car(Team::ORANGE, CarConfig::octane());
        arena.pin_mut().add_car(Team::ORANGE, CarConfig::octane());
        arena.pin_mut().add_car(Team::ORANGE, CarConfig::octane());

        arena.pin_mut().step(TICKS);
    })).collect::<Vec<_>>();

    threads.into_iter().for_each(|thread| thread.join().unwrap());

    let elapsed = start_time.elapsed().as_secs_f32();
    let simulated_ticks = num_cpu as f32 * TICKS as f32;

    println!("Simulated {:.2} hours in {:.3} seconds", simulated_ticks / 120. / 60. / 60., elapsed);

    println!("FPS: {}", simulated_ticks / elapsed);
}
