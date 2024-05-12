use autocxx::WithinUniquePtr;
use rocketsim_rs::{
    bytes::{FromBytes, FromBytesExact, ToBytes},
    cxx::UniquePtr,
    math::Vec3,
    sim::{Arena, ArenaConfig, BallState, CarConfig, CarControls, GameMode, Team},
    GameState,
};
use std::{
    io,
    net::{IpAddr, SocketAddr, UdpSocket},
    str::FromStr,
    sync::mpsc::{channel, Receiver},
    thread::sleep,
    time::{Duration, Instant},
};

// Pass this into rlviser as the first argument
// default: 45243
const RLVISER_PORT: u16 = 45243;

// Pass this into rlviser as the second argument
// default: 34254
const ROCKETSIM_PORT: u16 = 34254;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum UdpPacketTypes {
    Quit,
    GameState,
    Connection,
    Paused,
    Speed,
    Render,
}

impl From<u8> for UdpPacketTypes {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Quit,
            1 => Self::GameState,
            2 => Self::Connection,
            3 => Self::Paused,
            4 => Self::Speed,
            5 => Self::Render,
            _ => panic!("Invalid packet type"),
        }
    }
}

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = channel();

    // Setup Ctrl+C handler
    ctrlc::set_handler(move || {
        // Send a signal to the main thread to break the loop
        // If we can't send the signal for some reason,
        // then panic the process to shut down
        sender.send(()).unwrap();
    })?;

    Ok(receiver)
}

fn main() -> io::Result<()> {
    // Load rocketsim
    rocketsim_rs::init(None);

    let mut args = std::env::args();
    let _ = args.next();
    let arena_type = match args.next().as_deref() {
        Some("hoops") => GameMode::HOOPS,
        _ => GameMode::SOCCAR,
    };

    RLViserSocketHandler::new()?.run(arena_type)?;

    Ok(())
}

struct RLViserSocketHandler {
    socket: UdpSocket,
    rlviser_addr: SocketAddr,
    min_game_state_buf: [u8; GameState::MIN_NUM_BYTES],
    game_state_buffer: Vec<u8>,
    paused: bool,
}

impl RLViserSocketHandler {
    pub fn new() -> io::Result<Self> {
        let socket = UdpSocket::bind(("0.0.0.0", ROCKETSIM_PORT))?;
        // print the socket address
        println!("Listening on {}", socket.local_addr()?);

        let rlviser_addr = SocketAddr::new(IpAddr::from_str("127.0.0.1").unwrap(), RLVISER_PORT);

        println!("\nPress enter to start...");
        io::stdin().read_line(&mut String::new())?;

        // We now don't want to wait for anything UDP so set to non-blocking
        socket.set_nonblocking(true)?;

        // notify rlviser that we're connected
        // it will send us info on the desired game speed / if the game should be paused
        // if you choose to ignore this, at least send the right game speed / paused state back
        // otherwise things like packet interpolation will be off
        socket.send_to(&[UdpPacketTypes::Connection as u8], rlviser_addr)?;

        Ok(Self {
            socket,
            rlviser_addr,
            min_game_state_buf: [0; GameState::MIN_NUM_BYTES],
            game_state_buffer: Vec::new(),
            paused: false,
        })
    }

    pub fn run(&mut self, arena_type: GameMode) -> io::Result<()> {
        // Note: RLViser supports on-the-fly changing of the arena type
        let mut arena = setup_arena(arena_type);

        // listen for Ctrl+C signal
        let break_signal = ctrl_channel().unwrap();

        // we only want to loop at 120hz
        // speed 0.5 = half speed
        // speed 2 = double speed
        let mut interval = Duration::from_secs_f32(1. / 120.);
        let mut next_time = Instant::now() + interval;

        // we loop forever - can be broken by pressing Ctrl+C in terminal
        loop {
            if break_signal.try_recv().is_ok() {
                self.socket.send_to(&[UdpPacketTypes::Quit as u8], self.rlviser_addr)?;
                println!("Sent quit signal to rlviser");

                // Then break the loop
                break Ok(());
            }

            self.handle_return_message(&mut arena, &mut interval)?;

            if !self.paused {
                // advance the simulation by 1 tick
                arena.pin_mut().step(1);

                // send the new game state back
                let game_state = arena.pin_mut().get_game_state();

                // Send the packet type
                self.socket.send_to(&[UdpPacketTypes::GameState as u8], self.rlviser_addr)?;
                // Then send the packet
                self.socket.send_to(&game_state.to_bytes(), self.rlviser_addr)?;
            }

            // ensure we only calculate 120 steps per second
            let wait_time = next_time - Instant::now();
            if wait_time > Duration::default() {
                sleep(wait_time);
            }
            next_time += interval;
        }
    }

    fn handle_return_message(&mut self, arena: &mut UniquePtr<Arena>, interval: &mut Duration) -> io::Result<()> {
        let mut byte_buffer = [0];

        while let Ok((_, src)) = self.socket.recv_from(&mut byte_buffer) {
            let packet_type = UdpPacketTypes::from(byte_buffer[0]);

            match packet_type {
                UdpPacketTypes::GameState => {
                    self.socket.peek_from(&mut self.min_game_state_buf)?;

                    let num_bytes = GameState::get_num_bytes(&self.min_game_state_buf);
                    self.game_state_buffer.resize(num_bytes, 0);
                    self.socket.recv_from(&mut self.game_state_buffer)?;

                    // set the game state
                    let game_state = GameState::from_bytes(&self.game_state_buffer);
                    if let Err(e) = arena.pin_mut().set_game_state(&game_state) {
                        println!("Error setting game state: {e}");
                    };
                }
                UdpPacketTypes::Connection => {
                    println!("Connection established to {src}");
                }
                UdpPacketTypes::Speed => {
                    let mut speed_buffer = [0; f32::NUM_BYTES];
                    self.socket.recv_from(&mut speed_buffer)?;
                    let speed = f32::from_bytes(&speed_buffer);
                    *interval = Duration::from_secs_f32(1. / (120. * speed));
                }
                UdpPacketTypes::Paused => {
                    self.socket.recv_from(&mut byte_buffer)?;
                    self.paused = byte_buffer[0] == 1;
                }
                UdpPacketTypes::Quit | UdpPacketTypes::Render => {
                    panic!("We shouldn't be receiving packets of type {packet_type:?}")
                }
            }
        }

        Ok(())
    }
}

fn setup_arena(arena_type: GameMode) -> UniquePtr<Arena> {
    let mut arena = Arena::new(arena_type, ArenaConfig::default(), 120.).within_unique_ptr();

    let _ = arena.pin_mut().add_car(Team::BLUE, CarConfig::octane());
    let _ = arena.pin_mut().add_car(Team::BLUE, CarConfig::dominus());
    let _ = arena.pin_mut().add_car(Team::BLUE, CarConfig::merc());
    let _ = arena.pin_mut().add_car(Team::ORANGE, CarConfig::breakout());
    let _ = arena.pin_mut().add_car(Team::ORANGE, CarConfig::hybrid());
    let _ = arena.pin_mut().add_car(Team::ORANGE, CarConfig::plank());

    arena.pin_mut().set_ball(BallState {
        // pos: Vec3::new(0., -2000., 1500.),
        // vel: Vec3::new(0., 1500., 1.),
        pos: Vec3::new(3236.619, 4695.641, 789.734),
        vel: Vec3::new(742.26917, 1717.2388, -1419.7668),
        ang_vel: Vec3::new(-0.2784555, 2.6806574, 0.9157419),
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
                            steer: 0.2,
                            throttle: 1.,
                            pitch: -0.1,
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
