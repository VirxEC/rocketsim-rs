use std::mem::forget;

use autocxx::prelude::*;
use cxx::UniquePtr;
use rocketsim_rs::sim::{
    arena::{Arena, GameMode},
    car::{CarConfig, CarState, Team},
};

fn main() {
    let mut arena = Arena::new(GameMode::SOCCAR, 120.).within_unique_ptr();
    println!("Arena tick rate: {}", arena.pin_mut().GetTickRate());

    // custom initial car state
    let mut state = CarState::default();

    // pin_mut must be called every time because btVector3 is a C++ type
    state.pos.pin_mut().setX(5.);
    state.pos.pin_mut().setZ(50.);

    state.vel.pin_mut().setX(500.);
    state.vel.pin_mut().setY(800.);

    // for trivial Rust types, getting/setting is easier
    state.boost = 100.;

    println!("Created custom car state: {state:?}");

    let car_id = {
        let car = arena
            .pin_mut()
            .AddCar(UniquePtr::new(Team::BLUE), CarConfig::octane());

        println!("Added new car");

        // illusion of safety car
        // from docs:
        // This function is unsafe because improper use may lead to memory problems
        // For example a double-free may occur if the function is called twice on the same raw pointer
        // MUST BE DROPPED WITH forget() AND NOT DROP
        let mut iosc = unsafe { UniquePtr::from_raw(car) };

        println!("Created illusion of safety car");

        let car_id = iosc.id();

        println!("Car id: {car_id}");

        // set the state of the car
        iosc.pin_mut().SetState(&state);

        // DROP STATE
        // Idk why but it seg faults if we don't
        // drop(state);

        println!("Set state of car");

        // THIS IS HOW YOU MUST DROP THIS UNIQUEPTR
        // FAILURE TO DO SO WILL LEAD TO A SEG FAULT
        // drop(iosc) WILL ALSO SEG FAULT
        // This is only because we made it with forget()
        // Other UniquePtrs can be drop()'d fine
        forget(iosc);

        car_id
    };

    println!("Car id: {car_id}");

    // simulate for 5 seconds
    // arena.pin_mut().Step(c_int(120 * 5));

    // println!("Simulated for 5 seconds");

    // let new_state = {
    //     let car = arena.pin_mut().GetCarFromID(car_id);

    //     if car.is_null() {
    //         panic!("Couldn't find the car!");
    //     }

    //     println!("Got car from id");

    //     let mut iosc = unsafe { UniquePtr::from_raw(car) };

    //     let new_state = iosc.pin_mut().get_state();
        
    //     println!("Got state");
        
    //     forget(iosc);

    //     new_state
    // };

    // println!("thing: {:?}", unsafe { std::slice::from_raw_parts(&new_state.lastControls as *const _, 8) });

    // // let item = new_state.lastControls.pitch;
    // // dbg!(item);
    
    // forget(new_state);
}
