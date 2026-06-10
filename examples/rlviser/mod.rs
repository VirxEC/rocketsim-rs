use rocketsim_rs::{
    cxx::UniquePtr,
    flat_ext::{PACKET_SIZE_BYTES, PacketCodec, RlviserMessage},
    sim::{Arena, GameMode},
};
use std::{
    io,
    net::{IpAddr, SocketAddr, UdpSocket},
    str::FromStr,
    sync::mpsc::{Receiver, channel},
    thread::sleep,
    time::{Duration, Instant},
};

pub const RLVISER_PORT: u16 = 45243;
pub const ROCKETSIM_PORT: u16 = 34254;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = channel();

    ctrlc::set_handler(move || {
        sender.send(()).unwrap();
    })?;

    Ok(receiver)
}

pub struct RLViserSocketHandler {
    socket: UdpSocket,
    rlviser_addr: SocketAddr,
    packet_size_buffer: [u8; PACKET_SIZE_BYTES],
    packet_buffer: Vec<u8>,
    codec: PacketCodec,
    paused: bool,
}

impl RLViserSocketHandler {
    pub fn new() -> io::Result<Self> {
        let socket = UdpSocket::bind(("0.0.0.0", ROCKETSIM_PORT))?;
        println!("Listening on {}", socket.local_addr()?);

        let rlviser_addr = SocketAddr::new(IpAddr::from_str("127.0.0.1").unwrap(), RLVISER_PORT);

        println!("\nPress enter to start...");
        io::stdin().read_line(&mut String::new())?;

        socket.set_nonblocking(true)?;

        let mut handler = Self {
            socket,
            rlviser_addr,
            packet_size_buffer: [0; PACKET_SIZE_BYTES],
            packet_buffer: Vec::with_capacity(1024),
            codec: PacketCodec::new(),
            paused: false,
        };
        handler.send_message(RlviserMessage::Connection)?;

        Ok(handler)
    }

    pub fn run(&mut self, arena_type: GameMode, setup_arena: impl FnOnce(GameMode) -> UniquePtr<Arena>) -> io::Result<()> {
        let mut arena = setup_arena(arena_type);
        let break_signal = ctrl_channel().unwrap();

        let mut interval = Duration::from_secs_f32(1. / 120.);
        let mut next_time = Instant::now() + interval;

        loop {
            if break_signal.try_recv().is_ok() {
                self.send_message(RlviserMessage::Quit)?;
                println!("Sent quit signal to rlviser");
                break Ok(());
            }

            self.handle_return_messages(&mut arena, &mut interval)?;

            if !self.paused {
                arena.pin_mut().step(1);
                self.send_message(RlviserMessage::GameState(Box::new(arena.pin_mut().get_game_state())))?;
            }

            let wait_time = next_time - Instant::now();
            if wait_time > Duration::default() {
                sleep(wait_time);
            }
            next_time += interval;
        }
    }

    fn send_message(&mut self, message: RlviserMessage) -> io::Result<()> {
        self.socket.send_to(self.codec.encode(message), self.rlviser_addr)?;
        Ok(())
    }

    fn handle_return_messages(&mut self, arena: &mut UniquePtr<Arena>, interval: &mut Duration) -> io::Result<()> {
        while self.socket.peek_from(&mut self.packet_size_buffer).is_ok() {
            let packet_size = PacketCodec::packet_len_from_header(self.packet_size_buffer);
            self.packet_buffer.resize(packet_size, 0);
            let (_, src) = self.socket.recv_from(&mut self.packet_buffer)?;

            let Ok(Some(message)) = PacketCodec::decode_payload(&self.packet_buffer[PACKET_SIZE_BYTES..]) else {
                continue;
            };

            match message {
                RlviserMessage::GameState(game_state) => {
                    if let Err(e) = arena.pin_mut().set_game_state(&game_state) {
                        println!("Error setting game state: {e}");
                    };
                }
                RlviserMessage::Connection => {
                    println!("Connection established to {src}");
                }
                RlviserMessage::Speed(speed) => {
                    *interval = Duration::from_secs_f32(1. / (120. * speed));
                }
                RlviserMessage::Paused(paused) => {
                    self.paused = paused;
                }
                RlviserMessage::Quit => panic!("We shouldn't be receiving packets of this type"),
            }
        }

        Ok(())
    }
}
