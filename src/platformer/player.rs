use sdl2;

use geometry;

struct Player {
	size: geometry::Size,
}

struct PlayerState {
	player: ~Player,
	position: geometry::Point,
	sprite: ~sdl2::surface::Surface,
	sprite_path: ~str,
}

impl Player {
	pub fn new(size: geometry::Size) -> Player {
		Player{size: size}
	}

	pub fn clone(&self) -> ~Player {
		~Player{size: self.size}
	}
}

impl PlayerState {
	pub fn new(player: ~Player, position: geometry::Point, sprite_path: ~str) -> PlayerState {
        let sprite = match sdl2::surface::Surface::from_bmp(~Path::new(~"./player.bmp")) {
        	Ok(loaded_sprite) => loaded_sprite,
        	Err(_) => fail!(format!("Failed to load player sprite at path: {}", sprite_path))
        };

		PlayerState{player: player, position: position, sprite: sprite, sprite_path: sprite_path}
	}
}

impl Clone for PlayerState {
	fn clone(&self) -> PlayerState {
        let sprite = sdl2::surface::Surface::from_bmp(~Path::new(~"./player.bmp")).unwrap();
		PlayerState{player: self.player.clone(), position: self.position, sprite: sprite, sprite_path: self.sprite_path.clone()}
	}
}