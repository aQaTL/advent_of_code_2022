use anyhow::anyhow;
use nom::IResult;
use std::iter::once;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_10/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: \n{}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let (_, input) = parse_input(input).map_err(|err| anyhow!("{err:?}"))?;

	let mut x = 1;
	let mut cycle_count = 0;
	let mut sum = 0;

	let mut evaluate_signal_strength = |cycle_count, x| {
		if (cycle_count - 20) % 40 == 0 {
			sum += cycle_count * x
		}
	};

	for op in input {
		match op {
			Op::Noop => {
				cycle_count += 1;
				evaluate_signal_strength(cycle_count, x);
			}
			Op::AddX(val) => {
				cycle_count += 1;
				evaluate_signal_strength(cycle_count, x);
				cycle_count += 1;
				evaluate_signal_strength(cycle_count, x);
				x += val;
			}
		}
	}

	Ok(sum)
}

fn part_2(input: &str) -> anyhow::Result<String> {
	let (_, input) = parse_input(input).map_err(|err| anyhow!("{err:?}"))?;

	const WIDTH: usize = 40;
	const HEIGHT: usize = 6;

	let mut sprite = [[' '; WIDTH]; HEIGHT];

	let mut x: i64 = 1;
	let mut draw_position: usize = 0;
	let mut row: usize = 0;

	let mut draw_pixel = |x| {
		let sprite_position = (x - 1)..=(x + 1);
		if sprite_position.contains(&(draw_position as i64)) {
			sprite[row][draw_position] = '\u{2588}';
		}
		if (draw_position + 1) % 40 == 0 {
			row += 1;
		}
		draw_position = (draw_position + 1) % 40;
	};

	for op in input {
		match op {
			Op::Noop => {
				draw_pixel(x);
			}
			Op::AddX(val) => {
				draw_pixel(x);
				draw_pixel(x);
				x += val;
			}
		}
	}

	Ok(sprite
		.into_iter()
		.flat_map(|line| line.into_iter().chain(once('\n')))
		.collect::<String>())
}

enum Op {
	AddX(i64),
	Noop,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Op>> {
	use nom::{
		branch::alt,
		bytes::complete::tag,
		character::complete::{char, i64, multispace1},
		combinator::map,
		multi::many1,
		sequence::{preceded, terminated, tuple},
	};

	many1(terminated(
		alt((
			map(preceded(tuple((tag("addx"), char(' '))), i64), Op::AddX),
			map(tag("noop"), |_| Op::Noop),
		)),
		multispace1,
	))(input)
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

	#[test]
	fn part_1() {
		assert_eq!(super::part_1(EXAMPLE).unwrap(), 13140);
	}

	#[test]
	fn part_2() {
		let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
		.replace("#", "\u{2588}")
		.replace(".", " ");

		assert_eq!(super::part_2(EXAMPLE).unwrap(), expected);
	}
}
