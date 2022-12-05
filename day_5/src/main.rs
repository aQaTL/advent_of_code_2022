use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alpha1, multispace0, newline, not_line_ending};
use nom::combinator::{map, map_parser, opt};
use nom::multi::{many1, many_till};
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_5/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<String> {
	let (_, Input { mut stacks, steps }) = parse_input(input).map_err(|err| anyhow!("{err:?}"))?;

	for step in steps {
		let (source_stack, destination_stack) =
			get_2_mut_unchecked(&mut stacks, step.source_idx, step.destination_idx);

		let source_stack_len = source_stack.len();
		let removed_stack_iter = source_stack.drain((source_stack_len - step.count)..).rev();
		destination_stack.extend(removed_stack_iter);
	}

	Ok(stacks
		.iter()
		.filter_map(|stack| stack.last())
		.collect::<String>())
}

fn part_2(input: &str) -> anyhow::Result<String> {
	let (_, Input { mut stacks, steps }) = parse_input(input).map_err(|err| anyhow!("{err:?}"))?;

	for step in steps {
		let (source_stack, destination_stack) =
			get_2_mut_unchecked(&mut stacks, step.source_idx, step.destination_idx);

		let source_stack_len = source_stack.len();
		let removed_stack_iter = source_stack.drain((source_stack_len - step.count)..);
		destination_stack.extend(removed_stack_iter);
	}

	Ok(stacks
		.iter()
		.filter_map(|stack| stack.last())
		.collect::<String>())
}

fn get_2_mut_unchecked<T>(slice: &mut [T], a_idx: usize, b_idx: usize) -> (&mut T, &mut T) {
	let ptr = slice.as_mut_ptr();
	unsafe {
		let a: &mut T = &mut *ptr.add(a_idx);
		let b: &mut T = &mut *ptr.add(b_idx);
		(a, b)
	}
}

struct Input {
	stacks: Vec<Vec<char>>,
	steps: Vec<Step>,
}

struct Step {
	source_idx: usize,
	destination_idx: usize,
	count: usize,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
	let (input, stacks) = parse_stacks(input)?;
	let (input, _) = multispace0(input)?;
	let (input, steps) = parse_steps(input)?;
	Ok((input, Input { stacks, steps }))
}

fn parse_stacks(mut input: &str) -> IResult<&str, Vec<Vec<char>>> {
	let mut stacks: Vec<Vec<char>> = Vec::new();
	loop {
		let result: IResult<_, Vec<Option<char>>> = map(
			many_till(
				terminated(
					alt((
						map(tag("   "), |_| None),
						map(
							delimited(tag("["), map_parser(take(1usize), alpha1), tag("]")),
							|c: &str| Some(c.as_bytes()[0] as u8 as char),
						),
					)),
					opt(tag(" ")),
				),
				newline,
			),
			|(stack, _)| stack,
		)(input);

		match result {
			Ok((tail, line)) => {
				input = tail;
				for (idx, char) in line.into_iter().enumerate() {
					let Some(char) = char else {
						continue;
					};
					match stacks.get_mut(idx) {
						Some(stack) => stack.push(char),
						None => {
							(stacks.len()..=idx).for_each(|_| stacks.push(Vec::new()));
							stacks.get_mut(idx).unwrap().push(char);
						}
					}
				}
			}
			Err(_) => break,
		}
	}

	let (input, _) = terminated(not_line_ending, newline)(input)?;
	let (input, _) = terminated(not_line_ending, newline)(input)?;

	let stacks = stacks
		.into_iter()
		.map(|stack| stack.into_iter().rev().collect::<Vec<_>>())
		.collect();

	Ok((input, stacks))
}

fn parse_steps(input: &str) -> IResult<&str, Vec<Step>> {
	use nom::character::complete::u32;

	many1(terminated(
		map(
			tuple((
				tag("move "),
				map(u32, |v| v as usize),
				tag(" from "),
				map(u32, |v| (v - 1) as usize),
				tag(" to "),
				map(u32, |v| (v - 1) as usize),
			)),
			|(_, count, _, source_idx, _, destination_idx)| Step {
				source_idx,
				destination_idx,
				count,
			},
		),
		multispace0,
	))(input)
}

#[cfg(test)]
mod tests {
	use crate::{part_1, part_2};

	const EXAMPLE: &str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

	#[test]
	fn part_1_example_1() {
		assert_eq!(&part_1(EXAMPLE).unwrap(), "CMZ");
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(&part_2(EXAMPLE).unwrap(), "MCD");
	}
}
