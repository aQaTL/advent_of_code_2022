use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_20/input.txt")?;
	println!("Part 1: {}", part_1(&input));
	println!("Part 2: {}", part_2(&input));
	Ok(())
}

fn part_1(input: &str) -> i32 {
	let original_list: Vec<i32> = input
		.trim()
		.lines()
		.map(|n| n.parse())
		.try_collect()
		.unwrap();

	let len = original_list.len() as i32;
	let mut list: Vec<_> = original_list.iter().copied().enumerate().collect();

	for (idx, _) in original_list.iter().enumerate() {
		let item_idx = list
			.iter()
			.position(|(original_idx, _)| *original_idx == idx)
			.unwrap();
		let item = list.remove(item_idx);
		let new_idx = (item_idx as i32 + item.1).rem_euclid(len - 1);
		list.insert(new_idx as usize, item);
	}

	let zero_pos = list.iter().position(|(_, v)| *v == 0).unwrap();
	[
		list[(zero_pos + 1000).rem_euclid(original_list.len())].1,
		list[(zero_pos + 2000).rem_euclid(original_list.len())].1,
		list[(zero_pos + 3000).rem_euclid(original_list.len())].1,
	]
	.into_iter()
	.sum()
}

fn part_2(input: &str) -> i64 {
	let original_list: Vec<i64> = input
		.trim()
		.lines()
		.map(|n| n.parse::<i64>().map(|x| x * 811589153))
		.try_collect()
		.unwrap();

	let len = original_list.len() as i64;
	let mut list: Vec<_> = original_list.iter().copied().enumerate().collect();

	for _ in 0..10 {
		for (idx, _) in original_list.iter().enumerate() {
			let item_idx = list
				.iter()
				.position(|(original_idx, _)| *original_idx == idx)
				.unwrap();
			let item = list.remove(item_idx);
			let new_idx = (item_idx as i64 + item.1).rem_euclid(len - 1);
			list.insert(new_idx as usize, item);
		}
	}

	let zero_pos = list.iter().position(|(_, v)| *v == 0).unwrap();
	[
		list[(zero_pos + 1000).rem_euclid(original_list.len())].1,
		list[(zero_pos + 2000).rem_euclid(original_list.len())].1,
		list[(zero_pos + 3000).rem_euclid(original_list.len())].1,
	]
	.into_iter()
	.sum()
}

#[cfg(test)]
mod tests {
	static EXAMPLE: &str = "1
2
-3
3
-2
0
4
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE), 3);
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(super::part_2(EXAMPLE), 1623178306);
	}
}
