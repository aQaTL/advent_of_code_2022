use anyhow::anyhow;
use itertools::Itertools;
use nom::IResult;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_11/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<usize> {
	let (_, mut monkeys) = parse_input(input).map_err(|err| anyhow!("{err:?}"))?;

	for _ in 0..20 {
		for monkey_idx in 0..monkeys.len() {
			monkeys[monkey_idx].inspection_count += monkeys[monkey_idx].items.len();
			while let Some(item) = monkeys[monkey_idx].items.pop() {
				let monkey = &mut monkeys[monkey_idx];

				let worry_level =
					calc(item, monkey.operation, monkey.operand_a, monkey.operand_b) / 3;

				let pass_to = if worry_level % monkey.test_divisible_by == 0 {
					monkey.if_true_pass_to
				} else {
					monkey.if_false_pass_to
				};

				monkeys[pass_to].items.push(worry_level);
			}
		}
	}

	Ok(monkeys
		.iter()
		.map(|monkey| monkey.inspection_count)
		.sorted()
		.rev()
		.take(2)
		.product())
}

fn part_2(input: &str) -> anyhow::Result<usize> {
	let (_, mut monkeys) = parse_input(input).map_err(|err| anyhow!("{err:?}"))?;

	let lcm = monkeys.iter().map(|m| m.test_divisible_by).fold(1, lcm);

	for _ in 0..10_000 {
		for monkey_idx in 0..monkeys.len() {
			monkeys[monkey_idx].inspection_count += monkeys[monkey_idx].items.len();
			while let Some(item) = monkeys[monkey_idx].items.pop() {
				let monkey = &mut monkeys[monkey_idx];

				let worry_level = calc_mod(
					item,
					monkey.operation,
					monkey.operand_a,
					monkey.operand_b,
					lcm,
				);
				let pass_to = if worry_level % monkey.test_divisible_by == 0 {
					monkey.if_true_pass_to
				} else {
					monkey.if_false_pass_to
				};
				monkeys[pass_to].items.push(worry_level);
			}
		}
	}

	Ok(monkeys
		.iter()
		.map(|monkey| monkey.inspection_count)
		.sorted()
		.rev()
		.take(2)
		.product())
}

fn lcm(a: u64, b: u64) -> u64 {
	(a * b) / gcd(a, b)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
	while a != b {
		if a > b {
			a -= b
		} else {
			b -= a
		}
	}
	a
}

#[derive(Debug)]
struct Monkey {
	items: Vec<u64>,

	operand_a: Operand,
	operand_b: Operand,
	operation: Operation,

	test_divisible_by: u64,
	if_true_pass_to: usize,
	if_false_pass_to: usize,

	inspection_count: usize,
}

fn calc(old: u64, operation: Operation, operand_a: Operand, operand_b: Operand) -> u64 {
	let a = operand_a.number_or(old);
	let b = operand_b.number_or(old);
	match operation {
		Operation::Add => a + b,
		Operation::Multiply => a * b,
	}
}

fn calc_mod(
	old: u64,
	operation: Operation,
	operand_a: Operand,
	operand_b: Operand,
	lcm: u64,
) -> u64 {
	let a = operand_a.number_or(old);
	let b = operand_b.number_or(old);
	match operation {
		Operation::Add => ((a % lcm) + (b % lcm)) % lcm,
		Operation::Multiply => ((a % lcm) * (b % lcm)) % lcm,
	}
}

#[derive(Debug, Copy, Clone)]
enum Operand {
	Old,
	N(u64),
}

impl Operand {
	fn number_or(&self, other: u64) -> u64 {
		match self {
			Operand::Old => other,
			Operand::N(n) => *n,
		}
	}
}

#[derive(Debug, Copy, Clone)]
enum Operation {
	Add,
	Multiply,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
	use nom::branch::alt;
	use nom::bytes::complete::tag;
	use nom::character::complete::{char, multispace1, u64};
	use nom::combinator::map;
	use nom::multi::many1;
	use nom::multi::separated_list1;
	use nom::sequence::delimited;
	use nom::sequence::tuple;

	many1(map(
		tuple((
			delimited(
				tag("Monkey "),
				map(u64, |x| x as u64),
				tuple((char(':'), multispace1)),
			),
			delimited(
				tag("Starting items: "),
				separated_list1(tag(", "), u64),
				multispace1,
			),
			delimited(
				tag("Operation: new = "),
				tuple((
					alt((map(tag("old"), |_| Operand::Old), map(u64, Operand::N))),
					delimited(
						multispace1,
						alt((
							map(char('*'), |_| Operation::Multiply),
							map(char('+'), |_| Operation::Add),
						)),
						multispace1,
					),
					alt((map(tag("old"), |_| Operand::Old), map(u64, Operand::N))),
				)),
				multispace1,
			),
			delimited(tag("Test: divisible by "), u64, multispace1),
			delimited(
				tag("If true: throw to monkey "),
				map(u64, |n| n as usize),
				multispace1,
			),
			delimited(
				tag("If false: throw to monkey "),
				map(u64, |n| n as usize),
				multispace1,
			),
		)),
		|(
			_monkey_idx,
			starting_items,
			(operand_a, operation, operand_b),
			test_divisible_by,
			if_true_pass_to,
			if_false_pass_to,
		)| Monkey {
			items: starting_items,
			operand_a,
			operand_b,
			operation,
			test_divisible_by,
			if_true_pass_to,
			if_false_pass_to,
			inspection_count: 0,
		},
	))(input)
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE).unwrap(), 10605);
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(super::part_2(EXAMPLE).unwrap(), 2713310158);
	}
}
