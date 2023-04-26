use {
	advent_of_code::AdventOfCode,
	anyhow::{bail as yeet, Result},
	aoc_2022::*,
	clap::Parser,
};

#[derive(Debug, Parser)]
struct Args {
	#[arg(long)]
	#[clap(default_value = "1")]
	day: usize,
}

#[rustfmt::skip]
fn main() -> Result<()> {
	let args = Args::parse();

	match args.day {
		1 => day1::AoC::new(include_str!("./day1/input")).run()?,
		2 => day2::AoC::new(include_str!("./day2/input")).run()?,
		3 => day3::AoC::new(include_str!("./day3/input")).run()?,
		4 => day4::AoC::new(include_str!("./day4/input")).run()?,
		5 => day5::AoC::new(include_str!("./day5/input")).run()?,
		6 => day6::AoC::new(include_str!("./day6/input")).run()?,
		7 => day7::AoC::new(include_str!("./day7/input"))?.run()?,
		day => yeet!("Day {day} has not been solved yet.")
	};

	Ok(())
}
