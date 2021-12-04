use std::fs;

struct Board {
	numbers: [u32; 25],
	marked: [bool; 25],
}

impl Board {
	fn complete(&self) -> bool {
		for row_num in 0..5 {
			let mut row_complete = true;
			for row_index in row_num * 5..row_num * 5 + 5 {
				if !self.marked[row_index] {
					row_complete = false;
					break;
				}
			}
			if row_complete {
				return true;
			}
		}

		for column_num in 0..5 {
			let mut column_complete = true;
			for column_index in 0..5 {
				if !self.marked[column_num + 5 * column_index] {
					column_complete = false;
					break;
				}
			}
			if column_complete {
				return true;
			}
		}
		false
	}

	fn mark(&mut self, number: u32) {
		for (num_index, num) in self.numbers.iter().enumerate() {
			if *num == number {
				self.marked[num_index] = true;
			}
		}
	}

	fn score(&self, last_called: u32) -> u32 {
		let mut total_unmarked = 0;
		for (index, marked) in self.marked.iter().enumerate() {
			if !marked {
				total_unmarked += self.numbers[index];
			}
		}
		total_unmarked * last_called
	}
}

fn main() {
	let input: Vec<String> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(String::from)
		.collect();

	let mut boards: Vec<Board> = Vec::new();

	let mut line_iter = input.iter();
	let called_numbers: Vec<u32> = line_iter
		.next()
		.unwrap()
		.trim()
		.split(',')
		.map(|x| x.parse().unwrap())
		.collect();

	'build_boards: loop {
		let mut board_numbers: Vec<String> = Vec::new();
		for _ in 0..5 {
			let next_line = if let Some(line) = line_iter.next() {
				line
			} else {
				break 'build_boards;
			};
			board_numbers.push(next_line.trim().to_owned());
		}
		let mut numbers = [0; 25];
		let mut current_index = 0;
		for line in board_numbers.iter() {
			for number in line.split(' ').filter(|s| !s.is_empty()) {
				numbers[current_index] = number.parse().unwrap();
				current_index += 1;
			}
		}

		boards.push(Board {
			numbers,
			marked: [false; 25],
		});
	}

	for number in called_numbers.iter() {
		for board in boards.iter_mut() {
			board.mark(*number);
			if board.complete() {
				println!("{}", board.score(*number));
				return;
			}
		}
	}
}
