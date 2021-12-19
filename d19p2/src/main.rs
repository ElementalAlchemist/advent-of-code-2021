use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinate {
	x: i32,
	y: i32,
	z: i32,
}

impl Coordinate {
	fn new(coord_str: &str) -> Self {
		let mut str_iter = coord_str.split(',');
		let x = str_iter.next().unwrap().parse().unwrap();
		let y = str_iter.next().unwrap().parse().unwrap();
		let z = str_iter.next().unwrap().parse().unwrap();
		assert!(str_iter.next().is_none());
		Self { x, y, z }
	}

	fn get_all_facings(&self) -> Vec<Self> {
		// Start facing positive X
		let xp_90 = self.rotate_facing_axis();
		let xp_180 = xp_90.rotate_facing_axis();
		let xp_270 = xp_180.rotate_facing_axis();
		let xn = self.face_opposite_way();
		let xn_90 = xn.rotate_facing_axis();
		let xn_180 = xn_90.rotate_facing_axis();
		let xn_270 = xn_180.rotate_facing_axis();
		let yp = self.face_y_axis();
		let yp_90 = yp.rotate_facing_axis();
		let yp_180 = yp_90.rotate_facing_axis();
		let yp_270 = yp_180.rotate_facing_axis();
		let yn = yp.face_opposite_way();
		let yn_90 = yn.rotate_facing_axis();
		let yn_180 = yn_90.rotate_facing_axis();
		let yn_270 = yn_180.rotate_facing_axis();
		let zp = self.face_z_axis();
		let zp_90 = zp.rotate_facing_axis();
		let zp_180 = zp_90.rotate_facing_axis();
		let zp_270 = zp_180.rotate_facing_axis();
		let zn = zp.face_opposite_way();
		let zn_90 = zn.rotate_facing_axis();
		let zn_180 = zn_90.rotate_facing_axis();
		let zn_270 = zn_180.rotate_facing_axis();
		vec![
			self.clone(),
			xp_90,
			xp_180,
			xp_270,
			xn,
			xn_90,
			xn_180,
			xn_270,
			yp,
			yp_90,
			yp_180,
			yp_270,
			yn,
			yn_90,
			yn_180,
			yn_270,
			zp,
			zp_90,
			zp_180,
			zp_270,
			zn,
			zn_90,
			zn_180,
			zn_270,
		]
	}

	fn rotate_facing_axis(&self) -> Self {
		let x = self.x;
		let y = self.z;
		let z = -self.y;
		Self { x, y, z }
	}

	fn face_opposite_way(&self) -> Self {
		let x = -self.x;
		let y = -self.y;
		let z = self.z;
		Self { x, y, z }
	}

	fn face_y_axis(&self) -> Self {
		let x = self.y;
		let y = -self.x;
		let z = self.z;
		Self { x, y, z }
	}

	fn face_z_axis(&self) -> Self {
		let x = self.z;
		let y = self.y;
		let z = -self.x;
		Self { x, y, z }
	}

	/// Gets the position of the passed-in coordinate relative to this one
	fn position_relative(&self, position_of_coord: &Self) -> Self {
		let x = position_of_coord.x - self.x;
		let y = position_of_coord.y - self.y;
		let z = position_of_coord.z - self.z;
		Self { x, y, z }
	}

	fn translate(&self, translate_by: &Self) -> Self {
		let x = self.x + translate_by.x;
		let y = self.y + translate_by.y;
		let z = self.z + translate_by.z;
		Self { x, y, z }
	}

	fn is_origin(&self) -> bool {
		self.x == 0 && self.y == 0 && self.z == 0
	}

	fn manhattan(&self, other: &Self) -> i32 {
		let mut distance = 0;
		distance += (self.x - other.x).abs();
		distance += (self.y - other.y).abs();
		distance += (self.z - other.z).abs();
		distance
	}
}

impl Display for Coordinate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{},{},{}", self.x, self.y, self.z)
	}
}

struct BeaconGroup {
	locations: Vec<Coordinate>,
}

impl BeaconGroup {
	fn get_pairwise_translations(&self) -> Vec<Vec<Coordinate>> {
		let mut pairwise_coordinates: Vec<Vec<Coordinate>> = Vec::with_capacity(self.locations.len());
		for coordinate in self.locations.iter() {
			let mut this_coordinate: Vec<Coordinate> = Vec::with_capacity(self.locations.len());
			for other_coordinate in self.locations.iter() {
				this_coordinate.push(coordinate.position_relative(other_coordinate));
			}
			pairwise_coordinates.push(this_coordinate);
		}
		pairwise_coordinates
	}
}

