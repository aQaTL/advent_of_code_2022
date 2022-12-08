fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_8/input.txt")?;
	println!("Part 1: {}", part_1(&input));
	println!("Part 2: {}", part_2(&input));
	Ok(())
}

fn part_1(input: &str) -> usize {
	let width = input.lines().next().unwrap().len();
	let height = input.lines().count();
	let grid: Vec<u8> = input
		.trim()
		.lines()
		.flat_map(|line| line.trim().as_bytes().iter().copied())
		.map(|b| b - b'0')
		.collect();

	let grid_item = |x: i64, y: i64| {
		if x >= (width as i64) || y >= (height as i64) || x < 0 || y < 0 {
			return None;
		}
		grid.get((y as usize) * width + (x as usize))
	};

	let mut visible = 0;

	for y in 0..height {
		for x in 0..width {
			let current = grid_item(x as i64, y as i64).unwrap();
			// left, up, right, down
			let directions: [(i64, i64); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
			'directions: for (dx, dy) in directions {
				for idx in 1.. {
					match grid_item((x as i64) + dx * idx, (y as i64) + dy * idx) {
						Some(item) if item >= current => continue 'directions,
						Some(_) => (),
						None => {
							visible += 1;
							break 'directions;
						}
					}
				}
			}
		}
	}

	visible
}

fn part_2(input: &str) -> usize {
	let width = input.lines().next().unwrap().len();
	let height = input.lines().count();
	let grid: Vec<u8> = input
		.trim()
		.lines()
		.flat_map(|line| line.trim().as_bytes().iter().copied())
		.map(|b| b - b'0')
		.collect();

	let grid_item = |x: i64, y: i64| {
		if x >= (width as i64) || y >= (height as i64) || x < 0 || y < 0 {
			return None;
		}
		grid.get((y as usize) * width + (x as usize))
	};

	let mut scenic_highscore = 0;

	for y in 0..height {
		for x in 0..width {
			let current = grid_item(x as i64, y as i64).unwrap();
			// left, up, right, down
			let directions: [(i64, i64); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
			let mut seen_trees_in_directions = [0; 4];
			'directions: for (direction_idx, (dx, dy)) in directions.into_iter().enumerate() {
				for idx in 1.. {
					match grid_item((x as i64) + dx * idx, (y as i64) + dy * idx) {
						Some(item) if item >= current => {
							seen_trees_in_directions[direction_idx] += 1;
							continue 'directions;
						}
						Some(_) => {
							seen_trees_in_directions[direction_idx] += 1;
						}
						None => {
							continue 'directions;
						}
					}
				}
			}
			let scenic_score = seen_trees_in_directions.into_iter().product();
			if scenic_score > scenic_highscore {
				scenic_highscore = scenic_score;
			}
		}
	}

	scenic_highscore
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &str = "30373
25512
65332
33549
35390
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE), 21)
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(super::part_2(EXAMPLE), 8);
	}
}
