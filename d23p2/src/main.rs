use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

const START_CONFIG: [char; 16] = [
	'C', 'D', 'D', 'B', 'B', 'C', 'B', 'C', 'A', 'B', 'A', 'D', 'D', 'A', 'C', 'A',
];

fn energy_used_to_move(amphipod_type: char) -> usize {
	match amphipod_type {
		'A' => 1,
		'B' => 10,
		'C' => 100,
		'D' => 1000,
		_ => panic!("Invalid amphipod type {}", amphipod_type),
	}
}

fn amphipod_room_entry_space(amphipod_type: char) -> usize {
	match amphipod_type {
		'A' => 2,
		'B' => 4,
		'C' => 6,
		'D' => 8,
		_ => panic!("Invalid amphipod type {}", amphipod_type),
	}
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Configuration {
	hallway: [Option<char>; 11],
	room_a: [Option<char>; 4],
	room_b: [Option<char>; 4],
	room_c: [Option<char>; 4],
	room_d: [Option<char>; 4],
}

impl Configuration {
	fn new() -> Self {
		let room_a = [
			Some(START_CONFIG[0]),
			Some(START_CONFIG[1]),
			Some(START_CONFIG[2]),
			Some(START_CONFIG[3]),
		];
		let room_b = [
			Some(START_CONFIG[4]),
			Some(START_CONFIG[5]),
			Some(START_CONFIG[6]),
			Some(START_CONFIG[7]),
		];
		let room_c = [
			Some(START_CONFIG[8]),
			Some(START_CONFIG[9]),
			Some(START_CONFIG[10]),
			Some(START_CONFIG[11]),
		];
		let room_d = [
			Some(START_CONFIG[12]),
			Some(START_CONFIG[13]),
			Some(START_CONFIG[14]),
			Some(START_CONFIG[15]),
		];
		let hallway = [None; 11];
		Self {
			hallway,
			room_a,
			room_b,
			room_c,
			room_d,
		}
	}

	fn complete(&self) -> bool {
		self.hallway.iter().all(|v| v.is_none())
			&& self.room_a.iter().all(|v| *v == Some('A'))
			&& self.room_b.iter().all(|v| *v == Some('B'))
			&& self.room_c.iter().all(|v| *v == Some('C'))
			&& self.room_d.iter().all(|v| *v == Some('D'))
	}

	fn room_can_be_entered(&self, amphipod_type: char) -> bool {
		match amphipod_type {
			'A' => self.room_a.iter().all(|x| x.is_none() || *x == Some('A')),
			'B' => self.room_b.iter().all(|x| x.is_none() || *x == Some('B')),
			'C' => self.room_c.iter().all(|x| x.is_none() || *x == Some('C')),
			'D' => self.room_d.iter().all(|x| x.is_none() || *x == Some('D')),
			_ => panic!("Invalid amphipod type {}", amphipod_type),
		}
	}

	fn next_index_in_room(&self, amphipod_type: char) -> Option<usize> {
		let room = match amphipod_type {
			'A' => &self.room_a,
			'B' => &self.room_b,
			'C' => &self.room_c,
			'D' => &self.room_d,
			_ => panic!("Invalid amphipod type {}", amphipod_type),
		};
		if room[3].is_none() {
			Some(3)
		} else if room[2].is_none() {
			Some(2)
		} else if room[1].is_none() {
			Some(1)
		} else if room[0].is_none() {
			Some(0)
		} else {
			None
		}
	}
}

#[derive(Clone, Eq, PartialEq)]
struct State {
	energy_used: usize,
	current_config: Configuration,
}

impl State {
	fn new() -> Self {
		Self {
			energy_used: 0,
			current_config: Configuration::new(),
		}
	}
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.energy_used
			.cmp(&other.energy_used)
			.then(self.current_config.cmp(&other.current_config))
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

fn main() {
	let mut states: BinaryHeap<Reverse<State>> = BinaryHeap::new();
	states.push(Reverse(State::new()));
	let mut found_states: HashSet<Configuration> = HashSet::new();
	loop {
		let next_state = states.pop().expect("Ran out of configurations before completing");
		if next_state.0.current_config.complete() {
			println!("Energy: {}", next_state.0.energy_used);
			break;
		}
		if found_states.contains(&next_state.0.current_config) {
			continue;
		}
		found_states.insert(next_state.0.current_config.clone());
		for (space_index, space) in next_state.0.current_config.hallway.iter().enumerate() {
			let amphipod = if let Some(val) = space {
				*val
			} else {
				continue;
			};
			let mut left_space = space_index;
			while left_space > 0 && next_state.0.current_config.hallway[left_space - 1].is_none() {
				left_space -= 1;
			}
			let mut right_space = space_index;
			while right_space < 10 && next_state.0.current_config.hallway[right_space + 1].is_none() {
				right_space += 1;
			}
			if left_space == right_space {
				continue;
			}
			let room_entry_space = amphipod_room_entry_space(amphipod);
			let can_enter_room = left_space <= room_entry_space && right_space >= room_entry_space;
			if !can_enter_room {
				continue;
			}
			let can_enter_room = next_state.0.current_config.room_can_be_entered(amphipod);
			if !can_enter_room {
				continue;
			}

			let spaces_to_room = if room_entry_space > space_index {
				room_entry_space - space_index
			} else {
				space_index - room_entry_space
			};
			let spaces_into_room = next_state.0.current_config.next_index_in_room(amphipod).unwrap() + 1;
			let spaces_to_move = spaces_to_room + spaces_into_room;
			let energy_used = spaces_to_move * energy_used_to_move(amphipod);
			let mut new_state = next_state.clone();
			new_state.0.energy_used += energy_used;
			new_state.0.current_config.hallway[space_index] = None;
			match amphipod {
				'A' => new_state.0.current_config.room_a[spaces_into_room - 1] = Some('A'),
				'B' => new_state.0.current_config.room_b[spaces_into_room - 1] = Some('B'),
				'C' => new_state.0.current_config.room_c[spaces_into_room - 1] = Some('C'),
				'D' => new_state.0.current_config.room_d[spaces_into_room - 1] = Some('D'),
				_ => unreachable!(),
			}
			states.push(new_state);
		}

		if !next_state
			.0
			.current_config
			.room_a
			.iter()
			.all(|v| v.is_none() || *v == Some('A'))
		{
			let (amphipod, spaces_from_room) = next_state
				.0
				.current_config
				.room_a
				.iter()
				.enumerate()
				.filter(|(_, v)| v.is_some())
				.map(|(index, cell)| (cell.unwrap(), index + 1))
				.next()
				.unwrap();
			let hall_start = 2;
			let mut left_space = hall_start;
			while left_space > 0 && next_state.0.current_config.hallway[left_space - 1].is_none() {
				left_space -= 1;
			}
			let mut right_space = hall_start;
			while right_space < 10 && next_state.0.current_config.hallway[right_space + 1].is_none() {
				right_space += 1;
			}
			for dest_space in left_space..=right_space {
				if dest_space != 2 && dest_space != 4 && dest_space != 6 && dest_space != 8 {
					let distance = if dest_space > 2 { dest_space - 2 } else { 2 - dest_space } + spaces_from_room;
					let mut new_state = next_state.clone();
					new_state.0.current_config.room_a[spaces_from_room - 1] = None;
					new_state.0.current_config.hallway[dest_space] = Some(amphipod);
					new_state.0.energy_used += distance * energy_used_to_move(amphipod);
					states.push(new_state);
				}
			}
			if amphipod != 'A' {
				let room_entry_space = amphipod_room_entry_space(amphipod);
				if room_entry_space >= left_space
					&& room_entry_space <= right_space
					&& next_state.0.current_config.room_can_be_entered(amphipod)
				{
					let mut new_state = next_state.clone();
					let stop_index = new_state.0.current_config.next_index_in_room(amphipod).unwrap();
					let spaces_into_room = stop_index + 1;
					let room = match amphipod {
						'B' => &mut new_state.0.current_config.room_b,
						'C' => &mut new_state.0.current_config.room_c,
						'D' => &mut new_state.0.current_config.room_d,
						_ => unreachable!(),
					};
					room[stop_index] = Some(amphipod);
					new_state.0.current_config.room_a[spaces_from_room - 1] = None;
					let hall_dest = amphipod_room_entry_space(amphipod);
					let hall_distance = if hall_dest > 2 { hall_dest - 2 } else { 2 - hall_dest };
					let distance = hall_distance + spaces_from_room + spaces_into_room;
					new_state.0.energy_used += distance * energy_used_to_move(amphipod);
					states.push(new_state);
				}
			}
		}

		if !next_state
			.0
			.current_config
			.room_b
			.iter()
			.all(|v| v.is_none() || *v == Some('B'))
		{
			let (amphipod, spaces_from_room) = next_state
				.0
				.current_config
				.room_b
				.iter()
				.enumerate()
				.filter(|(_, v)| v.is_some())
				.map(|(index, cell)| (cell.unwrap(), index + 1))
				.next()
				.unwrap();
			let hall_start = 4;
			let mut left_space = hall_start;
			while left_space > 0 && next_state.0.current_config.hallway[left_space - 1].is_none() {
				left_space -= 1;
			}
			let mut right_space = hall_start;
			while right_space < 10 && next_state.0.current_config.hallway[right_space + 1].is_none() {
				right_space += 1;
			}
			for dest_space in left_space..=right_space {
				if dest_space != 2 && dest_space != 4 && dest_space != 6 && dest_space != 8 {
					let distance = if dest_space > 4 { dest_space - 4 } else { 4 - dest_space } + spaces_from_room;
					let mut new_state = next_state.clone();
					new_state.0.current_config.room_b[spaces_from_room - 1] = None;
					new_state.0.current_config.hallway[dest_space] = Some(amphipod);
					new_state.0.energy_used += distance * energy_used_to_move(amphipod);
					states.push(new_state);
				}
			}
			if amphipod != 'B' {
				let room_entry_space = amphipod_room_entry_space(amphipod);
				if room_entry_space >= left_space
					&& room_entry_space <= right_space
					&& next_state.0.current_config.room_can_be_entered(amphipod)
				{
					let mut new_state = next_state.clone();
					let stop_index = new_state.0.current_config.next_index_in_room(amphipod).unwrap();
					let spaces_into_room = stop_index + 1;
					let room = match amphipod {
						'A' => &mut new_state.0.current_config.room_a,
						'C' => &mut new_state.0.current_config.room_c,
						'D' => &mut new_state.0.current_config.room_d,
						_ => unreachable!(),
					};
					room[stop_index] = Some(amphipod);
					new_state.0.current_config.room_b[spaces_from_room - 1] = None;
					let hall_dest = amphipod_room_entry_space(amphipod);
					let hall_distance = if hall_dest > 4 { hall_dest - 4 } else { 4 - hall_dest };
					let distance = hall_distance + spaces_from_room + spaces_into_room;
					new_state.0.energy_used += distance * energy_used_to_move(amphipod);
					states.push(new_state);
				}
			}
		}

		if !next_state
			.0
			.current_config
			.room_c
			.iter()
			.all(|v| v.is_none() || *v == Some('C'))
		{
			let (amphipod, spaces_from_room) = next_state
				.0
				.current_config
				.room_c
				.iter()
				.enumerate()
				.filter(|(_, v)| v.is_some())
				.map(|(index, cell)| (cell.unwrap(), index + 1))
				.next()
				.unwrap();
			let hall_start = 6;
			let mut left_space = hall_start;
			while left_space > 0 && next_state.0.current_config.hallway[left_space - 1].is_none() {
				left_space -= 1;
			}
			let mut right_space = hall_start;
			while right_space < 10 && next_state.0.current_config.hallway[right_space + 1].is_none() {
				right_space += 1;
			}
			for dest_space in left_space..=right_space {
				if dest_space != 2 && dest_space != 4 && dest_space != 6 && dest_space != 8 {
					let distance = if dest_space > 6 { dest_space - 6 } else { 6 - dest_space } + spaces_from_room;
					let mut new_state = next_state.clone();
					new_state.0.current_config.room_c[spaces_from_room - 1] = None;
					new_state.0.current_config.hallway[dest_space] = Some(amphipod);
					new_state.0.energy_used += distance * energy_used_to_move(amphipod);
					states.push(new_state);
				}
			}
			if amphipod != 'C' {
				let room_entry_space = amphipod_room_entry_space(amphipod);
				if room_entry_space >= left_space
					&& room_entry_space <= right_space
					&& next_state.0.current_config.room_can_be_entered(amphipod)
				{
					let mut new_state = next_state.clone();
					let stop_index = new_state.0.current_config.next_index_in_room(amphipod).unwrap();
					let spaces_into_room = stop_index + 1;
					let room = match amphipod {
						'A' => &mut new_state.0.current_config.room_a,
						'B' => &mut new_state.0.current_config.room_b,
						'D' => &mut new_state.0.current_config.room_d,
						_ => unreachable!(),
					};
					room[stop_index] = Some(amphipod);
					new_state.0.current_config.room_c[spaces_from_room - 1] = None;
					let hall_dest = amphipod_room_entry_space(amphipod);
					let hall_distance = if hall_dest > 6 { hall_dest - 6 } else { 6 - hall_dest };
					let distance = hall_distance + spaces_from_room + spaces_into_room;
					new_state.0.energy_used += distance * energy_used_to_move(amphipod);
					states.push(new_state);
				}
			}
		}

		if !next_state
			.0
			.current_config
			.room_d
			.iter()
			.all(|v| v.is_none() || *v == Some('D'))
		{
			let (amphipod, spaces_from_room) = next_state
				.0
				.current_config
				.room_d
				.iter()
				.enumerate()
				.filter(|(_, v)| v.is_some())
				.map(|(index, cell)| (cell.unwrap(), index + 1))
				.next()
				.unwrap();
			let hall_start = 8;
			let mut left_space = hall_start;
			while left_space > 0 && next_state.0.current_config.hallway[left_space - 1].is_none() {
				left_space -= 1;
			}
			let mut right_space = hall_start;
			while right_space < 10 && next_state.0.current_config.hallway[right_space + 1].is_none() {
				right_space += 1;
			}
			for dest_space in left_space..=right_space {
				if dest_space != 2 && dest_space != 4 && dest_space != 6 && dest_space != 8 {
					let distance = if dest_space > 8 { dest_space - 8 } else { 8 - dest_space } + spaces_from_room;
					let mut new_state = next_state.clone();
					new_state.0.current_config.room_d[spaces_from_room - 1] = None;
					new_state.0.current_config.hallway[dest_space] = Some(amphipod);
					new_state.0.energy_used += distance * energy_used_to_move(amphipod);
					states.push(new_state);
				}
			}
			if amphipod != 'D' {
				let room_entry_space = amphipod_room_entry_space(amphipod);
				if room_entry_space >= left_space
					&& room_entry_space <= right_space
					&& next_state.0.current_config.room_can_be_entered(amphipod)
				{
					let mut new_state = next_state.clone();
					let stop_index = new_state.0.current_config.next_index_in_room(amphipod).unwrap();
					let spaces_into_room = stop_index + 1;
					let room = match amphipod {
						'A' => &mut new_state.0.current_config.room_a,
						'B' => &mut new_state.0.current_config.room_b,
						'C' => &mut new_state.0.current_config.room_c,
						_ => unreachable!(),
					};
					room[stop_index] = Some(amphipod);
					new_state.0.current_config.room_d[spaces_from_room - 1] = None;
					let hall_dest = amphipod_room_entry_space(amphipod);
					let hall_distance = if hall_dest > 8 { hall_dest - 8 } else { 8 - hall_dest };
					let distance = hall_distance + spaces_from_room + spaces_into_room;
					new_state.0.energy_used += distance * energy_used_to_move(amphipod);
					states.push(new_state);
				}
			}
		}
	}
}
