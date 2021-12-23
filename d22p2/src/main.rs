use std::fs;

#[derive(Clone, Eq, PartialEq)]
struct Coordinate {
	x: i64,
	y: i64,
	z: i64,
}

struct Step {
	activate: bool,
	region: Region,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Region {
	x_min: i64,
	x_max: i64,
	y_min: i64,
	y_max: i64,
	z_min: i64,
	z_max: i64,
}

impl Region {
	fn new(x_min: i64, x_max: i64, y_min: i64, y_max: i64, z_min: i64, z_max: i64) -> Option<Self> {
		if x_min > x_max || y_min > y_max || z_min > z_max {
			None
		} else {
			Some(Self {
				x_min,
				x_max,
				y_min,
				y_max,
				z_min,
				z_max,
			})
		}
	}

	fn remove_region(&self, remove_region: &Self) -> Vec<Region> {
		let remaining_regions = vec![
			Region::new(
				self.x_min,
				(remove_region.x_min - 1).min(self.x_max),
				self.y_min,
				self.y_max,
				self.z_min,
				self.z_max,
			),
			Region::new(
				remove_region.x_min.max(self.x_min),
				remove_region.x_max.min(self.x_max),
				self.y_min,
				(remove_region.y_min - 1).min(self.y_max),
				self.z_min,
				self.z_max,
			),
			Region::new(
				remove_region.x_min.max(self.x_min),
				remove_region.x_max.min(self.x_max),
				remove_region.y_min.max(self.y_min),
				remove_region.y_max.min(self.y_max),
				self.z_min,
				(remove_region.z_min - 1).min(self.z_max),
			),
			Region::new(
				remove_region.x_min.max(self.x_min),
				remove_region.x_max.min(self.x_max),
				remove_region.y_min.max(self.y_min),
				remove_region.y_max.min(self.y_max),
				(remove_region.z_max + 1).max(self.z_min),
				self.z_max,
			),
			Region::new(
				remove_region.x_min.max(self.x_min),
				remove_region.x_max.min(self.x_max),
				(remove_region.y_max + 1).max(self.y_min),
				self.y_max,
				self.z_min,
				self.z_max,
			),
			Region::new(
				(remove_region.x_max + 1).max(self.x_min),
				self.x_max,
				self.y_min,
				self.y_max,
				self.z_min,
				self.z_max,
			),
		];

		remaining_regions.into_iter().flatten().collect()
	}

	fn volume(&self) -> i64 {
		(self.x_max - self.x_min + 1) * (self.y_max - self.y_min + 1) * (self.z_max - self.z_min + 1)
	}
}

fn main() {
	let steps: Vec<Step> = {
		let mut steps: Vec<Step> = Vec::new();
		for input_line in fs::read_to_string("input.txt")
			.unwrap()
			.lines()
			.filter(|s| !s.is_empty())
		{
			let mut parts = input_line.split(' ');
			let activate = parts.next().unwrap() == "on";
			let mut coord_parts = parts.next().unwrap().split(',');
			let x_range = coord_parts.next().unwrap();
			let y_range = coord_parts.next().unwrap();
			let z_range = coord_parts.next().unwrap();

			let mut x_range = x_range.split('=').nth(1).unwrap().split("..");
			let x_min: i64 = x_range.next().unwrap().parse().unwrap();
			let x_max: i64 = x_range.next().unwrap().parse().unwrap();
			let mut y_range = y_range.split('=').nth(1).unwrap().split("..");
			let y_min: i64 = y_range.next().unwrap().parse().unwrap();
			let y_max: i64 = y_range.next().unwrap().parse().unwrap();
			let mut z_range = z_range.split('=').nth(1).unwrap().split("..");
			let z_min: i64 = z_range.next().unwrap().parse().unwrap();
			let z_max: i64 = z_range.next().unwrap().parse().unwrap();

			steps.push(Step {
				activate,
				region: Region {
					x_min,
					x_max,
					y_min,
					y_max,
					z_min,
					z_max,
				},
			});
		}
		steps
	};

	let mut active_regions: Vec<Region> = Vec::new();
	for (step_num, step) in steps.iter().enumerate() {
		if step.activate {
			let mut new_regions = vec![step.region.clone()];
			for current_region in active_regions.iter() {
				let mut new_new_regions = Vec::new();
				for new_region in new_regions.iter() {
					for region in new_region.remove_region(current_region) {
						new_new_regions.push(region);
					}
				}
				new_regions = new_new_regions;
			}
			for region in new_regions.into_iter() {
				active_regions.push(region);
			}
		} else {
			let mut new_active_regions = Vec::new();
			for region in active_regions.iter() {
				for new_region in region.remove_region(&step.region) {
					new_active_regions.push(new_region);
				}
			}
			active_regions = new_active_regions;
		}

		println!("Completed step: {}; region count: {}", step_num, active_regions.len());
		let volume: i64 = active_regions.iter().map(|region| region.volume()).sum();
		println!("Step volume: {}", volume);
		println!();
	}

	let volume: i64 = active_regions.iter().map(|region| region.volume()).sum();
	println!("{}", volume);
}
