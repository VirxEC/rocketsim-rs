use rocketsim_rs::sim::Arena;
use std::{
    thread::{available_parallelism, spawn},
    time::Instant,
};

fn main() {
    const TICKS: i32 = 600_000;

    rocketsim_rs::init(None);

    let num_cpu = available_parallelism().unwrap().get();

    println!("Running on {num_cpu} threads");

    let start_time = Instant::now();
    let threads = (0..num_cpu)
        .map(|_| spawn(|| Arena::default_standard().pin_mut().step(TICKS)))
        .collect::<Vec<_>>();

    threads.into_iter().for_each(|thread| thread.join().unwrap());

    let elapsed = start_time.elapsed().as_secs_f32();
    let simulated_ticks = num_cpu as f32 * TICKS as f32;

    println!(
        "Simulated {:.2} hours in {:.3} seconds",
        simulated_ticks / 120. / 60. / 60.,
        elapsed
    );

    println!("FPS: {}", simulated_ticks / elapsed);
}
