#![allow(clippy::type_complexity)]

use anyhow::anyhow;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use std::ops::RangeInclusive;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_4/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<usize> {
	let (_, input) = parse_input(input).map_err(|err| anyhow!("{err:?}"))?;
	Ok(input
		.into_iter()
		.filter(|(a, b)| {
			(a.start() >= b.start() && a.end() <= b.end())
				|| (b.start() >= a.start() && b.end() <= a.end())
		})
		.count())
}

fn part_2(input: &str) -> anyhow::Result<usize> {
	let (_, input) = parse_input(input).map_err(|err| anyhow!("{err:?}"))?;
	Ok(input
		.into_iter()
		.filter(|(a, b)| a.clone().into_iter().any(|a| b.contains(&a)))
		.count())
}

fn parse_input(input: &str) -> IResult<&str, Vec<(RangeInclusive<u64>, RangeInclusive<u64>)>> {
	use nom::character::complete::{char, u64};
	many1(terminated(
		map(
			separated_pair(
				separated_pair(u64, char('-'), u64),
				char(','),
				separated_pair(u64, char('-'), u64),
			),
			|((a_start, a_end), (b_start, b_end))| (a_start..=a_end, b_start..=b_end),
		),
		multispace0,
	))(input)
}

#[cfg(test)]
mod tests {
	use crate::{part_1, part_2};

	const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

	#[test]
	fn part_1_example() {
		let p1 = part_1(EXAMPLE).unwrap();
		assert_eq!(p1, 2);
	}

	#[test]
	fn part_2_example() {
		let p2 = part_2(EXAMPLE).unwrap();
		assert_eq!(p2, 4);
	}
}
