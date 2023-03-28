use rocketsim_rs::{
    math::Vec3,
    sim::{
        arena::Arena,
        car::{CarConfig, Team},
        CarControls,
    },
};

fn main() {
    // Load in the Rocket League assets from the collision_meshes folder in the current directory
    rocketsim_rs::init(None);

    // Create a new arena with gamemode soccar and a tick rate of 120
    let mut arena = Arena::default_standard();
    println!("Arena tick rate: {}", arena.get_tick_rate());

    let car_id = arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());

    println!("Car id: {car_id}");

    {
        // custom initial car state
        let mut car_state = arena.pin_mut().get_car(car_id);

        car_state.pos = Vec3::new(5., 0., 50.);
        car_state.vel = Vec3::new(500., 800., 0.);

        // for trivial Rust types, getting/setting is easier
        car_state.boost = 100.;

        println!("Created custom car state");

        arena
            .pin_mut()
            .set_car_controls(
                car_id,
                CarControls {
                    boost: true,
                    ..Default::default()
                },
            )
            .unwrap();

        // If car_id can't be found in arena than this will return Err
        arena.pin_mut().set_car(car_id, car_state).unwrap();

        println!("Set car ({car_id}) state");
    }

    {
        let mut ball_state = arena.pin_mut().get_ball();

        ball_state.pos.z = 1050.;
        ball_state.vel = Vec3::new(0., 0., 250.);

        arena.pin_mut().set_ball(ball_state);

        println!("Set ball state");
    }

    let ticks = 1800;
    let curr_time = std::time::Instant::now();

    arena.pin_mut().step(ticks);

    println!("Simulated {}s in {}ms", ticks as f32 / 120., curr_time.elapsed().as_millis());

    {
        // get the car state again
        let car_state = arena.pin_mut().get_car(car_id);

        println!("Got new car state");

        // You can debug the whole of the state
        // but it takes up a lot of space in stdout
        // dbg!(&car_state);

        println!("New car location: {}", car_state.pos);
        println!("New car boost: {}", car_state.boost);
    }

    // Cast the ball state position to a glam Vec3A
    #[cfg(feature = "glam")]
    println!("New ball location: {}", glam::Vec3A::from(arena.pin_mut().get_ball().pos));

    #[cfg(not(feature = "glam"))]
    println!("New ball location: {}", arena.pin_mut().get_ball().pos);
}
