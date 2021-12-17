const MIN_TARGET_X: i32 = 128;
const MAX_TARGET_X: i32 = 160;
const MIN_TARGET_Y: i32 = -142;
const MAX_TARGET_Y: i32 = -88;

#[derive(Default)]
struct Coordinate {
	x: i32,
	y: i32,
}

impl Coordinate {
	fn move_by(&mut self, velocity: &Velocity) {
		self.x += velocity.x;
		self.y += velocity.y;
	}

	fn in_target(&self) -> bool {
		self.x >= MIN_TARGET_X && self.x <= MAX_TARGET_X && self.y >= MIN_TARGET_Y && self.y <= MAX_TARGET_Y
	}

	fn can_target(&self) -> bool {
		self.y > MIN_TARGET_Y && self.x < MAX_TARGET_X
	}
}

struct Velocity {
	x: i32,
	y: i32,
}

impl Velocity {
	fn advance_step(&mut self) {
		if self.x > 0 {
			self.x -= 1;
		}
		self.y -= 1;
	}
}

fn main() {
	// only works with negative Y
	let up_unit = MIN_TARGET_Y.abs() - 1;
	let max_y = (up_unit * (up_unit + 1)) / 2;
	let max_x = MAX_TARGET_X;

	let mut valid_velocities: u32 = 0;
	for x in 0..=max_x {
		for y in MIN_TARGET_Y..=max_y {
			let mut coordinate = Coordinate::default();
			let mut velocity = Velocity { x, y };
			while coordinate.can_target() {
				coordinate.move_by(&velocity);
				velocity.advance_step();
				if coordinate.in_target() {
					valid_velocities += 1;
					break;
				}
			}
		}
	}

	println!("{}", valid_velocities);
}
