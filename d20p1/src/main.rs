use std::collections::HashSet;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
}

impl Coordinate {
	fn get_relevant_coords(&self) -> Vec<Self> {
		let mut result: Vec<Self> = Vec::with_capacity(9);
		for y_mod in -1..=1 {
			for x_mod in -1..=1 {
				let x = self.x + x_mod;
				let y = self.y + y_mod;
				result.push(Self { x, y });
			}
		}
		result
	}
}

fn main() {
	let (image_enhancement, mut image_light_set) = {
		let input: Vec<String> = fs::read_to_string("input.txt")
			.unwrap()
			.lines()
			.filter(|s| !s.is_empty())
			.map(String::from)
			.collect();
		let mut input_lines = input.iter();
		let image_enhancement: Vec<bool> = input_lines.next().unwrap().chars().map(|c| c == '#').collect();

		let mut image_light_set: HashSet<Coordinate> = HashSet::new();
		for (row_index, row_data) in input_lines.enumerate() {
			for (char_index, pixel) in row_data.chars().enumerate() {
				if pixel == '#' {
					let coordinate = Coordinate {
						x: char_index as i32,
						y: row_index as i32,
					};
					image_light_set.insert(coordinate);
				}
			}
		}
		(image_enhancement, image_light_set)
	};

	assert!(image_enhancement.len() == 512, "Input does not match puzzle guarantee");

	let mut light_set_contains_lit = true;

	for _ in 0..2 {
		let mut new_image_light_set = HashSet::new();
		let new_light_set_contains_lit = if image_enhancement[0] {
			!light_set_contains_lit
		} else {
			true
		};
		for coord in image_light_set.iter() {
			let check_coordinates = coord.get_relevant_coords();
			for check_coord in check_coordinates.iter() {
				let near_data: Vec<bool> = check_coord
					.get_relevant_coords()
					.iter()
					.map(|coord| image_light_set.contains(coord))
					.collect();
				let mut coord_value: usize = 0;
				for near_data_value in near_data {
					coord_value *= 2;
					if near_data_value == light_set_contains_lit {
						coord_value += 1;
					}
				}
				if image_enhancement[coord_value] == new_light_set_contains_lit {
					new_image_light_set.insert(check_coord.clone());
				}
			}
		}
		image_light_set = new_image_light_set;
		light_set_contains_lit = new_light_set_contains_lit;
	}

	println!("{}", image_light_set.len());
}
