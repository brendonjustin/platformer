/// X and Y position values
struct Point {
	x: f32,
	y: f32,
}

/// Height and width size values
struct Size {
	height: f32,
	width: f32,
}

/// A position and a size representing a view into an area.
struct Camera {
	position: Point,
	size: Size,
}

impl Point {
	pub fn new(x: f32, y: f32) -> Point {
		Point{x: x, y: y}
	}
}

impl Size {
	pub fn new(height: f32, width: f32) -> Size {
		Size{height: height, width: width}
	}
}

impl Camera {
	pub fn new(position: Point, size: Size) -> Camera {
		Camera{position: position, size: size}
	}
}