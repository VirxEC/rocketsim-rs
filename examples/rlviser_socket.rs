use std::{
    io,
    net::UdpSocket,
    thread::sleep,
    time::{Duration, Instant},
};

use rocketsim_rs::{
    bytes::{FromBytes, ToBytes},
    cxx::UniquePtr,
    math::Vec3,
    sim::{Arena, BallState, CarConfig, CarControls, Team},
    GameState,
};

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:34254")?;
    // print the socket address
    println!("Listening on {}", socket.local_addr()?);

    // Load rocketsim
    rocketsim_rs::init(None);

    run_socket(socket)
}

fn run_socket(socket: UdpSocket) -> io::Result<()> {
    println!("\nLaunch visualizer now, waiting for connection...");

    let mut buf = [0; 1];
    let (_, src) = socket.recv_from(&mut buf)?;

    if buf[0] == 1 {
        println!("Connection established to {src}");
    }

    // We now don't want to wait for anything UDP so set to non-blocking
    socket.set_nonblocking(true)?;

    let mut arena = setup_arena();

    // we only want to loop at 120hz
    let interval = Duration::from_secs_f32(1. / 120.);
    let mut next_time = Instant::now() + interval;
    let mut min_state_set_buf = [0; GameState::MIN_NUM_BYTES];

    // we loop forever - can be broken by pressing Ctrl+C in terminal
    loop {
        if socket.peek_from(&mut min_state_set_buf).is_ok() {
            // the socket sent data back
            // this is the other side telling us to update the game state
            handle_state_set(&min_state_set_buf, &socket, &mut arena)?;
        }

        // advance the simulation by 1 tick
        arena.pin_mut().step(1);

        // send the new game state back
        let game_state = arena.pin_mut().get_game_state();
        socket.send_to(&game_state.to_bytes(), src)?;

        // ensure we only calculate 120 steps per second
        let wait_time = next_time - Instant::now();
        if wait_time > Duration::default() {
            sleep(wait_time);
        }
        next_time += interval;
    }
}

fn handle_state_set(min_state_set_buf: &[u8], socket: &UdpSocket, arena: &mut UniquePtr<Arena>) -> io::Result<()> {
    let num_bytes = GameState::get_num_bytes(min_state_set_buf);
    let mut state_set_buf = vec![0; num_bytes];
    socket.recv_from(&mut state_set_buf)?;

    let game_state = GameState::from_bytes(&state_set_buf);
    if let Err(e) = arena.pin_mut().set_game_state(&game_state) {
        println!("Error setting game state: {e}");
    };

    Ok(())
}

fn setup_arena() -> UniquePtr<Arena> {
    let mut arena = Arena::default_standard();

    arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());
    arena.pin_mut().add_car(Team::BLUE, CarConfig::dominus());
    arena.pin_mut().add_car(Team::BLUE, CarConfig::merc());
    arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout());
    arena.pin_mut().add_car(Team::ORANGE, CarConfig::hybrid());
    arena.pin_mut().add_car(Team::ORANGE, CarConfig::plank());

    arena.pin_mut().set_ball(BallState {
        pos: Vec3::new(0., -2000., 1500.),
        vel: Vec3::new(0., 1500., 1.),
        ..Default::default()
    });

    arena.pin_mut().set_goal_scored_callback(
        |arena, _, _| {
            arena.reset_to_random_kickoff(None);
        },
        0,
    );

    arena
        .pin_mut()
        .set_all_controls(
            (1..=6u32)
                .map(|i| {
                    (
                        i,
                        CarControls {
                            throttle: 1.,
                            boost: true,
                            ..Default::default()
                        },
                    )
                })
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .unwrap();

    arena
}
