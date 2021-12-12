use std::collections::HashMap;
use std::fs;

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
			paths.entry(one_end.clone()).or_default().push(other_end.clone());
			paths.entry(other_end).or_default().push(one_end);
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
				if *next_dest == next_dest.to_ascii_lowercase() && path.contains(next_dest) {
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
