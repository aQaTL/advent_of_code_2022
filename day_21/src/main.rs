use nom::branch::alt;
use nom::bytes::complete::take;
use nom::multi::fold_many1;
use nom::sequence::{delimited, preceded};
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_21/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<f64> {
	let input = parse_input(input)?;

	Ok(eval(&input, "root"))
}

fn part_2(input: &str) -> anyhow::Result<u64> {
	let mut input = parse_input(input)?;

	let (left, right) = match &input["root"] {
		Monkey::Op(name1, name2, _) => (name1.to_string(), name2.to_string()),
		_ => panic!(),
	};

	let (side_with_humn, other_side) = if has_humn(&input, &left) {
		(left, right)
	} else {
		(right, left)
	};

	let other = eval(&input, &other_side);

	input.insert("humn".to_string(), Monkey::Number(0_f64));
	let result_for_0 = eval(&input, &side_with_humn);
	input.insert("humn".to_string(), Monkey::Number(u64::MAX as f64));
	let result_for_max = eval(&input, &side_with_humn);

	let (mut low, mut high) = (0, u64::MAX);
	while low != high {
		let mid = (low + high) / 2;
		input.insert("humn".to_string(), Monkey::Number(mid as f64));
		let result = eval(&input, &side_with_humn);

		if result == other {
			return Ok(mid);
		} else {
			if result_for_max < result_for_0 {
				if result > other {
					low = mid + 1;
				} else {
					high = mid - 1;
				}
			} else {
				if result < other {
					low = mid + 1;
				} else {
					high = mid - 1;
				}
			}
		}
	}

	Ok(low)
}

fn eval(input: &HashMap<String, Monkey>, name: &str) -> f64 {
	match &input[name] {
		Monkey::Number(v) => *v,
		Monkey::Op(name1, name2, op) => op.calc(eval(input, name1), eval(input, name2)),
	}
}

fn has_humn(input: &HashMap<String, Monkey>, name: &str) -> bool {
	match &input[name] {
		Monkey::Number(_) => false,
		Monkey::Op(name1, name2, _) if name1 == "humn" || name2 == "humn" => true,
		Monkey::Op(name1, name2, _) => has_humn(input, name1) || has_humn(input, name2),
	}
}

#[derive(Debug)]
enum Monkey {
	Number(f64),
	Op(String, String, Operation),
}

#[derive(Debug)]
enum Operation {
	Add,
	Multiply,
	Subtract,
	Divide,
}

impl Operation {
	fn calc(&self, a: f64, b: f64) -> f64 {
		match self {
			Operation::Add => a + b,
			Operation::Multiply => a * b,
			Operation::Subtract => a - b,
			Operation::Divide => a / b,
		}
	}
}

fn parse_input(
	input: &str,
) -> Result<HashMap<String, Monkey>, nom::Err<nom::error::Error<String>>> {
	use nom::bytes::complete::tag;
	use nom::character::complete::{char, i64, multispace0};
	use nom::combinator::{all_consuming, map};
	use nom::sequence::tuple;

	all_consuming(fold_many1(
		tuple((
			map(take(4_usize), |s: &str| s.to_string()),
			preceded(
				tag(": "),
				alt((
					map(i64, |x| Monkey::Number(x as f64)),
					map(
						tuple((
							map(take(4_usize), |s: &str| s.to_string()),
							delimited(
								char(' '),
								alt((
									map(char('+'), |_| Operation::Add),
									map(char('*'), |_| Operation::Multiply),
									map(char('-'), |_| Operation::Subtract),
									map(char('/'), |_| Operation::Divide),
								)),
								char(' '),
							),
							map(take(4_usize), |s: &str| s.to_string()),
						)),
						|(name1, op, name2)| Monkey::Op(name1, name2, op),
					),
				)),
			),
			multispace0,
		)),
		HashMap::new,
		|mut hm, (name, monkey, _)| {
			hm.insert(name, monkey);
			hm
		},
	))(input)
	.map(|(_input, res)| res)
	.map_err(|err: nom::Err<nom::error::Error<&str>>| err.to_owned())
}

#[cfg(test)]
mod tests {
	static EXAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE).unwrap(), 152.0);
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(super::part_2(EXAMPLE).unwrap(), 301);
	}
}
