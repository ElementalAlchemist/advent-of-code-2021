use std::fs;

#[derive(Clone, Eq, PartialEq)]
enum Cell {
	RightCucumber,
	DownCucumber,
	Vacant,
}

fn main() {
	let mut trench_grid = {
		let input: Vec<String> = fs::read_to_string("input.txt")
			.unwrap()
			.lines()
			.filter(|s| !s.is_empty())
			.map(String::from)
			.collect();
		let mut grid = Vec::new();
		for row in input {
			let mut grid_row = Vec::new();
			for cell in row.chars() {
				if cell == '>' {
					grid_row.push(Cell::RightCucumber);
				} else if cell == 'v' {
					grid_row.push(Cell::DownCucumber);
				} else {
					grid_row.push(Cell::Vacant);
				}
			}
			grid.push(grid_row);
		}
		grid
	};

	let mut iterations: u32 = 0;
	loop {
		let mut moved = false;
		let mut new_grid = trench_grid.clone();
		for (y, row) in trench_grid.iter().enumerate() {
			for (x, cell) in row.iter().enumerate() {
				if *cell != Cell::RightCucumber {
					continue;
				}
				let next_x = (x + 1) % row.len();
				if trench_grid[y][next_x] == Cell::Vacant {
					new_grid[y][next_x] = cell.clone();
					new_grid[y][x] = Cell::Vacant;
					moved = true;
				}
			}
		}
		trench_grid = new_grid.clone();
		for (y, row) in trench_grid.iter().enumerate() {
			for (x, cell) in row.iter().enumerate() {
				if *cell != Cell::DownCucumber {
					continue;
				}
				let next_y = (y + 1) % trench_grid.len();
				if trench_grid[next_y][x] == Cell::Vacant {
					new_grid[next_y][x] = cell.clone();
					new_grid[y][x] = Cell::Vacant;
					moved = true;
				}
			}
		}
		trench_grid = new_grid;
		iterations += 1;
		if !moved {
			break;
		}
	}

	println!("{}", iterations);
}
