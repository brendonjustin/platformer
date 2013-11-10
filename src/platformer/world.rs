use std;
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

/**
 Loop, updating the world state every time a value is sent on the timing port.
 Upon updating, the new world state is sent on the state server stream.
 */
pub fn world_handler(shared_timing_port: std::comm::SharedPort<()>, world_state_server: DuplexStream<WorldState, ()>) {
    let mut world_state = WorldState::new();

    'world : loop {
        shared_timing_port.recv();

        // update world state
        let updated_state = update_world_state(&world_state);
        world_state = updated_state;
        world_state_server.send(world_state);
    }
}
