use autocxx::prelude::*;
use rocketsim_rs::{
    sim::{
        arena::{Arena, GameMode},
        car::{CarConfig, Team},
    },
    Vec3,
};

fn main() {
    let mut arena = Arena::new(GameMode::SOCCAR, 120.).within_unique_ptr();
    println!("Arena tick rate: {}", arena.pin_mut().GetTickRate());

    let car_id = arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());

    println!("Car id: {car_id}");

    {
        // custom initial car state
        let mut car_state = arena.pin_mut().get_car_state_from_id(car_id).unwrap();

        car_state.pin_mut().set_pos(&Vec3::new1(&5., &0., &50.).within_unique_ptr());
        car_state.pin_mut().set_vel(&Vec3::new1(&500., &800., &0.).within_unique_ptr());

        // for trivial Rust types, getting/setting is easier
        car_state.boost = 100.;

        println!("Created custom car state");

        // If car_id can't be found in arena than this will return Err
        arena.pin_mut().set_car_state(car_id, &car_state).unwrap();

        println!("Set car ({car_id}) state");
    }

    {
        let mut ball_state = arena.pin_mut().get_ball_state();

        ball_state.pin_mut().set_pos(&Vec3::new1(&0., &0., &1050.).within_unique_ptr());
        ball_state.pin_mut().set_vel(&Vec3::new1(&0., &0., &250.).within_unique_ptr());

        arena.pin_mut().set_ball_state(&ball_state);

        println!("Set ball state");
    }

    // simulate for 5 seconds
    arena.pin_mut().Step(c_int(120 * 5));

    println!("Simulated for 5 seconds");

    {
        // get the car state again
        let car_state = arena.pin_mut().get_car_state_from_id(car_id).unwrap();

        println!("Got new car state");

        // You can debug the whole of the state
        // but it takes up a lot of space in stdout
        // dbg!(new_state)

        // Create new glam Vec3
        let glam_vec3 = car_state.pos().to_glam();
        println!("New car location: {glam_vec3}");
    }

    {
        let ball_state = arena.pin_mut().get_ball_state();

        // Create new glam SIMD-optimized Vec3A
        let glam_vec3a = ball_state.pos().to_glama();
        println!("New ball location: {glam_vec3a}")
    }
}
