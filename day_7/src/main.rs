use anyhow::{anyhow, bail};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::IResult;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_7/input.txt")?;
	let filesystem = parse_input(&input)?;
	println!("Part 1: {}", part_1(filesystem.clone())?);
	println!("Part 2: {}", part_2(filesystem)?);
	Ok(())
}

fn part_1(filesystem: Filesystem) -> anyhow::Result<u64> {
	let mut sum = 0;

	for (dir_name, nodes) in filesystem.graph.iter() {
		let mut nodes: Vec<_> = nodes
			.iter()
			.map(|node| match node {
				f @ Node::File { .. } => f.clone(),
				Node::Dir { name } => Node::Dir {
					name: push_dir(dir_name, name),
				},
			})
			.collect();

		let mut node_size = 0;
		while let Some(node) = nodes.pop() {
			match node {
				Node::File { size, .. } => node_size += size,
				Node::Dir { name } => {
					filesystem
						.graph
						.get(&name)
						.ok_or_else(|| anyhow!("{name} not found in the filesystem"))?
						.iter()
						.for_each(|node| {
							nodes.push(match node {
								f @ Node::File { .. } => f.clone(),
								Node::Dir { name: dir_name2 } => Node::Dir {
									name: push_dir(&name, dir_name2),
								},
							})
						});
				}
			}
			if node_size > 100000 {
				break;
			}
		}

		if node_size <= 100000 {
			sum += node_size;
		}
	}

	Ok(sum)
}

fn part_2(filesystem: Filesystem) -> anyhow::Result<u64> {
	const TOTAL_FILESYSTEM_SPACE: u64 = 70000000;
	const SPACE_NEEDED: u64 = 30000000;

	let used_space: u64 = filesystem
		.graph
		.values()
		.flat_map(|nodes| {
			nodes.iter().filter_map(|node| match node {
				Node::File { size, .. } => Some(size),
				Node::Dir { .. } => None,
			})
		})
		.sum();

	let space_to_cleanup = SPACE_NEEDED - (TOTAL_FILESYSTEM_SPACE - used_space);
	let mut size_of_dir_closest_to_needed_space_to_cleanup = u64::MAX;

	for (dir_name, nodes) in filesystem.graph.iter() {
		let mut nodes: Vec<_> = nodes
			.iter()
			.map(|node| match node {
				f @ Node::File { .. } => f.clone(),
				Node::Dir { name } => Node::Dir {
					name: push_dir(dir_name, name),
				},
			})
			.collect();

		let mut node_size = 0;
		while let Some(node) = nodes.pop() {
			match node {
				Node::File { size, .. } => node_size += size,
				Node::Dir { name } => {
					filesystem
						.graph
						.get(&name)
						.ok_or_else(|| anyhow!("{name} not found in the filesystem"))?
						.iter()
						.for_each(|node| {
							nodes.push(match node {
								f @ Node::File { .. } => f.clone(),
								Node::Dir { name: dir_name2 } => Node::Dir {
									name: push_dir(&name, dir_name2),
								},
							})
						});
				}
			}
		}

		if node_size >= space_to_cleanup
			&& node_size < size_of_dir_closest_to_needed_space_to_cleanup
		{
			size_of_dir_closest_to_needed_space_to_cleanup = node_size;
		}
	}

	Ok(size_of_dir_closest_to_needed_space_to_cleanup)
}

fn push_dir(stack: &str, dir: &str) -> String {
	if stack == "/" {
		format!("/{dir}")
	} else {
		format!("{stack}/{dir}")
	}
}

fn parse_input(input: &str) -> anyhow::Result<Filesystem> {
	parse_commands(input)
		.map_err(|err| anyhow!("{err:?}"))
		.map(|(_, commands)| commands)?
		.into_iter()
		.try_fold(Filesystem::new(), |mut filesystem, command| {
			match command {
				Command::Ls(nodes) => filesystem.push_nodes_to_current_dir(nodes),
				Command::Cd(dir) => filesystem.change_dir(dir)?,
			}
			Ok::<_, anyhow::Error>(filesystem)
		})
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
	use nom::character::complete::u64;
	many1(preceded(
		tag("$ "),
		alt((
			map(
				delimited(tag("cd "), take_till1(|c| c == '\n'), multispace0),
				|cmd: &str| Command::Cd(cmd.to_string()),
			),
			map(
				preceded(
					tag("ls"),
					delimited(
						multispace0,
						many0(alt((
							map(
								delimited(tag("dir "), take_till1(|c| c == '\n'), multispace0),
								|name: &str| Node::Dir {
									name: name.to_string(),
								},
							),
							map(
								terminated(
									separated_pair(u64, tag(" "), take_till1(|c| c == '\n')),
									multispace0,
								),
								|(size, name): (u64, &str)| Node::File {
									size,
									name: name.to_string(),
								},
							),
						))),
						multispace0,
					),
				),
				Command::Ls,
			),
		)),
	))(input)
}

#[derive(Debug)]
enum Command {
	Cd(String),
	Ls(Vec<Node>),
}

#[derive(Debug, Clone, Hash, PartialEq)]
enum Node {
	File { name: String, size: u64 },
	Dir { name: String },
}

#[derive(Debug, Clone)]
struct Filesystem {
	graph: HashMap<String, Vec<Node>>,
	stack: String,
}

impl Filesystem {
	fn new() -> Self {
		let mut graph = HashMap::new();
		graph.insert("/".to_string(), vec![]);
		Filesystem {
			graph,
			stack: "/".to_string(),
		}
	}

	fn change_dir(&mut self, dir: String) -> anyhow::Result<()> {
		match dir.as_str() {
			"/" => self.stack = dir,
			".." => {
				let (stack_dir, _) = self
					.stack
					.rsplit_once('/')
					.ok_or_else(|| anyhow!("failed to go up; current dir: {}", self.stack))?;
				self.stack = stack_dir.to_string();
			}
			_ => {
				let stack_dir = push_dir(&self.stack, &dir);
				if self.graph.get(&stack_dir).is_none() {
					bail!("dir {stack_dir} not found");
				}
				self.stack = stack_dir;
			}
		}
		Ok(())
	}

	fn push_nodes_to_current_dir(&mut self, nodes: Vec<Node>) {
		nodes.iter().for_each(|node| {
			if let Node::Dir { name } = node {
				self.graph.entry(push_dir(&self.stack, name)).or_default();
			}
		});
		let current_dir = self.graph.get_mut(&self.stack).unwrap();
		nodes.into_iter().for_each(|node| current_dir.push(node));
	}
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(
			super::part_1(super::parse_input(EXAMPLE).unwrap()).unwrap(),
			95437
		);
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(
			super::part_2(super::parse_input(EXAMPLE).unwrap()).unwrap(),
			24933642
		);
	}
}
