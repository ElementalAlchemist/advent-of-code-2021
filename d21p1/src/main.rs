#[derive(Default)]
struct Die {
	value: u32,
}

impl Die {
	fn roll(&mut self) -> u32 {
		self.value += 1;
		let value = self.value;
		self.value %= 100;
		value
	}
}

fn main() {
	let mut player_one_pos: u32 = 1;
	let mut player_two_pos: u32 = 3;

	let mut die = Die::default();

	let mut player_one_score: u32 = 0;
	let mut player_two_score: u32 = 0;

	let mut die_rolls = 0;

	loop {
		let roll = die.roll() + die.roll() + die.roll();
		die_rolls += 3;
		player_one_pos = (player_one_pos + roll) % 10;
		player_one_score += player_one_pos;

		if player_one_score >= 1000 {
			break;
		}

		let roll = die.roll() + die.roll() + die.roll();
		die_rolls += 3;
		player_two_pos = (player_two_pos + roll) % 10;
		player_two_score += player_two_pos;

		if player_two_score >= 1000 {
			break;
		}
	}

	println!("{}", player_one_score.min(player_two_score) * die_rolls);
}
