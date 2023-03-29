use std::{sync::Mutex, time::Instant};

use rand::Rng;

use rocketsim_rs::sim::{
    arena::Arena,
    car::{CarConfig, Team},
    CarControls,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub goals: u16,
    pub own_goals: u16,
    pub assists: u16,
    // pub saves: u16,
    // pub shots: u16,
    // pub demolitions: u16,
}

const TICK_SKIP: i32 = 8;

fn main() {
    // see the comment above `set_goal_scored_callback` for why we need Mutexes
    static BALL_TOUCHES: Mutex<[Vec<u32>; 2]> = Mutex::new([Vec::new(), Vec::new()]);
    static STATS: Mutex<Vec<(u32, Stats)>> = Mutex::new(Vec::new());
    static SCORE: Mutex<[u16; 2]> = Mutex::new([0; 2]);

    // Load in the Rocket League assets from the collision_meshes folder in the current directory
    rocketsim_rs::init(None);

    // Create a new arena with gamemode soccar and a tick rate of 120
    let mut arena = Arena::default_standard();

    // spawn a orange team breakout hitbox car
    arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout());
    arena.pin_mut().add_car(Team::ORANGE, CarConfig::octane());
    arena.pin_mut().add_car(Team::ORANGE, CarConfig::plank());

    // spawn a blue team hybrid hibox car
    arena.pin_mut().add_car(Team::BLUE, CarConfig::hybrid());
    arena.pin_mut().add_car(Team::BLUE, CarConfig::dominus());
    arena.pin_mut().add_car(Team::BLUE, CarConfig::merc());

    // Add a new default stats entry for each car
    STATS.lock().unwrap().extend(arena.pin_mut().get_cars().iter().map(|(id, _, _, _)| (*id, Stats::default())));

    // set kickoff with random seed
    arena.pin_mut().reset_to_random_kickoff(None);

    // note that this actually takes an `fn` type, not a closure
    // this means that the closure can't capture any variables
    // this is why our stats are stored in static Mutexes
    arena.pin_mut().set_goal_scored_callback(
        |mut arena, team, _| {
            println!("Goal scored by {:?}", team);

            // update stats
            let t_index = team as u8 as usize;

            // record the scored goal
            SCORE.lock().unwrap()[t_index] += 1;

            let ball_touches = BALL_TOUCHES.lock().unwrap().clone();
            let mut stats = STATS.lock().unwrap();
            let mut scorer = None;

            // it's possible no car touched the ball on the team that got the goal
            // so ensure that were was at least one ball touch
            if !ball_touches[t_index].is_empty() {
                // the latest ball touch on the same team is the scorer
                scorer = ball_touches[t_index].last().copied();
                // mark up the scorer's stats
                stats.iter_mut().find(|(id, _)| Some(*id) == scorer).unwrap().1.goals += 1;

                if ball_touches[t_index].len() > 1 {
                    // if there were two ball touches, they get the assist
                    let assist = ball_touches[t_index][ball_touches[t_index].len() - 2];
                    stats.iter_mut().find(|(id, _)| id == &assist).unwrap().1.assists += 1;
                }
            }

            let last_hit_id = arena.as_mut().get_ball().hit_info.car_id;
            if last_hit_id != 0 && Some(last_hit_id) != scorer {
                // if the last hit was not the scorer, they get the own goal
                stats.iter_mut().find(|(id, _)| *id == last_hit_id).unwrap().1.own_goals += 1;
            }

            // reset to a random kickoff to continue the game
            // this also reset info like last ball touch
            // so it needs to be done last
            arena.reset_to_random_kickoff(None);
        },
        0,
    );

    let mut random = rand::thread_rng();

    // run the simulation for 20 minutes (18000 * 8 = 144,000 ticks with 120 ticks per second and 60 seconds per minute)
    let sim_rounds = 18000;

    println!("Simulating {} minutes", sim_rounds * TICK_SKIP / 120 / 60);
    let start_time = Instant::now();

    for _ in 0..sim_rounds {
        #[cfg(not(feature = "glam"))]
        // Get the game state
        let game_state = arena.pin_mut().get_game_state();

        #[cfg(feature = "glam")]
        let game_state = arena.pin_mut().get_game_state().to_glam();

        // for tracking the ball touches
        if game_state.tick_count != 0 && game_state.ball.hit_info.tick_count_when_hit == game_state.tick_count - 1 {
            // get info on the car that hit the ball
            let (id, team, _, _) = game_state.cars.iter().find(|&car| car.0 == game_state.ball.hit_info.car_id).unwrap();

            let mut ball_touches = BALL_TOUCHES.lock().unwrap();
            let t_index = *team as u8 as usize;
            // add the car id to the list of cars that have touched the ball on it's team
            ball_touches[t_index].push(*id);

            // if there are more than 2 cars that have touched the ball on the team
            if ball_touches.len() > 2 {
                // remove the oldest car id
                ball_touches[t_index].remove(0);
            }
        }

        let mut all_controls = Vec::new();

        for (car_id, _team, _state, _config) in game_state.cars {
            // Randomize the controls
            let controls = CarControls {
                throttle: random.gen_range(0.1..1.0),
                steer: random.gen_range(-0.5..0.5),
                boost: random.gen_bool(0.75),
                ..Default::default()
            };

            // rocketsim wants car_id/control pairs so it knows which car to apply the controls to
            all_controls.push((car_id, controls));
        }

        // set all the controls
        // returns an error if any of the car_ids are invalid
        arena.pin_mut().set_all_controls(&all_controls).unwrap();

        // Advance the simulation by TICK_SKIP
        arena.pin_mut().step(TICK_SKIP);
    }

    println!("Simulation complete in {:.2} seconds", start_time.elapsed().as_secs_f32());

    let stats = STATS.lock().unwrap();
    let score = SCORE.lock().unwrap();

    println!("Score: {} - {}", score[0], score[1]);

    for (id, stats) in stats.iter() {
        println!("Car {} stats: {:?}", id, stats);
    }
}
