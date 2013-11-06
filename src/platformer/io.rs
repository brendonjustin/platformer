use sdl2;
use std;

fn keypress_loop(keypress_chan: Chan<int>) {
    'keypress_events : loop {
        match sdl2::event::poll_event() {
            sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                if key == sdl2::keycode::EscapeKey {
                    keypress_chan.send(-1)
                }
            }
            _ => {}
        }
    }
}

pub fn io_handler(io_chan: Chan<int>) {
    let (port, chan): (Port<int>, Chan<int>) = std::comm::stream();

    let chan_cell = std::cell::Cell::new(chan);
    do spawn {
        keypress_loop(chan_cell.take());
    }

    'io : loop {
        let keypress = port.recv();
        io_chan.send(keypress);
    }
}