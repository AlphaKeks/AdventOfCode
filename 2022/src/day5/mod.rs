use {advent_of_code::AdventOfCode, anyhow::Result, itertools::Itertools};

#[derive(Default)]
pub struct AoC {
	input: &'static str,
}

type Column = Vec<char>;
type Instructions = (usize, usize, usize);

impl AoC {
	pub fn new(input: &'static str) -> Self {
		Self { input }
	}

	fn parse(&self) -> Result<(Vec<Column>, Vec<Instructions>)> {
		let (crates, instructions) = self
			.input
			.split_once("\n\n")
			.expect("Expected 2 sections: crates and instructions");

		let mut columns = Vec::new();

		for row in crates
			.lines()
			.map(|row| row.chars().collect_vec())
		{
			for (idx, cell) in row.chunks(4).enumerate() {
				// e.g. `[N] `
				let cell = cell[1];

				match columns.get_mut(idx) {
					// We are still in the first row and have found a crate
					None if cell.is_alphabetic() => {
						columns.push(Vec::from_iter([cell]));
					}

					// We are still in the first row and have found an empty spot
					None => {
						columns.push(Vec::new());
					}

					// We are not in the first row anymore and have found a crate.
					Some(column) if cell.is_alphabetic() => {
						column.push(cell);
					}

					// We are not in the first row anymore, but we do not have a crate to push onto the
					// column.
					_ => {}
				};
			}
		}

		let instructions = instructions
			.lines()
			.filter_map(|line| {
				let (count, from, to) = line
					.split(' ')
					.flat_map(|word| word.parse::<usize>())
					.collect_tuple()?;

				Some((count, from - 1, to - 1))
			})
			.collect_vec();

		Ok((columns, instructions))
	}
}

impl AdventOfCode for AoC {
	type SolutionA = String;
	type SolutionB = String;

	const PROBLEM: &'static str = include_str!("./problem.md");
	const DAY: usize = 5;

	fn part_a(&mut self) -> Result<Self::SolutionA> {
		let (mut columns, instructions) = self.parse()?;
		for (count, from, to) in instructions {
			for _ in 0..count {
				let le_crate = columns[from].remove(0);
				columns[to].insert(0, le_crate);
			}
		}

		Ok(columns
			.into_iter()
			.map(|column| column[0])
			.collect())
	}

	fn part_b(&mut self) -> Result<Self::SolutionB> {
		let (mut columns, instructions) = self.parse()?;
		for (count, from, to) in instructions {
			let mut crates = columns[from]
				.drain(..count)
				.collect_vec();

			crates.reverse();

			for le_crate in crates {
				columns[to].insert(0, le_crate);
			}
		}

		Ok(columns
			.into_iter()
			.map(|column| column[0])
			.collect())
	}
}
