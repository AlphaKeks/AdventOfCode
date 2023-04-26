use {advent_of_code::AdventOfCode, anyhow::Result, aoc_2022::*};

#[rustfmt::skip]
fn main() -> Result<()> {
	let days = [
		day1::AoC::new(include_str!("./day1/input"))
	];

	for aoc in days {
		aoc.run()?;
	}

	Ok(())
}
