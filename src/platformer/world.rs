use std::comm::SharedPort;

struct WorldState {
    ticks: int,
}

impl WorldState {
    fn new() -> WorldState {
        WorldState {ticks: 0}
    }
}

fn update_world_state(world_state: WorldState) -> WorldState {
    WorldState{ticks: world_state.ticks + 1}
}

pub fn world_handler(timing_port: SharedPort<()>, out_chan: Chan<int>) {
    let mut world_state = WorldState::new();

    'world : loop {
        timing_port.recv();

        // update world state
        world_state = update_world_state(world_state);

        out_chan.send(0);
    }
}