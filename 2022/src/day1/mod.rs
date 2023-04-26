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

	fn parse(&self) -> Result<Vec<Vec<usize>>> {
		Ok(self
			.input
			.split("\n\n")
			.map(|elf| {
				elf.lines()
					.flat_map(|calories| calories.parse())
					.collect_vec()
			})
			.collect_vec())
	}
}

impl AdventOfCode for AoC {
	type SolutionA = usize;
	type SolutionB = usize;

	const PROBLEM: &'static str = include_str!("./problem.md");
	const DAY: usize = 1;

	fn part_a(&mut self) -> Result<Self::SolutionA> {
		self.parse()?
			.into_iter()
			.map(|calories| calories.into_iter().sum())
			.max()
			.ok_or(anyhow!("Did not find any calories."))
	}

	fn part_b(&mut self) -> Result<Self::SolutionB> {
		Ok(self
			.parse()?
			.into_iter()
			.map(|calories| calories.into_iter().sum::<usize>())
			.sorted_by(|a, b| b.cmp(a))
			.take(3)
			.sum())
	}
}
