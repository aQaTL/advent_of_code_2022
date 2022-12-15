use anyhow::bail;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_15/input.txt")?;
	println!("Part 1: {}", part_1(&input, 2000000)?);
	println!("Part 2: {}", part_2(&input, 4_000_000)?);
	Ok(())
}

fn part_1(input: &str, target_y: i64) -> anyhow::Result<usize> {
	let input = parse_input(input)?;

	let mut grid = HashMap::<(i64, i64), u8>::new();

	for ((sensor_x, sensor_y), (beacon_x, beacon_y)) in input {
		let distance = manhattan_distance((sensor_x, sensor_y), (beacon_x, beacon_y));

		if !((sensor_y - distance)..=(sensor_y + distance)).contains(&target_y) {
			continue;
		}

		grid.insert((sensor_x, sensor_y), b'S');
		grid.insert((beacon_x, beacon_y), b'B');

		for y in 0..=distance {
			let new_y = sensor_y - y;
			if new_y != target_y {
				continue;
			}
			for x in (sensor_x - distance + y)..=(sensor_x + distance - y) {
				grid.entry((x, sensor_y - y)).or_insert(b'#');
			}
		}
		for y in 0..=distance {
			let new_y = sensor_y + y;
			if new_y != target_y {
				continue;
			}
			for x in (sensor_x - distance + y)..=(sensor_x + distance - y) {
				grid.entry((x, sensor_y + y)).or_insert(b'#');
			}
		}
	}

	Ok(grid
		.iter()
		.filter(|((_x, y), item)| *y == target_y && **item != b'B')
		.count())
}

fn part_2(input: &str, search_coord: i64) -> anyhow::Result<i64> {
	let input = parse_input(input)?;

	let search_space = 0..=search_coord;

	let input: Vec<_> = input
		.into_iter()
		.map(|(sensor, beacon)| (sensor, manhattan_distance(sensor, beacon)))
		.collect();

	for ((sensor_x, sensor_y), distance) in &input {
		for y in ((sensor_y - distance - 1)..=(sensor_y + distance + 1))
			.into_iter()
			.filter(|y| search_space.contains(y))
		{
			let x_left = (distance + 1) - (sensor_y - y).abs();
			let b_x_left = sensor_x + x_left;
			let b_x_right = (-sensor_x) - x_left;
			for x in [b_x_left, b_x_right]
				.into_iter()
				.filter(|x| search_space.contains(x))
			{
				if !input
					.iter()
					.any(|(sensor, distance)| manhattan_distance(*sensor, (x, y)) <= *distance)
				{
					return Ok((x * 4000000) + y);
				}
			}
		}
	}

	bail!("not found");
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
	(a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_input(
	input: &str,
) -> Result<Vec<((i64, i64), (i64, i64))>, nom::Err<nom::error::Error<String>>> {
	use nom::bytes::complete::tag;
	use nom::character::complete::{i64, multispace0};
	use nom::combinator::{all_consuming, map};
	use nom::multi::many1;
	use nom::sequence::tuple;

	all_consuming(many1(map(
		tuple((
			tag("Sensor at x="),
			i64,
			tag(", y="),
			i64,
			tag(": closest beacon is at x="),
			i64,
			tag(", y="),
			i64,
			multispace0,
		)),
		|(_, sensor_x, _, sensor_y, _, beacon_x, _, beacon_y, _)| {
			((sensor_x, sensor_y), (beacon_x, beacon_y))
		},
	)))(input)
	.map(|(_input, res)| res)
	.map_err(|err: nom::Err<nom::error::Error<&str>>| err.to_owned())
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE, 10).unwrap(), 26);
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(super::part_2(EXAMPLE, 20).unwrap(), 56000011);
	}
}
