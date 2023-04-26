use {
	advent_of_code::AdventOfCode, anyhow::Result, itertools::Itertools, std::collections::VecDeque,
};

#[derive(Default)]
pub struct AoC {
	input: &'static str,
}

type Stack = VecDeque<char>;
type Instructions = (usize, usize, usize);

impl AoC {
	pub fn new(input: &'static str) -> Self {
		Self { input }
	}

	fn parse(&self) -> Result<(Vec<Stack>, Vec<Instructions>)> {
		let (crates, instructions) = self
			.input
			.split_once("\n\n")
			.expect("Expected 2 sections: crates and instructions");

		let mut crate_stacks = Vec::new();

		for row in crates
			.lines()
			.map(|row| row.chars().collect_vec())
		{
			for (idx, stack_crate) in row.chunks(4).enumerate() {
				// e.g. `[N] `
				let stack_crate = stack_crate[1];

				match crate_stacks.get_mut(idx) {
					// We are still in the first row and have found a crate
					None if stack_crate.is_alphabetic() => {
						crate_stacks.push(VecDeque::from_iter([stack_crate]));
					}

					// We are still in the first row and have found an empty spot
					None => {
						crate_stacks.push(VecDeque::new());
					}

					// We are not in the first row anymore and have found a crate.
					Some(stack) if stack_crate.is_alphabetic() => {
						stack.push_back(stack_crate);
					}

					// We are not in the first row anymore, but we do not have a crate to push onto the
					// stack.
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

		Ok((crate_stacks, instructions))
	}
}

impl AdventOfCode for AoC {
	type SolutionA = String;
	type SolutionB = String;

	const PROBLEM: &'static str = include_str!("./problem.md");
	const DAY: usize = 5;

	fn part_a(&mut self) -> Result<Self::SolutionA> {
		let (mut stacks, instructions) = self.parse()?;
		for (count, from, to) in instructions {
			for _ in 0..count {
				let crate_in_question = stacks[from].pop_front().unwrap();
				stacks[to].push_front(crate_in_question);
			}
		}

		Ok(stacks
			.into_iter()
			.map(|stack| stack[0].to_string())
			.collect())
	}

	fn part_b(&mut self) -> Result<Self::SolutionB> {
		let (mut stacks, instructions) = self.parse()?;
		for (count, from, to) in instructions {
			let mut crates_in_question = stacks[from]
				.drain(..count)
				.collect_vec();

			crates_in_question.reverse();

			for crate_in_question in crates_in_question {
				stacks[to].push_front(crate_in_question);
			}
		}

		Ok(stacks
			.into_iter()
			.map(|stack| stack[0].to_string())
			.collect())
	}
}
