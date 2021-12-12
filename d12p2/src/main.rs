use std::collections::{HashMap, HashSet};
use std::fs;

fn can_add_small_cave(path: &[String], next_cave: &String) -> bool {
	let mut visited_caves: HashSet<&String> = HashSet::new();
	let mut visited_small_twice = false;
	for cave in path.iter() {
		if *cave == cave.to_ascii_lowercase() {
			if visited_caves.contains(cave) {
				if visited_small_twice {
					return false;
				}
				visited_small_twice = true;
				continue;
			}
			visited_caves.insert(cave);
		}
	}
	!visited_small_twice || !visited_caves.contains(next_cave)
}

fn main() {
	let path_segments: HashMap<String, Vec<String>> = {
		let path_descriptions: Vec<String> = fs::read_to_string("input.txt")
			.unwrap()
			.lines()
			.filter(|s| !s.is_empty())
			.map(String::from)
			.collect();
		let mut paths: HashMap<String, Vec<String>> = HashMap::new();
		for path in path_descriptions.iter() {
			let mut path_iter = path.split('-');
			let one_end = String::from(path_iter.next().unwrap());
			let other_end = String::from(path_iter.next().unwrap());
			if one_end != "end" && other_end != "start" {
				paths.entry(one_end.clone()).or_default().push(other_end.clone());
			}
			if other_end != "end" && one_end != "start" {
				paths.entry(other_end).or_default().push(one_end);
			}
		}
		paths
	};

	let mut paths: Vec<Vec<String>> = vec![vec![String::from("start")]];
	loop {
		let mut added_paths = false;
		let mut new_paths: Vec<Vec<String>> = Vec::new();
		for path in paths.iter() {
			if path.last().unwrap() == "end" {
				new_paths.push(path.clone());
				continue;
			}
			for next_dest in path_segments[path.last().unwrap()].iter() {
				if *next_dest == next_dest.to_ascii_lowercase() && !can_add_small_cave(path, next_dest) {
					continue;
				}
				let mut next_path = path.clone();
				next_path.push(next_dest.clone());
				new_paths.push(next_path);
				added_paths = true;
			}
		}
		paths = new_paths;
		if !added_paths {
			break;
		}
	}

	println!("{}", paths.len());
}
