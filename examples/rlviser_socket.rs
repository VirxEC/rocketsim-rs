use std::{
    io,
    net::UdpSocket,
    thread::sleep,
    time::{Duration, Instant},
};

use rocketsim_rs::{
    bytes::{FromBytes, ToBytes},
    sim::Arena,
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
        println!("Connection established to {}", src);
    }

    // We now don't want to wait for anything UDP so set to non-blocking
    socket.set_nonblocking(true)?;

    // Create a new arena
    let mut arena = Arena::default_standard();

    // we only want to loop at 120hz
    let interval = Duration::from_secs_f32(1. / 120.);
    let mut next_time = Instant::now() + interval;
    let mut min_state_set_buf = [0; GameState::MIN_NUM_BYTES];

    loop {
        if socket.peek_from(&mut min_state_set_buf).is_ok() {
            let num_bytes = GameState::get_num_bytes(&min_state_set_buf);
            let mut state_set_buf = vec![0; num_bytes];
            socket.recv_from(&mut state_set_buf)?;

            let game_state = GameState::from_bytes(&state_set_buf);
            arena.pin_mut().set_game_state(&game_state);
        }

        arena.pin_mut().step(1);

        let game_state = arena.pin_mut().get_game_state();

        socket.send_to(&game_state.to_bytes(), src)?;

        let wait_time = next_time - Instant::now();
        if wait_time > Duration::default() {
            sleep(wait_time);
        }
        next_time += interval;
    }
}
