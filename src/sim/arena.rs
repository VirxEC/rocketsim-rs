#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde_utils", derive(serde::Serialize, serde::Deserialize))]
pub enum GameMode {
    #[default]
    Soccar,
    Hoops,
    Heatseeker,
    Snowday,
    Dropshot,
    TheVoid,
}

unsafe impl cxx::ExternType for GameMode {
    #[allow(unused_attributes)]
    #[doc(hidden)]
    type Id = cxx::type_id!("RocketSim::GameMode");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
mod base {
    unsafe extern "C++" {
        include!("arenar.h");

        #[rust_name = "Arena"]
        type Arenar;

        #[namespace = "RocketSim"]
        type CarState = crate::sim::CarState;
        #[namespace = "RocketSim"]
        type BallState = crate::sim::BallState;
        #[cxx_name = "EBoostPadState"]
        type BoostPadState = crate::sim::BoostPadState;
        #[namespace = "RocketSim"]
        type CarConfig = crate::sim::CarConfig;
        #[namespace = "RocketSim"]
        type CarControls = crate::sim::CarControls;
        #[namespace = "RocketSim"]
        type Team = crate::sim::Team;
        #[namespace = "RocketSim"]
        type MutatorConfig = crate::sim::MutatorConfig;
        #[namespace = "RocketSim"]
        type GameMode = crate::sim::GameMode;
        type BoostPadConfig = crate::sim::BoostPadConfig;

        #[must_use]
        #[doc(hidden)]
        #[rust_name = "rsc"]
        fn SetCar(self: Pin<&mut Arena>, car_id: u32, car: CarState) -> bool;

        #[must_use]
        #[doc(hidden)]
        #[rust_name = "rscc"]
        fn SetCarControls(self: Pin<&mut Arena>, car_id: u32, car_controls: CarControls) -> bool;

        #[must_use]
        #[doc(hidden)]
        #[rust_name = "rmvc"]
        fn RemoveCar(self: Pin<&mut Arena>, car_id: u32) -> bool;

        #[doc(hidden)]
        #[rust_name = "rtrk"]
        fn ResetToRandomKickoff(self: Pin<&mut Arena>, seed: i32);

        #[must_use]
        #[doc(hidden)]
        #[rust_name = "dc"]
        fn DemolishCar(self: Pin<&mut Arena>, car_id: u32) -> bool;

        #[must_use]
        #[doc(hidden)]
        #[rust_name = "rspc"]
        fn RespawnCar(self: Pin<&mut Arena>, car_id: u32, seed: i32, boost_amount: f32) -> bool;

        #[must_use]
        #[doc(hidden)]
        #[rust_name = "ibpgi"]
        fn IsBallProbablyGoingIn(self: &Arena, max_time: f32, extra_margin: f32) -> bool;

        /// Returns all of the car ids"
        #[must_use]
        #[cxx_name = "GetCars"]
        fn get_cars(self: &Arena) -> Vec<u32>;

        /// Returns the car state of the car with the given id
        #[must_use]
        #[cxx_name = "GetCar"]
        fn get_car(self: Pin<&mut Arena>, car_id: u32) -> CarState;

        /// Adds a car to the arena with the given team and car config
        #[must_use]
        #[cxx_name = "AddCar"]
        fn add_car(self: Pin<&mut Arena>, team: Team, car_config: &CarConfig) -> u32;

        /// Returns the ball state
        #[must_use]
        #[cxx_name = "GetBall"]
        fn get_ball(self: Pin<&mut Arena>) -> BallState;
        /// Sets the ball state

        #[cxx_name = "SetBall"]
        fn set_ball(self: Pin<&mut Arena>, ball: BallState);

        /// Returns the config of the pad with the given index
        #[must_use]
        #[cxx_name = "GetPadConfig"]
        fn get_pad_config(self: &Arena, index: usize) -> BoostPadConfig;

        /// Sets the state of the pad with the given index
        #[cxx_name = "SetPadState"]
        fn set_pad_state(self: Pin<&mut Arena>, index: usize, pad: BoostPadState);

        /// Returns the state of the pad with the given index
        #[must_use]
        #[cxx_name = "GetPadState"]
        fn get_pad_state(self: &Arena, index: usize) -> BoostPadState;

        /// Returns the car config of the car with the given id
        #[must_use]
        #[cxx_name = "GetCarConfig"]
        fn get_car_config(self: &Arena, id: u32) -> CarConfig;

        /// Returns the team of the car with the given id
        #[must_use]
        #[cxx_name = "GetCarTeam"]
        fn get_car_team(self: &Arena, id: u32) -> Team;

        /// Sets the goal scored callback
        #[cxx_name = "SetGoalScoreCallback"]
        fn set_goal_scored_callback(
            self: Pin<&mut Arena>,
            callback: fn(arena: Pin<&mut Arena>, car_team: Team, user_data: usize),
            user_data: usize,
        );

        /// Sets the car bump callback
        #[cxx_name = "SetCarBumpCallback"]
        fn set_car_bump_callback(
            self: Pin<&mut Arena>,
            callback: fn(arena: Pin<&mut Arena>, bumper: u32, victim: u32, is_demo: bool, user_data: usize),
            user_data: usize,
        );

        /// Returns the mutator config
        #[must_use]
        #[cxx_name = "GetMutatorConfig"]
        fn get_mutator_config(self: &Arena) -> MutatorConfig;

        /// Sets the mutator config
        #[cxx_name = "SetMutatorConfig"]
        fn set_mutator_config(self: Pin<&mut Arena>, config: MutatorConfig);

        /// Deep clone the arena, optionally copying the callbacks
        ///
        /// If `copy_callbacks` is true, the callbacks will be copied,
        /// otherwise the new arena will have no callbacks
        #[must_use]
        #[cxx_name = "Clone"]
        fn clone(self: &Arena, copy_callbacks: bool) -> UniquePtr<Arena>;

        /// Returns the number of cars in the arena
        #[must_use]
        #[cxx_name = "NumCars"]
        fn num_cars(self: &Arena) -> usize;

        /// Returns the radius of the ball
        #[must_use]
        #[cxx_name = "GetBallRadius"]
        fn get_ball_radius(self: &Arena) -> f32;

        /// Returns the number of pads in the arena
        #[must_use]
        #[cxx_name = "NumPads"]
        fn num_pads(self: &Arena) -> usize;

        /// Resets the tick count
        #[cxx_name = "ResetTickCount"]
        fn reset_tick_count(self: Pin<&mut Arena>);

        /// Returns the tick count
        #[must_use]
        #[cxx_name = "GetTickCount"]
        fn get_tick_count(self: &Arena) -> u64;

        /// Returns the tick rate (i.e. `0.008333` aka `1 / 120`)
        #[must_use]
        #[cxx_name = "GetTickRate"]
        fn get_tick_rate(self: &Arena) -> f32;

        /// Returns the game mode
        #[must_use]
        #[cxx_name = "GetGameMode"]
        fn get_game_mode(self: &Arena) -> GameMode;

        /// Steps the simulation by the given number of ticks
        #[cxx_name = "Step"]
        fn step(self: Pin<&mut Arena>, num_ticks: u32);

        /// Returns if the ball is within a goal
        #[must_use]
        #[cxx_name = "IsBallScored"]
        fn is_ball_scored(self: &Arena) -> bool;
    }

    impl UniquePtr<Arena> {}
}

unsafe impl Send for Arena {}

pub use base::Arena;
