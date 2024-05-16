use rocketsim_rs::{math::Vec3, sim::Arena};
use std::time::{Duration, Instant};

fn main() {
    const RUNS: usize = 10000;
    const SECONDS_PER_RUN: u32 = 8;

    rocketsim_rs::init(None);

    let mut arena = Arena::default_standard();
    let mut times = Vec::with_capacity(RUNS);

    for _ in 0..RUNS {
        let mut ball_state = arena.pin_mut().get_ball();
        ball_state.pos = Vec3::new(0., 0., 1.1 * 91.25);
        ball_state.vel = Vec3::new(600., 1550., 0.);
        ball_state.ang_vel = Vec3::new(0., 0., 0.);
        arena.pin_mut().set_ball(ball_state);

        let time_start = Instant::now();
        arena.pin_mut().step(SECONDS_PER_RUN * 120);
        times.push(Instant::now() - time_start);
    }

    let total_time = times.iter().sum::<Duration>();
    println!(
        "Simulated {} seconds in {}µs",
        SECONDS_PER_RUN,
        total_time.as_micros() / RUNS as u128
    );

    times.sort();
    let num_fastest = times.len() / 1000;
    let fastest_times = &times[..num_fastest];
    let total_time = fastest_times.iter().sum::<Duration>();
    println!(
        "(Fastest 0.1%) Simulated {} seconds in {}µs",
        SECONDS_PER_RUN,
        total_time.as_micros() / num_fastest as u128
    );
}
