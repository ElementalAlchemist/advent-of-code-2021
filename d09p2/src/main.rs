use std::collections::HashSet;
use std::fs;

fn search_from(
	height_map: &[Vec<u32>],
	point: (usize, usize),
	point_count: &mut u32,
	visited: &mut HashSet<(usize, usize)>,
) {
	if visited.contains(&point) {
		return;
	}
	if height_map[point.0][point.1] == 9 {
		return;
	}
	*point_count += 1;
	visited.insert(point);
	if point.0 > 0 {
		let new_point = (point.0 - 1, point.1);
		search_from(height_map, new_point, point_count, visited);
	}
	if point.0 < height_map.len() - 1 {
		let new_point = (point.0 + 1, point.1);
		search_from(height_map, new_point, point_count, visited);
	}
	if point.1 > 0 {
		let new_point = (point.0, point.1 - 1);
		search_from(height_map, new_point, point_count, visited);
	}
	if point.1 < height_map[point.0].len() - 1 {
		let new_point = (point.0, point.1 + 1);
		search_from(height_map, new_point, point_count, visited);
	}
}

fn main() {
	let height_map: Vec<Vec<u32>> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(|s| s.chars().map(|c| c as u32 - 48).collect())
		.collect();

	let mut min_points: Vec<(usize, usize)> = Vec::new();
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
				min_points.push((line_num, position));
			}
		}
	}

	let mut basin_sizes: Vec<u32> = Vec::new();
	for basin_point in min_points.iter() {
		let mut basin_point_count: u32 = 0;
		search_from(&height_map, *basin_point, &mut basin_point_count, &mut HashSet::new());
		basin_sizes.push(basin_point_count);
	}
	let mut top_basin_sizes: Vec<u32> = vec![0, 0, 0];
	for size in basin_sizes.iter() {
		if *size > top_basin_sizes[0] {
			top_basin_sizes[0] = *size;
			top_basin_sizes.sort_unstable();
		}
	}

	let result: u32 = top_basin_sizes.iter().product::<u32>();
	println!("{:?} = {}", top_basin_sizes, result);
}
