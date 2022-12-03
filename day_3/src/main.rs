use anyhow::anyhow;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_3/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<u64> {
	let mut sum_of_priorities: u64 = 0;

	for line in input.lines().map(str::as_bytes) {
		let first_compartment = &line[..line.len() / 2];
		let second_compartment = &line[line.len() / 2..];

		let shared_item = first_compartment
			.iter()
			.find(|x| second_compartment.contains(x))
			.ok_or_else(|| anyhow!("invalid input"))?;

		if shared_item.is_ascii_lowercase() {
			sum_of_priorities += (*shared_item - b'a' + 1) as u64;
		} else if shared_item.is_ascii_uppercase() {
			sum_of_priorities += (*shared_item - b'A' + 27) as u64;
		}
	}

	Ok(sum_of_priorities)
}

fn part_2(input: &str) -> anyhow::Result<u64> {
	let mut sum_of_priorities: u64 = 0;

	for (a, b, c) in input.lines().map(str::as_bytes).tuples() {
		let shared_item = a
			.iter()
			.find(|a_elem| b.contains(a_elem) && c.contains(a_elem))
			.ok_or_else(|| anyhow!("didn't find a common item"))?;

		if shared_item.is_ascii_lowercase() {
			sum_of_priorities += (*shared_item - b'a' + 1) as u64;
		} else if shared_item.is_ascii_uppercase() {
			sum_of_priorities += (*shared_item - b'A' + 27) as u64;
		}
	}

	Ok(sum_of_priorities)
}