fn main() {
	let scanner_beacons: Vec<BeaconGroup> = {
		let input: Vec<String> = fs::read_to_string("input.txt")
			.unwrap()
			.lines()
			.filter(|s| !s.is_empty())
			.map(String::from)
			.collect();
		let mut beacons: Vec<BeaconGroup> = Vec::new();
		let mut current_scanner: Vec<Coordinate> = Vec::new();
		for line in input {
			if line.starts_with("---") {
				if line != "--- scanner 0 ---" {
					beacons.push(BeaconGroup {
						locations: current_scanner,
					});
					current_scanner = Vec::new();
				}
				continue;
			}
			current_scanner.push(Coordinate::new(&line));
		}
		beacons.push(BeaconGroup {
			locations: current_scanner,
		});
		beacons
	};

	// Levels of this:
	// Vec< // scanners
	//  Vec< // beacons detected by scanner
	//   Vec< // beacons relative to the upper level's beacon
	//    Coordinate // Location of the inner beacon relative to the outer one
	// >>>
	let mut pairwise_beacon_comparisons: Vec<Vec<Vec<Coordinate>>> = Vec::with_capacity(scanner_beacons.len());
	for scanner in scanner_beacons.iter() {
		pairwise_beacon_comparisons.push(scanner.get_pairwise_translations());
	}

	// Normalize
	for scanner in pairwise_beacon_comparisons.iter_mut() {
		for origin_beacon in scanner.iter_mut() {
			for detected_beacon in origin_beacon.iter_mut() {
				let mut facings = detected_beacon.get_all_facings();
				facings.sort_unstable();
				*detected_beacon = facings.remove(0);
			}
		}
	}

	let mut overlapping_beacons: HashMap<usize, HashMap<usize, HashMap<usize, usize>>> = HashMap::new();
	for (first_scanner_index, first_scanner) in pairwise_beacon_comparisons.iter().enumerate() {
		for (second_scanner_index, second_scanner) in pairwise_beacon_comparisons.iter().enumerate() {
			if first_scanner_index == second_scanner_index {
				continue;
			}
			if let Some(overlapping_first_scanner) = overlapping_beacons.get(&first_scanner_index) {
				if overlapping_first_scanner.contains_key(&second_scanner_index) {
					continue;
				}
			}

			let mut same_beacons: HashMap<usize, usize> = HashMap::new();
			for (first_beacon_origin_index, first_beacon_origin) in first_scanner.iter().enumerate() {
				for (first_beacon_remote_index, first_beacon_distance) in first_beacon_origin.iter().enumerate() {
					if first_beacon_distance.is_origin() {
						continue;
					}
					'next_map: for (second_beacon_origin_index, second_beacon_origin) in
						second_scanner.iter().enumerate()
					{
						for (second_beacon_remote_index, second_beacon_distance) in
							second_beacon_origin.iter().enumerate()
						{
							if second_beacon_distance.is_origin() {
								continue;
							}
							if *first_beacon_distance == *second_beacon_distance {
								same_beacons.insert(first_beacon_origin_index, second_beacon_origin_index);
								same_beacons.insert(first_beacon_remote_index, second_beacon_remote_index);
								break 'next_map;
							}
						}
					}
				}
			}
			if same_beacons.len() >= 12 {
				(*overlapping_beacons.entry(second_scanner_index).or_default()).insert(
					first_scanner_index,
					same_beacons
						.values()
						.zip(same_beacons.keys())
						.map(|(a, b)| (*a, *b))
						.collect(),
				);
				(*overlapping_beacons.entry(first_scanner_index).or_default())
					.insert(second_scanner_index, same_beacons);
			}
		}
	}

	let mut mapped_scanners: HashMap<usize, (Coordinate, usize)> = HashMap::new();
	mapped_scanners.insert(0, (Coordinate { x: 0, y: 0, z: 0 }, 0));
	while mapped_scanners.len() != scanner_beacons.len() {
		for (mapped_scanner_index, overlap_data) in overlapping_beacons.iter() {
			let mapped_scanner_data = if let Some(data) = mapped_scanners.get(mapped_scanner_index) {
				data.clone()
			} else {
				continue;
			};
			for (next_scanner_index, same_beacons) in overlap_data.iter() {
				if mapped_scanners.contains_key(next_scanner_index) {
					continue;
				}
				let mut same_beacon_iter = same_beacons.iter();
				let (mapped_origin_beacon_index, next_origin_beacon_index) = same_beacon_iter.next().unwrap();
				let (mapped_remote_beacon_index, next_remote_beacon_index) = same_beacon_iter.next().unwrap();

				let mapped_origin_beacon = scanner_beacons[*mapped_scanner_index].locations
					[*mapped_origin_beacon_index]
					.get_all_facings()
					.remove(mapped_scanner_data.1);
				let next_origin_beacon = &scanner_beacons[*next_scanner_index].locations[*next_origin_beacon_index];
				let mapped_remote_beacon = scanner_beacons[*mapped_scanner_index].locations
					[*mapped_remote_beacon_index]
					.get_all_facings()
					.remove(mapped_scanner_data.1);
				let next_remote_beacon = &scanner_beacons[*next_scanner_index].locations[*next_remote_beacon_index];

				let next_origin_facings = next_origin_beacon.get_all_facings();
				let next_remote_facings = next_remote_beacon.get_all_facings();
				let mut facing_index: Option<usize> = None;
				for check_facing_index in 0..next_origin_facings.len() {
					let next_origin_facing = &next_origin_facings[check_facing_index];
					let translation = next_origin_facing.position_relative(&mapped_origin_beacon);
					let next_remote_facing = next_remote_facings[check_facing_index].translate(&translation);
					if mapped_remote_beacon == next_remote_facing {
						facing_index = Some(check_facing_index);
						break;
					}
				}
				let facing_index = facing_index.expect("Failed to find facing for matching beacons");

				let next_origin_facing = &next_origin_facings[facing_index];
				let relative_scanner_coordinate = next_origin_facing.position_relative(&mapped_origin_beacon);
				let scanner_coordinate = relative_scanner_coordinate.translate(&mapped_scanner_data.0);
				mapped_scanners.insert(*next_scanner_index, (scanner_coordinate, facing_index));
			}
		}
	}

	let mut largest_manhattan = 0;
	for scanner in mapped_scanners.values() {
		for other_scanner in mapped_scanners.values() {
			let distance = scanner.0.manhattan(&other_scanner.0);
			if distance > largest_manhattan {
				largest_manhattan = distance;
			}
		}
	}

	println!("{}", largest_manhattan);
}
