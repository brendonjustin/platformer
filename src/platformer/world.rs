use std;
use extra::arc::Arc;
use extra::comm::DuplexStream;

struct WorldState {
    ticks: int,
}

impl WorldState {
    fn new() -> WorldState {
        WorldState{ticks: 0}
    }
}

fn update_world_state(world_state: &WorldState) -> WorldState {
    WorldState{ticks: world_state.ticks + 1}
}

pub fn world_handler(shared_timing_port: std::comm::SharedPort<()>, world_state_server: DuplexStream<Arc<WorldState>, ()>) {
    let mut world_state = Arc::new(WorldState::new());

    let (timing_port, timing_chan) = std::rt::comm::stream();
    do spawn {
        let shared_timing_port = shared_timing_port.clone();
        loop {
            shared_timing_port.recv();
            timing_chan.send(());
        }
    }

    'world : loop {
        if timing_port.peek() {
            timing_port.recv();

            // update world state
            let updated_state = update_world_state(world_state.get());
            world_state = Arc::new(updated_state);
        }

        if world_state_server.peek() {
            world_state_server.recv();
            world_state_server.send(world_state.clone());
        }
    }
}
