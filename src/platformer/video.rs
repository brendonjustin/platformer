use sdl2;

use extra;

use camera;
use render;
use world;

pub fn main(io_chan: Chan<int>, world_state_client: extra::comm::DuplexStream<(), extra::arc::Arc<world::WorldState>>) {
    sdl2::init([sdl2::InitVideo]);

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, [sdl2::video::OpenGL]) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

    sdl2::video::gl_set_attribute(sdl2::video::GLDoubleBuffer, 1);

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, [sdl2::render::Accelerated]) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    renderer.set_draw_color(sdl2::pixels::RGB(255, 0, 0));
    renderer.clear();
    renderer.present();

    let camera = camera::Camera::new(camera::Point::new(0.0,0.0), camera::Size::new(10.0,10.0));

    'main : loop {
        'event : loop {
            match sdl2::event::poll_event() {
                sdl2::event::NoEvent => break 'event,
                sdl2::event::QuitEvent(_) => break 'main,
                sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                    if key == sdl2::keycode::EscapeKey {
                        io_chan.send(-1)
                    }
                },
                _ => {}
            }
        }

        world_state_client.send(());
        let world_state = world_state_client.recv();
        render::render_world(renderer, camera, world_state.unwrap());

        renderer.present();
    }

    sdl2::quit();
}
