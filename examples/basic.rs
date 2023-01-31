use autocxx::WithinUniquePtr;
use rocketsim_rs::{
    sim::arena::{Arena, GameMode},
    Vec,
};

fn main() {
    let mut arena = Arena::new(GameMode::SOCCAR, 120.).within_unique_ptr();
    println!("Arena tick rate: {}", arena.pin_mut().GetTickRate());

    let mut vec = Vec::from_array([1., 2., 3.]);
    println!("Vec as array: {:?}", vec.to_array());
    println!("x: {}", vec.getX());
    println!("y: {}", vec.getY());
    println!("z: {}", vec.getZ());

    println!("-------------");

    vec.pin_mut().setX(5.);
    println!("x: {}", vec.getX());
    println!("Vec as array: {:?}", vec.to_array());
}
