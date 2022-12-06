use anyhow::anyhow;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_6/input.txt")?;
	println!("Part 1: {}", part_1(input.trim())?);
	println!("Part 2: {}", part_2(input.trim())?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<usize> {
	input
		.as_bytes()
		.windows(4)
		.enumerate()
		.find(|(_, window)| window.iter().duplicates().next().is_none())
		.ok_or_else(|| anyhow!("not found"))
		.map(|(idx, window)| idx + window.len())
}

fn part_2(input: &str) -> anyhow::Result<usize> {
	input
		.as_bytes()
		.windows(14)
		.enumerate()
		.find(|(_, window)| window.iter().duplicates().next().is_none())
		.ok_or_else(|| anyhow!("not found"))
		.map(|(idx, window)| idx + window.len())
}

#[cfg(test)]
mod tests {
	use super::{part_1, part_2};

	#[test]
	fn part_1_example() {
		assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 7);
		assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
		assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
		assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
		assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
	}

	#[test]
	fn part_2_example() {
		assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 19);
		assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 23);
		assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 23);
		assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 29);
		assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 26);
	}
}
