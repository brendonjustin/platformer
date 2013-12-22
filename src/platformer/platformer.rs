extern mod sdl2;
extern mod extra;
extern mod native;

use std::io::Timer;
use extra::comm::DuplexStream;

mod camera;
mod geometry;
mod player;
mod render;
mod video;
mod world;


#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    let (timing_port, timing_chan): (Port<()>, Chan<()>) = Chan::new();

    let (world_state_server, world_state_client): (DuplexStream<world::WorldState, ~[int]>, DuplexStream<~[int], world::WorldState>) = DuplexStream::new();

    do spawn {
        let player = ~player::Player::new(geometry::Size::new(48.0,48.0));
        let player_pos = geometry::Point::new(10.0,10.0);
        let player_state = ~player::PlayerState::new(player, player_pos, ~"./player.bmp");
        world::world_handler(timing_port, player_state, world_state_server);
    }

    let (io_port, io_chan): (Port<int>, Chan<int>) = Chan::new();

    do spawn {
        let mut timer = Timer::new().unwrap();
        let periodic = timer.periodic(16);
        loop {
            let tick = periodic.recv();
            timing_chan.send(tick);
        }
    }

    video::main(io_chan, world_state_client);
}