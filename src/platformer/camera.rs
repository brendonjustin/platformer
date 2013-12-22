use geometry::Point;
use geometry::Size;

/// A position and a size representing a view into an area.
pub struct Camera {
	position: Point,
	size: Size,
}

impl Camera {
	pub fn new(position: Point, size: Size) -> Camera {
		Camera{position: position, size: size}
	}
}