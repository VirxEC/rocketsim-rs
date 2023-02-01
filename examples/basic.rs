use autocxx::{c_int, WithinBox, WithinUniquePtr};
use cxx::UniquePtr;
use rocketsim_rs::sim::{
    arena::{Arena, GameMode},
    car::{CarConfig, Team},
};

fn main() {
    let mut arena = Arena::new(GameMode::SOCCAR, 120.).within_unique_ptr();
    println!("Arena tick rate: {}", arena.pin_mut().GetTickRate());

    let car = arena
        .pin_mut()
        .AddCar(UniquePtr::new(Team::BLUE), CarConfig::octane());

    // illusion of safety car
    // from docs:
    // This function is unsafe because improper use may lead to memory problems. For example a double-free may occur if the function is called twice on the same raw pointer.
    let mut iosc = unsafe { UniquePtr::from_raw(car) };

    // get the state of the car
    let mut state = iosc.pin_mut().GetState().within_box();

    // set the X position of the car to 5
    dbg!(&state.pos);
    state.pos.pin_mut().setX(5.);
    dbg!(&state.pos);

    // set the velocity of the car to 500, 900, 0
    dbg!(&state.vel);
    state.vel.pin_mut().setX(500.);
    state.vel.pin_mut().setY(900.);
    dbg!(&state.vel);

    // set the state of the car
    iosc.pin_mut().SetState(&state);

    // simulate for 5 seconds
    arena.pin_mut().Step(c_int(120 * 5))
}
