use rocketsim_rs::sim::{
    arena::Arena,
    car::{CarConfig, Team},
    CarControls,
};

const TICK_SKIP: i32 = 8;

fn main() {
    // Load in the Rocket League assets from the collision_meshes folder in the current directory
    rocketsim_rs::init();

    // Create a new arena with gamemode soccar and a tick rate of 120
    let mut arena = Arena::default_standard();

    // spawn a orange team breakout hitbox car
    arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout());

    // spawn a blue team hybrid hibox car
    arena.pin_mut().add_car(Team::BLUE, CarConfig::hybrid());

    // set kickoff with random seed
    arena.pin_mut().reset_to_random_kickoff(None);

    // run the simulation for 2 seconds (30 * 8 = 240 ticks with 120 ticks per second)
    for _ in 0..30 {
        #[cfg(not(feature = "glam"))]
        // Get the game state
        let game_state = arena.pin_mut().get_game_state();

        #[cfg(feature = "glam")]
        // If you're using glam,
        // you can use the to_glam() method
        // to convert RocketSim types into Glam types
        let game_state = arena.pin_mut().get_game_state().to_glam();

        let mut all_controls = Vec::new();

        // car_id: the unique id of the car
        // team: either Team::ORANGE or TEAM::BLUE
        // state: e.x. the position, velocity, boost amount, etc.
        // config: the car config info, e.x. hitbox size, wheel base, etc.
        for (car_id, _team, _state, _config) in game_state.cars {
            // In this case we're going for kickoff
            // so we're just going to drive forwards and boost
            // (for both cars)
            let controls = CarControls {
                throttle: 1.,
                steer: 0.,
                boost: true,
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
}
