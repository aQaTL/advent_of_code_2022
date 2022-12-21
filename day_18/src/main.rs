use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_18/input.txt")?;
	println!("Part 1: {}", part_1(input.trim()));
	println!("Part 2: {}", part_2(input.trim()));
	Ok(())
}

static SIDES: [(i64, i64, i64); 6] = [
	(1, 0, 0),
	(-1, 0, 0),
	(0, 1, 0),
	(0, -1, 0),
	(0, 0, 1),
	(0, 0, -1),
];

fn part_1(input: &str) -> usize {
	let mut cubes = HashSet::<(i64, i64, i64)>::new();
	for line in input.lines() {
		let cube = line
			.split(',')
			.map(|n| n.parse().unwrap())
			.next_tuple()
			.unwrap();
		cubes.insert(cube);
	}

	cubes
		.iter()
		.flat_map(|(x, y, z)| {
			SIDES
				.iter()
				.map(|(dx, dy, dz)| (*x + *dx, *y + *dy, *z + *dz))
		})
		.filter(|cube| !cubes.contains(cube))
		.count()
}

fn part_2(input: &str) -> u64 {
	let mut cubes = HashSet::<(i64, i64, i64)>::new();
	for line in input.lines() {
		let cube = line
			.split(',')
			.map(|n| n.parse().unwrap())
			.next_tuple()
			.unwrap();
		cubes.insert(cube);
	}

	let [mut min_x, mut min_y, mut min_z] = [i64::MAX; 3];
	let [mut max_x, mut max_y, mut max_z] = [0; 3];
	cubes.iter().for_each(|(x, y, z)| {
		min_x = min_x.min(*x);
		min_y = min_y.min(*y);
		min_z = min_z.min(*z);
		max_x = max_x.max(*x);
		max_y = max_y.max(*y);
		max_z = max_z.max(*z);
	});

	min_x -= 1;
	min_y -= 1;
	min_z -= 1;
	max_x += 1;
	max_y += 1;
	max_z += 1;

	let mut q = VecDeque::new();
	q.push_back((min_x, min_y, min_z));

	let mut visited = HashSet::new();
	let mut surface_area = 0;

	while let Some(cube @ (x, y, z)) = q.pop_front() {
		if visited.contains(&cube) {
			continue;
		}
		visited.insert(cube);

		for (dx, dy, dz) in &SIDES {
			let cube2 = (x + dx, y + dy, z + dz);

			if cube2.0 < min_x
				|| cube2.0 > max_x
				|| cube2.1 < min_y
				|| cube2.1 > max_y
				|| cube2.2 < min_z
				|| cube2.2 > max_z
			{
				continue;
			}

			if cubes.contains(&cube2) {
				surface_area += 1;
			} else {
				q.push_back(cube2);
			}
		}
	}

	surface_area
}

#[cfg(test)]
mod tests {
	static EXAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE), 64);
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(super::part_2(EXAMPLE), 58);
	}
}
