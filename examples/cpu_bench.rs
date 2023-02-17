use std::{
    thread::{available_parallelism, spawn},
    time::Instant,
};

use autocxx::{c_int, WithinUniquePtr};
use rocketsim_rs::sim::arena::{Arena, GameMode};

fn main() {
    const TICKS: i32 = 200000;

    let num_cpu = available_parallelism().unwrap().get();

    let start_time = Instant::now();
    let threads = (0..num_cpu).map(|_| {
        spawn(|| {
            let mut arena = Arena::new(GameMode::SOCCAR, 120.).within_unique_ptr();
            arena.pin_mut().Step(c_int(TICKS));
        })
    });

    for thread in threads {
        thread.join().unwrap();
    }

    let ms_elapsed = start_time.elapsed().as_millis();
    println!(
        "Simulated {:.2} hours in {} seconds",
        num_cpu as f32 * TICKS as f32 / 120. / 60. / 60.,
        ms_elapsed as f32 / 1000.
    );
    println!("FPS: {}", num_cpu as f32 * TICKS as f32 / ms_elapsed as f32 * 1000.0);
}
