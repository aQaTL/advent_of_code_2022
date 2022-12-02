#![allow(clippy::identity_op)]

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_2/input.txt")?;
	println!("Part 1: {}", part_1(&input));
	println!("Part 2: {}", part_2(&input));
	Ok(())
}

fn part_1(input: &str) -> u64 {
	let mut total = 0;

	for line in input
		.split('\n')
		.filter(|l| !l.is_empty())
		.map(str::as_bytes)
	{
		let opponent_move = line[0];
		let my_move = line[2];

		total += match (opponent_move, my_move) {
			(b'A', b'X') => 1 + 3,
			(b'A', b'Y') => 2 + 6,
			(b'A', b'Z') => 3 + 0,

			(b'B', b'X') => 1 + 0,
			(b'B', b'Y') => 2 + 3,
			(b'B', b'Z') => 3 + 6,

			(b'C', b'X') => 1 + 6,
			(b'C', b'Y') => 2 + 0,
			(b'C', b'Z') => 3 + 3,

			_ => panic!("{} {}", opponent_move as char, my_move as char),
		};
	}

	total
}

fn part_2(input: &str) -> u64 {
	let mut total = 0;

	for line in input
		.split('\n')
		.filter(|l| !l.is_empty())
		.map(str::as_bytes)
	{
		let opponent_move = line[0];
		let result = line[2];

		total += match (opponent_move, result) {
			(b'A', b'X') => 3 + 0,
			(b'A', b'Y') => 1 + 3,
			(b'A', b'Z') => 2 + 6,

			(b'B', b'X') => 1 + 0,
			(b'B', b'Y') => 2 + 3,
			(b'B', b'Z') => 3 + 6,

			(b'C', b'X') => 2 + 0,
			(b'C', b'Y') => 3 + 3,
			(b'C', b'Z') => 1 + 6,

			_ => panic!("{} {}", opponent_move as char, result as char),
		};
	}

	total
}
