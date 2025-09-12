#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CarControls {
    pub throttle: f32,
    pub steer: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub jump: bool,
    pub boost: bool,
    pub handbrake: bool,
}

unsafe impl cxx::ExternType for CarControls {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::CarControls");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "RocketSim")]
mod base {
    unsafe extern "C++" {
        include!("Sim/CarControls.h");

        type CarControls = crate::sim::CarControls;
    }
}
