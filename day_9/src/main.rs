use anyhow::bail;
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_9/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<usize> {
	// 0: x, 1: y
	let mut head: (i64, i64) = (0, 0);
	let mut tail: (i64, i64) = (0, 0);

	let mut tail_positions = HashSet::from([(0, 0)]);

	for (direction, step_count) in input.trim().lines().flat_map(|l| l.split(' ').tuples()) {
		let step_count: i64 = step_count.parse()?;

		for _ in 0..step_count {
			match direction {
				"U" => head.1 += 1,
				"R" => head.0 += 1,
				"D" => head.1 -= 1,
				"L" => head.0 -= 1,
				_ => bail!("unknown direction {direction}"),
			}
			if distance(head, tail) > 1 {
				tail.0 += (head.0 - tail.0).signum();
				tail.1 += (head.1 - tail.1).signum();
			}
			tail_positions.insert(tail);
		}
	}

	Ok(tail_positions.len())
}

fn part_2(input: &str) -> anyhow::Result<usize> {
	// 0: x, 1: y
	// head idx : 0
	// tail idx: 9
	let mut knots = [(0_i64, 0_i64); 10];

	let mut tail_positions = HashSet::from([(0, 0)]);

	for (direction, step_count) in input.trim().lines().flat_map(|l| l.split(' ').tuples()) {
		let step_count: i64 = step_count.parse()?;

		for _ in 0..step_count {
			match direction {
				"U" => knots[0].1 += 1,
				"R" => knots[0].0 += 1,
				"D" => knots[0].1 -= 1,
				"L" => knots[0].0 -= 1,
				_ => bail!("unknown direction {direction}"),
			}

			for idx in 0..(knots.len() - 1) {
				let (head, tail) = get_2_mut_unchecked(&mut knots, idx, idx + 1);
				if distance(*head, *tail) <= 1 {
					continue;
				}

				let (x_diff, y_diff) = (head.0 - tail.0, head.1 - tail.1);
				tail.0 += x_diff.signum();
				tail.1 += y_diff.signum();
			}

			tail_positions.insert(knots[9]);
		}
	}

	Ok(tail_positions.len())
}

fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
	i64::max(i64::abs(b.0 - a.0), i64::abs(b.1 - a.1))
}

fn get_2_mut_unchecked<T>(slice: &mut [T], a_idx: usize, b_idx: usize) -> (&mut T, &mut T) {
	let ptr = slice.as_mut_ptr();
	unsafe {
		let a: &mut T = &mut *ptr.add(a_idx);
		let b: &mut T = &mut *ptr.add(b_idx);
		(a, b)
	}
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE_1).unwrap(), 13);
	}

	#[test]
	fn distance_test() {
		assert_eq!(super::distance((1, 1), (0, 0)), 1);
		assert_eq!(super::distance((-1, -1), (0, 0)), 1);
		assert_eq!(super::distance((1, -1), (0, 0)), 1);
		assert_eq!(super::distance((-1, 1), (0, 0)), 1);
		assert_eq!(super::distance((2, 1), (0, 0)), 2);
		assert_eq!(super::distance((4, 2), (3, 0)), 2);
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(super::part_2(EXAMPLE_1).unwrap(), 1);
	}

	const EXAMPLE_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

	#[test]
	fn part_2_example_2() {
		assert_eq!(super::part_2(EXAMPLE_2).unwrap(), 36);
	}
}
