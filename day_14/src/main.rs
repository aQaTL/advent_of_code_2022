use itertools::Itertools;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_14/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<usize> {
	let mut grid = HashMap::<(i64, i64), Point>::new();

	for line in parse_input(input)? {
		for ((a_x, a_y), (b_x, b_y)) in line.into_iter().tuple_windows() {
			for y in (a_y.min(b_y))..=(a_y.max(b_y)) {
				for x in (a_x.min(b_x))..=(a_x.max(b_x)) {
					grid.insert((x, y), Point::Rock);
				}
			}
		}
	}

	let sand_producer = (500, 0);
	let lowest_point = *grid.keys().map(|(_x, y)| y).max().unwrap();

	'l: loop {
		let mut sand = sand_producer;

		'falling_loop: loop {
			for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
				let (nx, ny) = (sand.0 + dx, sand.1 + dy);
				match grid.get(&(nx, ny)).unwrap_or(&Point::Air) {
					Point::Rock | Point::RestingSand => {
						continue;
					}
					Point::Air => {
						if ny > lowest_point {
							break 'l;
						}
						sand = (nx, ny);
						continue 'falling_loop;
					}
				}
			}
			grid.insert(sand, Point::RestingSand);
			break 'falling_loop;
		}
	}

	Ok(grid
		.values()
		.filter(|p| matches!(p, Point::RestingSand))
		.count())
}

fn part_2(input: &str) -> anyhow::Result<usize> {
	let mut grid = HashMap::<(i64, i64), Point>::new();

	for line in parse_input(input)? {
		for ((a_x, a_y), (b_x, b_y)) in line.into_iter().tuple_windows() {
			for y in (a_y.min(b_y))..=(a_y.max(b_y)) {
				for x in (a_x.min(b_x))..=(a_x.max(b_x)) {
					grid.insert((x, y), Point::Rock);
				}
			}
		}
	}

	let sand_producer = (500, 0);
	let lowest_point = 2 + *grid.keys().map(|(_x, y)| y).max().unwrap();

	'l: loop {
		let mut sand = sand_producer;

		'falling_loop: loop {
			for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
				let (nx, ny) = (sand.0 + dx, sand.1 + dy);
				match grid.get(&(nx, ny)).unwrap_or(&Point::Air) {
					Point::Rock | Point::RestingSand => {
						continue;
					}
					Point::Air => {
						if ny == lowest_point {
							continue;
						}
						sand = (nx, ny);
						continue 'falling_loop;
					}
				}
			}
			grid.insert(sand, Point::RestingSand);
			if sand == sand_producer {
				break 'l;
			}
			break 'falling_loop;
		}
	}

	Ok(grid
		.values()
		.filter(|p| matches!(p, Point::RestingSand))
		.count())
}

fn parse_input(input: &str) -> Result<Vec<Vec<(i64, i64)>>, nom::Err<nom::error::Error<String>>> {
	use nom::bytes::complete::tag;
	use nom::character::complete::{char, i64, multispace0};
	use nom::combinator::all_consuming;
	use nom::multi::{many1, separated_list1};
	use nom::sequence::{separated_pair, terminated};

	all_consuming(many1(terminated(
		separated_list1(tag(" -> "), separated_pair(i64, char(','), i64)),
		multispace0,
	)))(input)
	.map(|(_input, res)| res)
	.map_err(|err: nom::Err<nom::error::Error<&str>>| err.to_owned())
}

#[derive(Hash, Copy, Clone)]
enum Point {
	Air,
	Rock,
	RestingSand,
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE).unwrap(), 24);
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(super::part_2(EXAMPLE).unwrap(), 93);
	}
}
