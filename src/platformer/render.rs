use std;
use std::comm::SharedPort;
use extra::arc::Arc;
use world;

pub fn rendering_handler(timing_port: SharedPort<()>, world_state_port: Port<Arc<world::WorldState>>, render_channel: Chan<int>) {
    'render : loop {
        timing_port.recv();
        render_channel.send(0);
    }
}