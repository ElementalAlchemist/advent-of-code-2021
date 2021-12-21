use std::collections::HashMap;

fn points_for_space(space: u32) -> u32 {
	if space == 0 {
		10
	} else {
		space
	}
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct GameState {
	player_one_pos: u32,
	player_two_pos: u32,
	player_one_score: u32,
	player_two_score: u32,
}

impl GameState {
	fn get_die_results() -> Vec<u32> {
		let mut die_results: Vec<u32> = Vec::new();
		for first in 1..=3 {
			for second in 1..=3 {
				for third in 1..=3 {
					die_results.push(first + second + third);
				}
			}
		}
		die_results
	}

	fn roll_player_one(&self) -> Vec<Self> {
		let die_results = Self::get_die_results();

		let mut results: Vec<Self> = Vec::new();
		for result in die_results.iter() {
			let mut new_state = self.clone();
			new_state.player_one_pos += result;
			new_state.player_one_pos %= 10;
			new_state.player_one_score += points_for_space(new_state.player_one_pos);
			results.push(new_state);
		}
		results
	}

	fn roll_player_two(&self) -> Vec<Self> {
		let die_results = Self::get_die_results();

		let mut results: Vec<Self> = Vec::new();
		for result in die_results.iter() {
			let mut new_state = self.clone();
			new_state.player_two_pos += result;
			new_state.player_two_pos %= 10;
			new_state.player_two_score += points_for_space(new_state.player_two_pos);
			results.push(new_state);
		}
		results
	}

	fn game_winner(&self) -> Option<WinningPlayer> {
		if self.player_one_score >= 21 {
			Some(WinningPlayer::One)
		} else if self.player_two_score >= 21 {
			Some(WinningPlayer::Two)
		} else {
			None
		}
	}
}

impl Default for GameState {
	fn default() -> Self {
		let player_one_pos = 1;
		let player_two_pos = 3;
		Self {
			player_one_pos,
			player_two_pos,
			player_one_score: 0,
			player_two_score: 0,
		}
	}
}

enum WinningPlayer {
	One,
	Two,
}

#[derive(Default)]
struct WinCount {
	p1: u64,
	p2: u64,
}

fn main() {
	let mut game_states: HashMap<GameState, u64> = HashMap::new();
	game_states.insert(GameState::default(), 1);
	let mut win_count = WinCount::default();

	loop {
		let mut new_game_states = HashMap::new();
		for (state, number_of_games) in game_states.iter() {
			let roll_p1 = state.roll_player_one();
			for new_state_p1 in roll_p1.iter() {
				if let Some(winner) = new_state_p1.game_winner() {
					match winner {
						WinningPlayer::One => win_count.p1 += number_of_games,
						WinningPlayer::Two => win_count.p2 += number_of_games,
					}
					continue;
				}
				let roll_p2 = new_state_p1.roll_player_two();
				for new_state_p2 in roll_p2.iter() {
					match new_state_p2.game_winner() {
						Some(WinningPlayer::One) => win_count.p1 += number_of_games,
						Some(WinningPlayer::Two) => win_count.p2 += number_of_games,
						None => (*new_game_states.entry(new_state_p2.clone()).or_default()) += number_of_games,
					}
				}
			}
		}
		game_states = new_game_states;
		if game_states.is_empty() {
			break;
		}
	}

	println!("{}", win_count.p1);
	println!("{}", win_count.p2);
}
