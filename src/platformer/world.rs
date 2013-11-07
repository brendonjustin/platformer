use std::comm::SharedPort;
use extra::arc::Arc;

struct WorldState {
    ticks: int,
}

impl WorldState {
    fn new() -> WorldState {
        WorldState {ticks: 0}
    }
}

fn update_world_state(world_state: &WorldState) -> WorldState {
    WorldState{ticks: world_state.ticks + 1}
}

pub fn world_handler(timing_port: SharedPort<()>, out_chan: Chan<Arc<WorldState>>) {
    let mut world_state = Arc::new(WorldState::new());

    'world : loop {
        timing_port.recv();

        // update world state
        let updated_state = update_world_state(world_state.get());
        world_state = Arc::new(updated_state);

        out_chan.send(world_state.clone());
    }
}
