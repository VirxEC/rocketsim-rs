use rand::Rng;
use rocketsim_rs::sim::{Arena, CarConfig, CarControls, Team};
use std::{sync::Mutex, time::Instant};

#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub goals: u16,
    pub own_goals: u16,
    pub assists: u16,
    pub demolitions: u16,
    pub shots: u16,
    pub saves: u16,
}

const TICK_SKIP: u32 = 8;

fn main() {
    // see the comment above `set_goal_scored_callback` for why we need Mutexes
    static STATS: Mutex<Vec<(u32, Stats)>> = Mutex::new(Vec::new());
    static SCORE: Mutex<[u16; 2]> = Mutex::new([0; 2]);

    // Load in the Rocket League assets from the collision_meshes folder in the current directory
    rocketsim_rs::init(None);

    // Create a new arena with gamemode soccar and a tick rate of 120
    let mut arena = Arena::default_standard();

    // spawn the blue team is various hitboxes
    let _ = arena.pin_mut().add_car(Team::Blue, CarConfig::hybrid());
    let _ = arena.pin_mut().add_car(Team::Blue, CarConfig::dominus());
    let _ = arena.pin_mut().add_car(Team::Blue, CarConfig::merc());

    // spawn the orange team is various hitboxes
    let _ = arena.pin_mut().add_car(Team::Orange, CarConfig::breakout());
    let _ = arena.pin_mut().add_car(Team::Orange, CarConfig::octane());
    let _ = arena.pin_mut().add_car(Team::Orange, CarConfig::plank());

    // Add a new default stats entry for each car
    STATS
        .lock()
        .unwrap()
        .extend(arena.pin_mut().get_cars().iter().map(|&id| (id, Stats::default())));

    // set kickoff with random seed
    arena.pin_mut().reset_to_random_kickoff(None);

    // note that this actually takes an `fn` type, not a closure
    // this means that the closure can't capture any variables
    // this is why our stats are stored in static Mutexes
    arena.pin_mut().set_goal_scored_callback(
        |mut arena, team, _| {
            println!("Goal scored by {team:?}");

            // Collect all valid ball touches
            let mut all_ball_touches = arena
                .as_mut()
                .get_car_infos()
                .into_iter()
                .filter_map(|car_info| {
                    if car_info.state.ball_hit_info.is_valid {
                        Some((car_info.id, car_info.team, car_info.state.ball_hit_info.tick_count_when_hit))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            // Sort by ball touch time
            all_ball_touches.sort_by_key(|(_, _, tick_count_when_hit)| *tick_count_when_hit);

            // Sort ball touches by team
            let ball_touches = [
                all_ball_touches
                    .iter()
                    .filter(|(_, team, _)| *team == Team::Blue)
                    .map(|(id, _, _)| *id)
                    .collect::<Vec<_>>(),
                all_ball_touches
                    .iter()
                    .filter(|(_, team, _)| *team == Team::Orange)
                    .map(|(id, _, _)| *id)
                    .collect::<Vec<_>>(),
            ];

            // update stats
            let t_index = team as u8 as usize;

            // record the scored goal
            SCORE.lock().unwrap()[t_index] += 1;

            let mut stats = STATS.lock().unwrap();

            // it's possible no car touched the ball on the team that got the goal
            // so ensure that were was at least one ball touch
            if !ball_touches[t_index].is_empty() {
                // the latest ball touch on the same team is the scorer
                let scorer = ball_touches[t_index].last().copied().unwrap();
                println!("Car {scorer} SCORED");

                // +1 to the car's goals stat
                stats.iter_mut().find(|(id, _)| *id == scorer).unwrap().1.goals += 1;

                if ball_touches[t_index].len() > 1 {
                    // if there were two ball touches, they get the assist
                    let assist = ball_touches[t_index][ball_touches[t_index].len() - 2];

                    // Get the tick count of when the scorer and assist touched the ball
                    let scorer_tick = arena.as_mut().get_car(scorer).ball_hit_info.tick_count_when_hit;
                    let assist_tick = arena.as_mut().get_car(assist).ball_hit_info.tick_count_when_hit;

                    // ensure that the assist is < 5s before the touch of the scoring player
                    if (assist_tick - scorer_tick) as f32 / arena.get_tick_rate() < 5. {
                        println!("CAR {assist} got an ASSIST");

                        // +1 to the car's assists stat
                        stats.iter_mut().find(|(id, _)| id == &assist).unwrap().1.assists += 1;
                    }
                }

                if let Some(latest_hit_id) = all_ball_touches.last().map(|(id, _, _)| *id) {
                    // if the last hit was not the scorer, they get the own goal
                    // rocket league tracks this stat in secret and isn't shown on the scoreboard
                    if latest_hit_id != scorer {
                        println!("CAR {latest_hit_id} OWN GOALED");

                        // +1 to the car's own goals stat
                        stats.iter_mut().find(|(id, _)| *id == latest_hit_id).unwrap().1.own_goals += 1;
                    }
                }
            }

            // reset to a random kickoff to continue the game
            // this also reset info like last ball touch
            // so it needs to be done last
            arena.reset_to_random_kickoff(None);
        },
        0,
    );

    arena.pin_mut().set_car_bump_callback(
        |_, bumper, victim, is_demo, _| {
            // If there was a demo (and not just a normal bump)
            if is_demo {
                println!("Car {bumper:?} DEMOED {victim:?}");
                // +1 to the bumper's demolitions stat
                STATS
                    .lock()
                    .unwrap()
                    .iter_mut()
                    .find(|(id, _)| *id == bumper)
                    .unwrap()
                    .1
                    .demolitions += 1;
            }
        },
        0,
    );

    let mut random = rand::thread_rng();

    // run the simulation for 20 minutes (18000 * 8 = 144,000 ticks with 120 ticks per second and 60 seconds per minute)
    let sim_rounds = 18000;

    println!("Simulating {} minutes\n", sim_rounds * TICK_SKIP / 120 / 60);
    let start_time = Instant::now();

    let mut prev_ball_touch_time = 0;
    let mut prev_ball_going_in = false;

    for _ in 0..sim_rounds {
        #[cfg(not(feature = "glam"))]
        // get the game state
        let game_state = arena.pin_mut().get_game_state();

        #[cfg(feature = "glam")]
        let game_state = arena.pin_mut().get_game_state().to_glam();

        // get the latest ball touch
        if let Some((car_id, tick_count_when_hit)) = game_state
            .cars
            .iter()
            .filter(|car_info| car_info.state.ball_hit_info.is_valid)
            .map(|car_info| (car_info.id, car_info.state.ball_hit_info.tick_count_when_hit))
            .max_by_key(|(_, tick_count_when_hit)| *tick_count_when_hit)
        {
            // ensure we haven't already processed this ball touch
            if tick_count_when_hit != prev_ball_touch_time {
                let ball_going_in = arena.is_ball_probably_going_in(None, None);

                // if the ball is suddenly going in
                // OR if the ball suddenly isn't going in
                if (ball_going_in && !prev_ball_going_in) || (!ball_going_in && prev_ball_going_in) {
                    if ball_going_in {
                        println!("Car {car_id:?} SHOT ON GOAL");

                        // +1 to the car's shots stat
                        STATS
                            .lock()
                            .unwrap()
                            .iter_mut()
                            .find(|(id, _)| *id == car_id)
                            .unwrap()
                            .1
                            .shots += 1;
                    } else {
                        println!("Car {car_id:?} SAVED SHOT");

                        // +1 to the car's saves stat
                        STATS
                            .lock()
                            .unwrap()
                            .iter_mut()
                            .find(|(id, _)| *id == car_id)
                            .unwrap()
                            .1
                            .saves += 1;
                    }

                    // ensure we don't process this ball touch again
                    // we want this to be in the conditional
                    // because is_ball_probably_going_in only looks 0.2 seconds into the future
                    prev_ball_touch_time = tick_count_when_hit;
                }

                // ensure we know when the ball changes between going in and not going in
                prev_ball_going_in = ball_going_in;
            }
        }

        let mut all_controls = Vec::new();

        for car_info in game_state.cars {
            // Randomize the controls
            let controls = CarControls {
                throttle: random.gen_range(0.1..1.0),
                steer: random.gen_range(-0.5..0.5),
                boost: random.gen_bool(0.75),
                ..Default::default()
            };

            // rocketsim wants car_id/control pairs so it knows which car to apply the controls to
            all_controls.push((car_info.id, controls));
        }

        // set all the controls
        // returns an error if any of the car_ids are invalid
        arena.pin_mut().set_all_controls(&all_controls).unwrap();

        // Advance the simulation by TICK_SKIP
        arena.pin_mut().step(TICK_SKIP);
    }

    println!("\nSimulation complete in {:.2} seconds", start_time.elapsed().as_secs_f32());

    let stats = STATS.lock().unwrap();
    let score = SCORE.lock().unwrap();

    println!("Score: {} - {}", score[0], score[1]);

    for (id, stats) in stats.iter() {
        println!("Car {id} stats: {stats:?}");
    }
}
