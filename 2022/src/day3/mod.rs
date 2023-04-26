use {
	advent_of_code::AdventOfCode, anyhow::Result, itertools::Itertools, std::collections::HashSet,
};

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

	fn calc_priority(char: char) -> usize {
		if char.is_ascii_lowercase() {
			char as usize - 96
		} else if char.is_ascii_uppercase() {
			char as usize - 38
		} else {
			panic!("Expected alphabetic character.");
		}
	}
}

impl AdventOfCode for AoC {
	type SolutionA = usize;
	type SolutionB = usize;

	const PROBLEM: &'static str = include_str!("./problem.md");
	const DAY: usize = 3;

	fn part_a(&mut self) -> Result<Self::SolutionA> {
		Ok(self
			.parse()?
			.into_iter()
			.map(|rucksack| rucksack.chars().collect_vec())
			.map(|rucksack| {
				let mid = rucksack.len() / 2;
				let first = HashSet::<_>::from_iter(&rucksack[..mid]);
				let second = HashSet::<_>::from_iter(&rucksack[mid..]);
				first
					.intersection(&second)
					.map(|item| Self::calc_priority(**item))
					.sum::<usize>()
			})
			.sum())
	}

	fn part_b(&mut self) -> Result<Self::SolutionB> {
		Ok(self
			.parse()?
			.chunks(3)
			.map(|group| {
				group
					.iter()
					.map(|elf| elf.chars().collect_vec())
					.collect_vec()
			})
			.filter_map(|group| {
				let common = group[0].iter().find(|item| {
					group[1].iter().contains(item) && group[2].iter().contains(item)
				})?;

				Some(Self::calc_priority(*common))
			})
			.sum())
	}
}
