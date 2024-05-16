#[cxx::bridge]
mod boostpadstate {
    unsafe extern "C++" {
        include!("arenar.h");

        type EBoostPadState;
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct EBoostPadState {
        is_active: bool,
        cooldown: f32,
        cur_locked_car_id: u32,
        prev_locked_car_id: u32,
    }
}

pub use boostpadstate::EBoostPadState as BoostPadState;
