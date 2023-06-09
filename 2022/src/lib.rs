pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

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

	#[test]
	fn day2() -> Result<()> {
		let mut day = day2::AoC::new(include_str!("./day2/input"));

		let part_a = day.part_a()?;
		assert_eq!(part_a, 11449);

		let part_b = day.part_b()?;
		assert_eq!(part_b, 13187);

		Ok(())
	}

	#[test]
	fn day3() -> Result<()> {
		let mut day = day3::AoC::new(include_str!("./day3/input"));

		let part_a = day.part_a()?;
		assert_eq!(part_a, 7826);

		let part_b = day.part_b()?;
		assert_eq!(part_b, 2577);

		Ok(())
	}

	#[test]
	fn day4() -> Result<()> {
		let mut day = day4::AoC::new(include_str!("./day4/input"));

		let part_a = day.part_a()?;
		assert_eq!(part_a, 503);

		let part_b = day.part_b()?;
		assert_eq!(part_b, 827);

		Ok(())
	}

	#[test]
	fn day5() -> Result<()> {
		let mut day = day5::AoC::new(include_str!("./day5/input"));

		let part_a = day.part_a()?;
		assert_eq!(part_a, "FWNSHLDNZ");

		let part_b = day.part_b()?;
		assert_eq!(part_b, "RNRGDNFQG");

		Ok(())
	}

	#[test]
	fn day6() -> Result<()> {
		let mut day = day6::AoC::new(include_str!("./day6/input"));

		let part_a = day.part_a()?;
		assert_eq!(part_a, 1651);

		let part_b = day.part_b()?;
		assert_eq!(part_b, 3837);

		Ok(())
	}

	#[test]
	fn day7() -> Result<()> {
		let mut day = day7::AoC::new(include_str!("./day7/input"))?;

		let part_a = day.part_a()?;
		assert_eq!(part_a, 1915606);

		let part_b = day.part_b()?;
		assert_eq!(part_b, 5025657);

		Ok(())
	}
}
