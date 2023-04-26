use {
	advent_of_code::AdventOfCode, anyhow::Result, itertools::Itertools, std::ops::RangeInclusive,
};

#[derive(Default)]
pub struct AoC {
	input: &'static str,
}

impl AoC {
	pub fn new(input: &'static str) -> Self {
		Self { input }
	}

	fn parse(&self) -> Result<Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>> {
		Ok(self
			.input
			.lines()
			.filter_map(|pair| {
				let (elf1, elf2) = pair.split_once(',')?;
				let (start1, end1) = elf1.split_once('-')?;
				let elf1 = start1.parse::<usize>().ok()?..=end1.parse::<usize>().ok()?;
				let (start2, end2) = elf2.split_once('-')?;
				let elf2 = start2.parse::<usize>().ok()?..=end2.parse::<usize>().ok()?;
				Some((elf1, elf2))
			})
			.collect_vec())
	}
}

impl AdventOfCode for AoC {
	type SolutionA = usize;
	type SolutionB = usize;

	const PROBLEM: &'static str = include_str!("./problem.md");
	const DAY: usize = 4;

	fn part_a(&mut self) -> Result<Self::SolutionA> {
		Ok(self
			.parse()?
			.into_iter()
			.filter(|(elf1, elf2)| {
				(elf1.start() >= elf2.start() && elf1.end() <= elf2.end())
					|| (elf2.start() >= elf1.start() && elf2.end() <= elf1.end())
			})
			.count())
	}

	fn part_b(&mut self) -> Result<Self::SolutionB> {
		Ok(self
			.parse()?
			.into_iter()
			.filter(|(elf1, elf2)| {
				(elf1.start() >= elf2.start() && elf1.end() <= elf2.end())
					|| (elf2.start() >= elf1.start() && elf2.end() <= elf1.end())
					|| (elf1.start() <= elf2.start()
						&& elf1.end() >= elf2.start()
						&& elf1.end() <= elf2.end())
					|| (elf2.start() <= elf1.start()
						&& elf2.end() >= elf1.start()
						&& elf2.end() <= elf1.end())
			})
			.count())
	}
}
