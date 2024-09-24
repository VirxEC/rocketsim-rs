#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/CarControls.h");

        type CarControls;
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct CarControls {
        pub throttle: f32,
        pub steer: f32,
        pub pitch: f32,
        pub yaw: f32,
        pub roll: f32,
        pub jump: bool,
        pub boost: bool,
        pub handbrake: bool,
    }
}

pub use base::CarControls;
