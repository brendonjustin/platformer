use std;
use std::comm::SharedPort;
use extra::arc::Arc;

use sdl2;

use camera;
use world;

pub fn render_world(renderer: &sdl2::render::Renderer, camera: camera::Camera, world_state: world::WorldState) {
	let r: u8 =
	match world_state.ticks % 8 {
		0 => 100,
		1 => 100,
		2 => 100,
		3 => 100,
		4 => 100,
		_ => 255,
	};

	renderer.set_draw_color(sdl2::pixels::RGB(r, 0, 0));
}