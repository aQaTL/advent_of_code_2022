use std::collections::{HashSet, VecDeque};

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_12/input.txt")?;
	println!("Part 1: {}", part_1(&input));
	println!("Part 2: {}", part_2(&input));
	Ok(())
}

fn part_1(input: &str) -> u64 {
	let width = input.trim().lines().next().unwrap().trim().len();
	let height = input.trim().lines().count();
	let mut grid: Vec<u8> = input
		.trim()
		.lines()
		.flat_map(|line| line.trim().as_bytes().iter().copied())
		.collect();

	let start = (0..height)
		.into_iter()
		.find_map(|y| {
			(0..width)
				.find(|x| grid[y * width + x] == b'S')
				.map(|x| (x, y))
		})
		.unwrap();

	let end = (0..height)
		.into_iter()
		.find_map(|y| {
			(0..width)
				.find(|x| grid[y * width + x] == b'E')
				.map(|x| (x, y))
		})
		.unwrap();
	grid[start.1 * width + start.0] = b'a';
	grid[end.1 * width + end.0] = b'z';

	find_shortest_path_from_to(&grid, start, end, width, height).unwrap()
}

fn part_2(input: &str) -> u64 {
	let width = input.trim().lines().next().unwrap().trim().len();
	let height = input.trim().lines().count();
	let mut grid: Vec<u8> = input
		.trim()
		.lines()
		.flat_map(|line| line.trim().as_bytes().iter().copied())
		.collect();

	let start = (0..height)
		.into_iter()
		.find_map(|y| {
			(0..width)
				.find(|x| grid[y * width + x] == b'S')
				.map(|x| (x, y))
		})
		.unwrap();
	let end = (0..height)
		.into_iter()
		.find_map(|y| {
			(0..width)
				.find(|x| grid[y * width + x] == b'E')
				.map(|x| (x, y))
		})
		.unwrap();
	grid[start.1 * width + start.0] = b'a';
	grid[end.1 * width + end.0] = b'z';

	let mut min = u64::MAX;
	for y in 0..height {
		for x in 0..width {
			if grid[y * width + x] == b'a' {
				if let Some(v) = find_shortest_path_from_to(&grid, (x, y), end, width, height) {
					if v < min {
						min = v;
					}
				}
			}
		}
	}
	min
}

fn find_shortest_path_from_to(
	grid: &[u8],
	start: (usize, usize),
	end: (usize, usize),
	width: usize,
	height: usize,
) -> Option<u64> {
	let mut queue = VecDeque::new();

	let mut visited = HashSet::new();

	queue.extend(
		get_adjacent(grid, width, height, start.0, start.1)
			.into_iter()
			.map(|x| (x, 1)),
	);

	while let Some((position, level)) = queue.pop_front() {
		if visited.get(&position).is_some() {
			continue;
		}

		let (x, y) = position;
		if position == end {
			return Some(level);
		} else {
			get_adjacent(grid, width, height, x, y)
				.into_iter()
				.for_each(|pos| queue.push_back((pos, level + 1)));
			visited.insert(position);
		}
	}

	None
}

fn get_adjacent(
	grid: &[u8],
	width: usize,
	height: usize,
	x: usize,
	y: usize,
) -> Vec<(usize, usize)> {
	let current = grid[y * width + x];
	let (width, height, x, y) = (width as i64, height as i64, x as i64, y as i64);
	[(0, 1), (0, -1), (1, 0), (-1, 0)]
		.into_iter()
		.map(|(dx, dy)| (x + dx, y + dy))
		.filter(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
		.map(|(x, y)| (x as usize, y as usize))
		.filter(|(x, y)| grid[*y * (width as usize) + *x].saturating_sub(current) <= 1)
		.collect()
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

	#[test]
	fn part_1() {
		assert_eq!(super::part_1(EXAMPLE), 31);
	}

	#[test]
	fn part_2() {
		assert_eq!(super::part_2(EXAMPLE), 29);
	}
}
