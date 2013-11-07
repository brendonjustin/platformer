use std;
use std::comm::SharedPort;

pub fn rendering_handler(timing_port: SharedPort<()>, render_channel: Chan<int>) {
    'render : loop {
        timing_port.recv();
        render_channel.send(0);
    }
}