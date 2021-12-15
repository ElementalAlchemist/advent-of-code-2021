use std::collections::{BTreeMap, HashSet};
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct PathState {
	row: usize,
	column: usize,
	total_risk: u32,
}

fn main() {
	let risk_map: Vec<Vec<u32>> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(|s| s.chars().map(|c| (c as u8 - 48) as u32).collect())
		.collect();

	let mut risk_states: BTreeMap<u32, HashSet<PathState>> = BTreeMap::new();
	let mut initial_risk_level = HashSet::new();
	initial_risk_level.insert(PathState {
		row: 0,
		column: 0,
		total_risk: 0,
	});
	risk_states.insert(0, initial_risk_level);

	let destination_row = risk_map.len() - 1;
	let destination_column = risk_map[destination_row].len() - 1;

	let lowest_risk = 'per_risk_level: loop {
		let current_risk = *risk_states.keys().next().unwrap();
		let current_risk_set = risk_states.remove(&current_risk).unwrap();
		for state in current_risk_set {
			if state.row == destination_row && state.column == destination_column {
				break 'per_risk_level state.total_risk;
			}
			if state.row > 0 {
				let mut next_north = state.clone();
				next_north.row -= 1;
				next_north.total_risk += risk_map[next_north.row][next_north.column];
				(*risk_states.entry(next_north.total_risk).or_default()).insert(next_north);
			}
			if state.column > 0 {
				let mut next_west = state.clone();
				next_west.column -= 1;
				next_west.total_risk += risk_map[next_west.row][next_west.column];
				(*risk_states.entry(next_west.total_risk).or_default()).insert(next_west);
			}
			let mut next_south = state.clone();
			next_south.row += 1;
			if next_south.row < risk_map.len() {
				next_south.total_risk += risk_map[next_south.row][next_south.column];
				(*risk_states.entry(next_south.total_risk).or_default()).insert(next_south);
			}
			let mut next_east = state.clone();
			next_east.column += 1;
			if next_east.column < risk_map[next_east.row].len() {
				next_east.total_risk += risk_map[next_east.row][next_east.column];
				(*risk_states.entry(next_east.total_risk).or_default()).insert(next_east);
			}
		}
	};

	println!("{}", lowest_risk);
}
