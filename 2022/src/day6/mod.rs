use {
	advent_of_code::AdventOfCode,
	anyhow::{anyhow, Result},
	itertools::Itertools,
};

#[derive(Default)]
pub struct AoC {
	input: &'static str,
}

impl AoC {
	pub fn new(input: &'static str) -> Self {
		Self { input }
	}
}

impl AdventOfCode for AoC {
	type SolutionA = usize;
	type SolutionB = usize;

	const PROBLEM: &'static str = include_str!("./problem.md");
	const DAY: usize = 6;

	fn part_a(&mut self) -> Result<Self::SolutionA> {
		self.input
			.chars()
			.collect_vec()
			.windows(4)
			.position(|chars| {
				for (idx, char) in chars.iter().enumerate() {
					if chars[(idx + 1)..].contains(char) {
						return false;
					}
				}
				true
			})
			.map(|pos| pos + 4)
			.ok_or(anyhow!("Did not find a marker."))
	}

	fn part_b(&mut self) -> Result<Self::SolutionB> {
		self.input
			.chars()
			.collect_vec()
			.windows(14)
			.position(|chars| {
				for (idx, char) in chars.iter().enumerate() {
					if chars[(idx + 1)..].contains(char) {
						return false;
					}
				}
				true
			})
			.map(|pos| pos + 14)
			.ok_or(anyhow!("Did not find a marker."))
	}
}
