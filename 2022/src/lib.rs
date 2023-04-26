pub mod day1;

#[cfg(test)]
mod tests {
	use {super::*, advent_of_code::AdventOfCode, anyhow::Result};

	#[test]
	fn day1() -> Result<()> {
		let mut day = day1::AoC::new(include_str!("./day1/input"));

		let part_a = day.part_a()?;
		assert_eq!(part_a, 67027);

		let part_b = day.part_b()?;
		assert_eq!(part_b, 197291);

		Ok(())
	}
}
