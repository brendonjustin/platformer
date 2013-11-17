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

	let path_str = world_state.player_state.sprite_path.clone();
	let player_sprite_path = ~Path::new(path_str.clone());

	let player_sprite: ~sdl2::surface::Surface = match sdl2::surface::Surface::from_bmp(player_sprite_path) {
    	Ok(loaded_sprite) => loaded_sprite,
    	Err(_) => fail!(format!("Failed to load player sprite at path: {}", path_str))
    };

	let player_tex = match renderer.create_texture_from_surface(player_sprite) {
		Ok(texture) => texture,
		Err(msg) => fail!(msg),
	};

	let player_state = &world_state.player_state;
	let player_size = player_state.player.size;
	let player_pos = player_state.position;

	let player_rect = sdl2::rect::Rect::new(player_pos.x as i32, player_pos.y as i32,
											player_size.width as i32, player_size.height as i32);

	renderer.copy(player_tex, None, Some(player_rect));
}
