use std::collections::HashSet;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

enum FoldDirection {
	Horizontal,
	Vertical,
}

struct FoldInstruction {
	direction: FoldDirection,
	position: u32,
}

fn main() {
	let (mut coordinates, folds) = {
		let input: Vec<String> = fs::read_to_string("input.txt")
			.unwrap()
			.lines()
			.filter(|s| !s.is_empty())
			.map(String::from)
			.collect();
		let mut coordinates: HashSet<Coordinate> = HashSet::new();
		let mut fold_instructions: Vec<FoldInstruction> = Vec::new();
		for line in input {
			if let Some(fold) = line.strip_prefix("fold along ") {
				let mut fold = fold.split('=');
				let direction = fold.next().unwrap();
				let position: u32 = fold.next().unwrap().parse().unwrap();
				let direction = match direction {
					"x" => FoldDirection::Vertical,
					"y" => FoldDirection::Horizontal,
					_ => panic!("Invalid axis"),
				};
				fold_instructions.push(FoldInstruction { direction, position });
			} else {
				let mut parts = line.split(',');
				let x: u32 = parts.next().unwrap().parse().unwrap();
				let y: u32 = parts.next().unwrap().parse().unwrap();
				coordinates.insert(Coordinate { x, y });
			}
		}
		(coordinates, fold_instructions)
	};

	for fold_instruction in folds.iter() {
		let mut new_coordinates = HashSet::new();
		for coordinate in coordinates.iter() {
			let new_coordinate = match fold_instruction.direction {
				FoldDirection::Horizontal => {
					if coordinate.y > fold_instruction.position {
						Coordinate {
							x: coordinate.x,
							y: coordinate.y - ((coordinate.y - fold_instruction.position) * 2),
						}
					} else {
						coordinate.clone()
					}
				}
				FoldDirection::Vertical => {
					if coordinate.x > fold_instruction.position {
						Coordinate {
							x: coordinate.x - ((coordinate.x - fold_instruction.position) * 2),
							y: coordinate.y,
						}
					} else {
						coordinate.clone()
					}
				}
			};
			new_coordinates.insert(new_coordinate);
		}
		coordinates = new_coordinates;
	}

	let mut coordinates_iter = coordinates.iter();
	let first_coordinate = coordinates_iter.next().unwrap();
	let mut min_x = first_coordinate.x;
	let mut max_x = first_coordinate.x;
	let mut min_y = first_coordinate.y;
	let mut max_y = first_coordinate.y;
	for coordinate in coordinates_iter {
		if coordinate.x < min_x {
			min_x = coordinate.x;
		}
		if coordinate.x > max_x {
			max_x = coordinate.x;
		}
		if coordinate.y < min_y {
			min_y = coordinate.y;
		}
		if coordinate.y > max_y {
			max_y = coordinate.y;
		}
	}
	for y in min_y..=max_y {
		for x in min_x..=max_x {
			let coord = Coordinate { x, y };
			if coordinates.contains(&coord) {
				print!("#");
			} else {
				print!(" ");
			}
		}
		println!();
	}
}
