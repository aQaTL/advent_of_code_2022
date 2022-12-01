use anyhow::anyhow;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_1/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<u64> {
	input
		.split("\n\n")
		.map(|elf| {
			elf.split('\n')
				.filter_map(|num| num.parse::<u64>().ok())
				.sum()
		})
		.max()
		.ok_or_else(|| anyhow!("malformed input"))
}

fn part_2(input: &str) -> anyhow::Result<u64> {
	let top_three_sum = input
		.split("\n\n")
		.map(|elf| {
			elf.split('\n')
				.filter_map(|num| num.parse::<u64>().ok())
				.sum::<u64>()
		})
		.sorted()
		.rev()
		.take(3)
		.sum();

	Ok(top_three_sum)
}
