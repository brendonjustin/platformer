use sdl2;

pub fn main(port: Port<int>) {
    sdl2::init([sdl2::InitVideo]);

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, [sdl2::video::OpenGL]) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, [sdl2::render::Accelerated]) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    renderer.set_draw_color(sdl2::pixels::RGB(255, 0, 0));
    renderer.clear();
    renderer.present();

    'main : loop {
        'inner : loop {
            let val = port.recv();
        }
    }

    sdl2::quit();
}
