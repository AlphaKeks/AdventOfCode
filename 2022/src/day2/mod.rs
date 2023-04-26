use {
	advent_of_code::AdventOfCode,
	anyhow::{anyhow, Result},
	itertools::Itertools,
	std::str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

impl Move {
	fn beats(self) -> Self {
		match self {
			Self::Rock => Self::Scissors,
			Self::Paper => Self::Rock,
			Self::Scissors => Self::Paper,
		}
	}

	fn play(self, opponent: Self) -> Outcome {
		if self.beats() == opponent {
			Outcome::Win
		} else if opponent.beats() == self {
			Outcome::Loss
		} else {
			Outcome::Draw
		}
	}
}

impl FromStr for Move {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		match s {
			"A" | "X" => Ok(Self::Rock),
			"B" | "Y" => Ok(Self::Paper),
			"C" | "Z" => Ok(Self::Scissors),
			input => Err(anyhow!("`{input}` is not a valid move.")),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
	Loss = 0,
	Draw = 3,
	Win = 6,
}

impl Outcome {
	fn compute_move(self, opponent: Move) -> Move {
		match self {
			Outcome::Loss => opponent.beats(),
			Outcome::Draw => opponent,
			Outcome::Win => [Move::Rock, Move::Paper, Move::Scissors]
				.into_iter()
				.find(|&m| m.beats() == opponent)
				.expect("All possible matchups are covered."),
		}
	}
}

impl FromStr for Outcome {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		match s {
			"X" => Ok(Self::Loss),
			"Y" => Ok(Self::Draw),
			"Z" => Ok(Self::Win),
			input => Err(anyhow!("`{input}` is not a valid outcome.")),
		}
	}
}

#[derive(Default)]
pub struct AoC {
	input: &'static str,
}

impl AoC {
	pub fn new(input: &'static str) -> Self {
		Self { input }
	}

	fn parse(&self) -> Result<Vec<String>> {
		Ok(self
			.input
			.lines()
			.map(String::from)
			.collect_vec())
	}
}

impl AdventOfCode for AoC {
	type SolutionA = usize;
	type SolutionB = usize;

	const PROBLEM: &'static str = include_str!("./problem.md");
	const DAY: usize = 2;

	fn part_a(&mut self) -> Result<Self::SolutionA> {
		Ok(self
			.parse()?
			.into_iter()
			.filter_map(|game| {
				let (opponent, me) = game
					.split_once(' ')
					.expect("Expected format `A Y`");
				let opponent = opponent.parse::<Move>().ok()?;
				let me = me.parse::<Move>().ok()?;
				Some(me as usize + me.play(opponent) as usize)
			})
			.sum())
	}

	fn part_b(&mut self) -> Result<Self::SolutionB> {
		Ok(self
			.parse()?
			.into_iter()
			.filter_map(|game| {
				let (opponent, outcome) = game
					.split_once(' ')
					.expect("Expected format `A Y`");
				let opponent = opponent.parse::<Move>().ok()?;
				let outcome = outcome.parse::<Outcome>().ok()?;
				let me = outcome.compute_move(opponent);
				Some(me as usize + outcome as usize)
			})
			.sum())
	}
}
