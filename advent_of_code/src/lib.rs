use {
	anyhow::Result,
	std::{
		fmt::{Debug, Display},
		time::Instant,
	},
};

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Part {
	A,
	B,
}

impl Display for Part {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Part::A => "Part A",
			Part::B => "Part B",
		})
	}
}

pub trait AdventOfCode {
	type SolutionA: Display;
	type SolutionB: Display;

	const PROBLEM: &'static str;
	const DAY: usize;

	fn part_a(&mut self) -> Result<Self::SolutionA>;
	fn part_b(&mut self) -> Result<Self::SolutionB>;

	fn run(mut self) -> Result<()>
	where
		Self: Sized,
	{
		println!("--- Day {} ---", Self::DAY);
		println!("{}", Self::PROBLEM);

		let start = Instant::now();
		let solution_a = self.part_a()?;
		println!("Part A: {} ({:?})", solution_a, start.elapsed());

		let start = Instant::now();
		let solution_b = self.part_b()?;
		println!("Part B: {} ({:?})", solution_b, start.elapsed());

		Ok(())
	}
}
