use std::time::Instant;

use autocxx::{c_int, WithinUniquePtr};
use rocketsim_rs::sim::arena::{Arena, GameMode};

fn main() {
    const NUM_CPU: u8 = 24;
    const TICKS: i32 = 500000;

    let start_time = Instant::now();
    let threads = (0..NUM_CPU).map(|_| {
        std::thread::spawn(|| {
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
        NUM_CPU as f32 * TICKS as f32 / 120. / 60. / 60.,
        ms_elapsed as f32 / 1000.
    );
    println!("FPS: {}", NUM_CPU as f32 * TICKS as f32 / ms_elapsed as f32 * 1000.0);
}
