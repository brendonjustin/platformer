use sdl2;

use camera;
use player;
use world;

pub fn render_world(renderer: &sdl2::render::Renderer, camera: &camera::Camera, world_state: &world::WorldState) {
	let r: u8 =
	if world_state.ticks < 300 {
		255
	}
	else {
		100
	};

	renderer.set_draw_color(sdl2::pixels::RGB(r, 0, 0));
	renderer.clear();

	// render the player. no blit function as of yet, so this is speculative.
	// world_state.player_state.sprite.blit(None, match renderer.parent {
	// 	Left(window) => window,
	// 	Right(surface) => surface
	// }, None);
}