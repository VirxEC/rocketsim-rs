use std::{fs, io};

use byteorder::{LittleEndian, WriteBytesExt};
use rocketsim_rs::sim::{Arena, BallState};

const NUM_TICKS: u16 = 120 * 12;

fn write_ball(file: &mut fs::File, ball: BallState, time: f32) -> io::Result<()> {
    file.write_f32::<LittleEndian>(time)?;
    file.write_f32::<LittleEndian>(ball.pos.x)?;
    file.write_f32::<LittleEndian>(ball.pos.y)?;
    file.write_f32::<LittleEndian>(ball.pos.z)?;
    file.write_f32::<LittleEndian>(ball.vel.x)?;
    file.write_f32::<LittleEndian>(ball.vel.y)?;
    file.write_f32::<LittleEndian>(ball.vel.z)?;
    file.write_f32::<LittleEndian>(ball.ang_vel.x)?;
    file.write_f32::<LittleEndian>(ball.ang_vel.y)?;
    file.write_f32::<LittleEndian>(ball.ang_vel.z)?;
    Ok(())
}

fn main() -> io::Result<()> {
    rocketsim_rs::init(None);

    let mut arena = Arena::default_standard();
    let mut ball = arena.pin_mut().get_ball();
    ball.pos.z = 1800.;
    ball.vel.x = 1000.;
    ball.vel.y = 1000.;
    ball.vel.z = 650.;

    arena.pin_mut().set_ball(ball);

    let mut file = fs::File::create("examples/ball.dump")?;
    file.write_u16::<LittleEndian>(1 + NUM_TICKS)?;
    write_ball(&mut file, ball, 0.)?;

    for _ in 0..NUM_TICKS {
        arena.pin_mut().step(1);
        let ball = arena.pin_mut().get_ball();
        write_ball(&mut file, ball, arena.get_tick_count() as f32 / arena.get_tick_rate())?;
    }

    Ok(())
}
