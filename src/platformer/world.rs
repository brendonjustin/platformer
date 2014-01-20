use std;
use extra::comm::DuplexStream;

use player;

pub struct WorldState {
    player_state: ~player::PlayerState,
    ticks: int,
}

impl WorldState {
    pub fn new(player_state: ~player::PlayerState) -> WorldState {
        WorldState{ticks: 0, player_state: player_state}
    }
}

impl Clone for WorldState {
    fn clone(&self) -> WorldState {
        WorldState{ticks: self.ticks, player_state: self.player_state.clone()}
    }
}

fn update_world_state(world_state: &WorldState, input: &[int]) -> WorldState {
    let mut move_x = 0 as f32;
    let mut move_y = 0 as f32;

    for val in input.iter() {
        move_x = match *val {
            1073741903 => move_x + 1.0,
            _ => move_x
        };
        move_y = match *val {
            _ => move_y
        };
    }

    WorldState{ticks: world_state.ticks + 1, player_state: ~world_state.player_state.move_by(move_x, move_y)}
}

/**
 Loop, updating the world state every time a value is sent on the timing port.
 Upon updating, the new world state is sent on the state server stream.
 */
pub fn world_handler(shared_timing_port: std::comm::Port<()>, player_state: ~player::PlayerState, world_state_server: DuplexStream<WorldState, ~[int]>) {
    let mut world_state = WorldState::new(player_state);

    'world : loop {
        shared_timing_port.recv();

        let input = world_state_server.recv();

        // update world state
        let updated_state = update_world_state(&world_state, input);
        world_state = updated_state;
        world_state_server.send(world_state.clone());
    }
}
