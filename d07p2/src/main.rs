use std::fs;

fn fuel_cost(distance: u32) -> u32 {
	let mut cost: u32 = 0;
	for distance_cost in 1..=distance {
		cost += distance_cost;
	}
	cost
}

fn main() {
	let crab_positions: Vec<u32> = fs::read_to_string("input.txt")
		.unwrap()
		.trim()
		.split(',')
		.map(|s| s.parse().unwrap())
		.collect();

	let mut used_fuel: u32 = u32::MAX;
	let min_position = *crab_positions.iter().min().unwrap();
	let max_position = *crab_positions.iter().max().unwrap();
	for position in min_position..=max_position {
		let mut position_fuel: u32 = 0;
		for crab in crab_positions.iter() {
			let distance = if *crab > position {
				crab - position
			} else {
				position - crab
			};
			position_fuel += fuel_cost(distance);
			if position_fuel > used_fuel {
				break;
			}
		}
		if position_fuel < used_fuel {
			used_fuel = position_fuel;
		}
	}

	println!("{}", used_fuel);
}
