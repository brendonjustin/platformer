use geometry;

struct Player {
	size: geometry::Size,
}

struct PlayerState {
	player: ~Player,
	position: geometry::Point,
	sprite_path: ~str,
}

impl Player {
	pub fn new(size: geometry::Size) -> Player {
		Player{size: size}
	}
}

impl Clone for Player {
	fn clone(&self) -> Player {
		Player{size: self.size}
	}
}

impl PlayerState {
	pub fn new(player: ~Player, position: geometry::Point, sprite_path: ~str) -> PlayerState {

		PlayerState{player: player, position: position, sprite_path: sprite_path.clone()}
	}

	pub fn move_by(&self, delta_x: f32, delta_y: f32) -> PlayerState {
		self.move_to(geometry::Point::new(self.position.x + delta_x, self.position.y + delta_y))
	}

	pub fn move_to(&self, position: geometry::Point) -> PlayerState {
		PlayerState{player: self.player.clone(), position: position, sprite_path: self.sprite_path.clone()}
	}
}

impl Clone for PlayerState {
	fn clone(&self) -> PlayerState {
		PlayerState{player: self.player.clone(), position: self.position, sprite_path: self.sprite_path.clone()}
	}
}
