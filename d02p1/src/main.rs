use std::fs;
use std::str::FromStr;

enum Direction {
	Forward,
	Down,
	Up,
}

struct Instruction {
	direction: Direction,
	distance: u32,
}

impl Instruction {
	fn process(&self, position: &mut u32, depth: &mut u32) {
		match self.direction {
			Direction::Forward => *position += self.distance,
			Direction::Down => *depth += self.distance,
			Direction::Up => *depth -= self.distance,
		}
	}
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split(' ');
		let direction = parts.next().unwrap();
		let distance: u32 = parts.next().unwrap().parse().unwrap();
		let direction = match direction {
			"forward" => Direction::Forward,
			"down" => Direction::Down,
			"up" => Direction::Up,
			_ => panic!("Bad direction input"),
		};
		Ok(Self { direction, distance })
	}
}

fn main() {
	let instructions: Vec<Instruction> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(|s| s.trim().parse().unwrap())
		.collect();

	let mut horizontal_position: u32 = 0;
	let mut depth: u32 = 0;

	for instruction in instructions.iter() {
		instruction.process(&mut &mut horizontal_position, &mut depth);
	}

	let answer = horizontal_position * depth;
	println!("{}", answer);
}
