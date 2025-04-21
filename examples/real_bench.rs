use rocketsim_rs::sim::{Arena, CarConfig, CarControls, Team};
use std::{
    thread::{available_parallelism, spawn},
    time::Instant,
};

fn main() {
    const TICKS: u32 = 200_000;

    // load in assets
    rocketsim_rs::init(None, true);

    let num_cpu = available_parallelism().unwrap().get();

    println!("Running on {num_cpu} threads");

    let start_time = Instant::now();
    let threads = (0..num_cpu)
        .map(|_| {
            spawn(|| {
                let mut arena = Arena::default_standard();

                let ids = [
                    arena.pin_mut().add_car(Team::Blue, CarConfig::octane()),
                    arena.pin_mut().add_car(Team::Blue, CarConfig::octane()),
                    arena.pin_mut().add_car(Team::Blue, CarConfig::octane()),
                    arena.pin_mut().add_car(Team::Orange, CarConfig::octane()),
                    arena.pin_mut().add_car(Team::Orange, CarConfig::octane()),
                    arena.pin_mut().add_car(Team::Orange, CarConfig::octane()),
                ];

                let controls = ids
                    .into_iter()
                    .map(|id| {
                        (
                            id,
                            CarControls {
                                throttle: 1.0,
                                steer: 0.1,
                                boost: true,
                                ..Default::default()
                            },
                        )
                    })
                    .collect::<Vec<_>>();

                arena.pin_mut().reset_to_random_kickoff(Some(0));
                arena.pin_mut().set_all_controls(&controls).unwrap();

                arena.pin_mut().step(TICKS);
            })
        })
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
