use std::fs;

fn main() {
	let height_map: Vec<Vec<u32>> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(|s| s.chars().map(|c| c as u32 - 48).collect())
		.collect();

	let mut total_risk: u32 = 0;
	for (line_num, line_data) in height_map.iter().enumerate() {
		for (position, value) in line_data.iter().enumerate() {
			let mut other_heights: Vec<u32> = Vec::new();
			if position > 0 {
				other_heights.push(height_map[line_num][position - 1]);
			}
			if position < line_data.len() - 1 {
				other_heights.push(height_map[line_num][position + 1]);
			}
			if line_num > 0 {
				other_heights.push(height_map[line_num - 1][position]);
			}
			if line_num < height_map.len() - 1 {
				other_heights.push(height_map[line_num + 1][position]);
			}
			if other_heights.iter().all(|x| *x > *value) {
				total_risk += *value + 1;
			}
		}
	}
	println!("{}", total_risk);
}
