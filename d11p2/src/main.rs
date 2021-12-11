use std::collections::HashSet;
use std::fs;

const ROW_WIDTH: usize = 10;

fn adjacent_positions(position: usize, num_positions: usize) -> Vec<usize> {
	let mut positions: Vec<usize> = Vec::new();
	if position % ROW_WIDTH > 0 {
		positions.push(position - 1);
	}
	if position >= ROW_WIDTH {
		positions.push(position - ROW_WIDTH);
	}
	if position % ROW_WIDTH > 0 && position >= ROW_WIDTH {
		positions.push(position - 1 - ROW_WIDTH);
	}
	if position % ROW_WIDTH < ROW_WIDTH - 1 && position >= ROW_WIDTH {
		positions.push(position - ROW_WIDTH + 1);
	}
	if position % ROW_WIDTH < ROW_WIDTH - 1 {
		positions.push(position + 1);
	}
	if position < num_positions - ROW_WIDTH {
		positions.push(position + ROW_WIDTH);
	}
	if position % ROW_WIDTH < ROW_WIDTH - 1 && position < num_positions - ROW_WIDTH {
		positions.push(position + ROW_WIDTH + 1);
	}
	if position % ROW_WIDTH > 0 && position < num_positions - ROW_WIDTH {
		positions.push(position + ROW_WIDTH - 1);
	}
	positions
}

fn main() {
	let mut octopuses: Vec<u8> = fs::read_to_string("input.txt")
		.unwrap()
		.chars()
		.filter(|c| c.is_digit(10))
		.map(|c| c as u8 - 48)
		.collect();

	let mut iterations: usize = 0;
	loop {
		let mut new_octopuses: Vec<u8> = octopuses.iter().map(|x| *x + 1).collect();
		let mut flashes: HashSet<usize> = HashSet::new();
		loop {
			let mut changes = false;
			for index in 0..new_octopuses.len() {
				if new_octopuses[index] > 9 && !flashes.contains(&index) {
					flashes.insert(index);
					changes = true;
					let positions = adjacent_positions(index, new_octopuses.len());
					for position in positions.iter() {
						new_octopuses[*position] += 1;
					}
				}
			}
			if !changes {
				break;
			}
		}
		for octopus in new_octopuses.iter_mut() {
			if *octopus > 9 {
				*octopus = 0;
			}
		}
		octopuses = new_octopuses;
		iterations += 1;
		if flashes.len() == octopuses.len() {
			break;
		}
	}

	println!("{}", iterations);
}
