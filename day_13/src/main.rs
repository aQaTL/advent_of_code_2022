use anyhow::anyhow;
use itertools::Itertools;
use nom::sequence::delimited;
use nom::IResult;
use std::cmp::Ordering;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_13/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<usize> {
	let mut sum = 0;
	for (idx, (left, right)) in input
		.trim()
		.split("\n\n")
		.flat_map(|lines| lines.lines().tuples())
		.enumerate()
	{
		let (_, left) = parse_list(left).map_err(|err| anyhow!("{err:?}"))?;
		let (_, right) = parse_list(right).map_err(|err| anyhow!("{err:?}"))?;
		if left <= right {
			sum += idx + 1;
		}
	}
	Ok(sum)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
	let additional_packet_1 = List::List(vec![List::List(vec![List::Integer(2)])]);
	let additional_packet_2 = List::List(vec![List::List(vec![List::Integer(6)])]);

	let mut packets = Vec::new();
	for line in input.trim().lines().filter(|l| !l.is_empty()) {
		let (_, packet) = parse_list(line).map_err(|err| anyhow!("{err:?}"))?;
		packets.push(packet);
	}

	packets.push(additional_packet_1.clone());
	packets.push(additional_packet_2.clone());

	packets.sort();

	Ok((packets
		.iter()
		.position(|packet| packet == &additional_packet_1)
		.unwrap()
		+ 1) * (packets
		.iter()
		.position(|packet| packet == &additional_packet_2)
		.unwrap()
		+ 1))
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum List {
	List(Vec<List>),
	Integer(u64),
}

impl Ord for List {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl PartialOrd for List {
	fn partial_cmp(&self, right: &Self) -> Option<Ordering> {
		match (self, right) {
			(List::Integer(n), List::Integer(m)) => Some(n.cmp(m)),
			(List::List(l), List::List(l2)) => {
				let mut l_iter = l.iter();
				let mut l2_iter = l2.iter();
				loop {
					match (l_iter.next(), l2_iter.next()) {
						(Some(l_item), Some(l2_item)) => match l_item.partial_cmp(l2_item).unwrap()
						{
							Ordering::Equal => (),
							ordering => return Some(ordering),
						},
						(Some(_), None) => return Some(Ordering::Greater),
						(None, Some(_)) => return Some(Ordering::Less),
						(None, None) => return Some(Ordering::Equal),
					}
				}
			}
			(l @ List::List(_), List::Integer(n)) => {
				l.partial_cmp(&List::List(vec![List::Integer(*n)]))
			}
			(List::Integer(n), l @ List::List(_)) => {
				List::List(vec![List::Integer(*n)]).partial_cmp(l)
			}
		}
	}
}

fn parse_list(input: &str) -> IResult<&str, List> {
	use nom::branch::alt;
	use nom::bytes::complete::tag;
	use nom::character::complete::{char, u64};
	use nom::combinator::map;
	use nom::multi::separated_list0;

	delimited(
		char('['),
		map(
			separated_list0(tag(","), alt((map(u64, List::Integer), parse_list))),
			List::List,
		),
		char(']'),
	)(input)
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE).unwrap(), 13);
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(super::part_2(EXAMPLE).unwrap(), 140);
	}
}
