use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_19/input.txt")?;
	println!("Part 1: {}", part_1(&input));
	println!("Part 2: {}", part_2(&input));
	Ok(())
}

fn part_1(input: &str) -> u32 {
	let blueprints = parse_input(input);
	let blueprints: Vec<_> = blueprints.into_iter().enumerate().collect();

	blueprints
		.into_par_iter()
		.map(|(idx, blueprint)| {
			let mut cache = vec![Default::default(); 24 + 1];
			let mut cache2 = HashMap::new();
			(idx as u32 + 1)
				* dfs(
					&blueprint,
					24,
					Resources::default(),
					&mut cache,
					&mut cache2,
				)
		})
		.sum()
}

fn part_2(input: &str) -> u32 {
	let blueprints = parse_input(input);
	let blueprints: Vec<_> = blueprints.into_iter().take(3).collect();

	blueprints
		.into_par_iter()
		.map(|blueprint| {
			let mut cache = vec![Default::default(); 32 + 1];
			let mut cache2 = HashMap::new();
			dfs(
				&blueprint,
				32,
				Resources::default(),
				&mut cache,
				&mut cache2,
			)
		})
		.product()
}

fn dfs(
	bp: &Blueprint,
	minute: u32,
	mut resources: Resources,
	cache: &mut Vec<Option<u32>>,
	cache2: &mut HashMap<(Resources, u32), u32>,
) -> u32 {
	if minute == 1 {
		return resources.geodes + resources.geode_cracking_robots;
	}

	if resources.geodes
		+ (0..minute)
			.map(|x| x + resources.geode_cracking_robots)
			.sum::<u32>()
		< cache[minute as usize].unwrap_or_default()
	{
		return 0;
	}

	if let Some(v) = cache2.get(&(resources, minute)) {
		return *v;
	}

	if resources.ores >= bp.geode_robot_ore_cost
		&& resources.obsidian >= bp.geode_robot_obsidian_cost
	{
		let mut resources = resources.clone();
		resources.ores -= bp.geode_robot_ore_cost;
		resources.obsidian -= bp.geode_robot_obsidian_cost;

		resources.ores += resources.ore_collecting_robots;
		resources.clays += resources.clay_collecting_robots;
		resources.obsidian += resources.obsidian_collecting_robots;
		resources.geodes += resources.geode_cracking_robots;

		resources.geode_cracking_robots += 1;

		let v = dfs(bp, minute - 1, resources, cache, cache2);
		return v;
	}

	let mut max = 0;

	if resources.ores >= bp.ore_robot_cost
		&& resources.ore_collecting_robots * minute
			<= bp.most_expensive_robot_ore_cost * minute - resources.ores
	{
		let mut resources = resources.clone();
		resources.ores -= bp.ore_robot_cost;

		resources.ores += resources.ore_collecting_robots;
		resources.clays += resources.clay_collecting_robots;
		resources.obsidian += resources.obsidian_collecting_robots;
		resources.geodes += resources.geode_cracking_robots;

		resources.ore_collecting_robots += 1;

		max = max.max(dfs(bp, minute - 1, resources, cache, cache2));
	}

	if resources.ores >= bp.clay_robot_cost
		&& resources.clay_collecting_robots * minute
			<= bp.obsidian_robot_clay_cost * minute - resources.clays
	{
		let mut resources = resources.clone();
		resources.ores -= bp.clay_robot_cost;

		resources.ores += resources.ore_collecting_robots;
		resources.clays += resources.clay_collecting_robots;
		resources.obsidian += resources.obsidian_collecting_robots;
		resources.geodes += resources.geode_cracking_robots;

		resources.clay_collecting_robots += 1;

		max = max.max(dfs(bp, minute - 1, resources, cache, cache2));
	}

	if resources.ores >= bp.obsidian_robot_ore_cost
		&& resources.clays >= bp.obsidian_robot_clay_cost
		&& resources.obsidian_collecting_robots * minute
			<= bp.geode_robot_obsidian_cost * minute - resources.obsidian
	{
		let mut resources = resources.clone();
		resources.ores -= bp.obsidian_robot_ore_cost;
		resources.clays -= bp.obsidian_robot_clay_cost;

		resources.ores += resources.ore_collecting_robots;
		resources.clays += resources.clay_collecting_robots;
		resources.obsidian += resources.obsidian_collecting_robots;
		resources.geodes += resources.geode_cracking_robots;

		resources.obsidian_collecting_robots += 1;

		max = max.max(dfs(bp, minute - 1, resources, cache, cache2));
	}

	if (resources.geode_cracking_robots == 0) || max == 0 {
		resources.ores += resources.ore_collecting_robots;
		resources.clays += resources.clay_collecting_robots;
		resources.obsidian += resources.obsidian_collecting_robots;
		resources.geodes += resources.geode_cracking_robots;

		max = max.max(dfs(bp, minute - 1, resources, cache, cache2));
	}

	cache2.insert((resources, minute), max);
	cache[minute as usize] = Some(max);
	max
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Resources {
	ore_collecting_robots: u32,
	clay_collecting_robots: u32,
	obsidian_collecting_robots: u32,
	geode_cracking_robots: u32,

	ores: u32,
	clays: u32,
	obsidian: u32,
	geodes: u32,
}

impl Default for Resources {
	fn default() -> Self {
		Resources {
			ore_collecting_robots: 1,
			clay_collecting_robots: 0,
			obsidian_collecting_robots: 0,
			geode_cracking_robots: 0,
			ores: 0,
			clays: 0,
			obsidian: 0,
			geodes: 0,
		}
	}
}

#[derive(Debug)]
struct Blueprint {
	ore_robot_cost: u32,
	clay_robot_cost: u32,
	obsidian_robot_ore_cost: u32,
	obsidian_robot_clay_cost: u32,
	geode_robot_ore_cost: u32,
	geode_robot_obsidian_cost: u32,

	most_expensive_robot_ore_cost: u32,
}

fn parse_input(input: &str) -> Vec<Blueprint> {
	Regex::new(
		"Blueprint \\d+: Each ore robot costs (\\d+) ore. Each clay robot costs (\\d+) ore. Each \
		obsidian robot costs (\\d+) ore and (\\d+) clay. Each geode robot costs (\\d+) ore and \
		(\\d+) obsidian.",
	)
	.unwrap()
	.captures_iter(input)
	.map(|cap| Blueprint {
		ore_robot_cost: cap.get(1).unwrap().as_str().parse().unwrap(),
		clay_robot_cost: cap.get(2).unwrap().as_str().parse().unwrap(),
		obsidian_robot_ore_cost: cap.get(3).unwrap().as_str().parse().unwrap(),
		obsidian_robot_clay_cost: cap.get(4).unwrap().as_str().parse().unwrap(),
		geode_robot_ore_cost: cap.get(5).unwrap().as_str().parse().unwrap(),
		geode_robot_obsidian_cost: cap.get(6).unwrap().as_str().parse().unwrap(),
		most_expensive_robot_ore_cost: 0,
	})
	.map(|mut bp| {
		bp.most_expensive_robot_ore_cost = bp
			.ore_robot_cost
			.max(bp.clay_robot_cost)
			.max(bp.obsidian_robot_ore_cost)
			.max(bp.geode_robot_ore_cost);
		bp
	})
	.collect()
}

#[cfg(test)]
mod tests {
	use regex::Regex;

	static EXAMPLE: &str = "Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.
";

	#[test]
	fn part_1_example_1() -> anyhow::Result<()> {
		let example = Regex::new(r"\n([\s&&[^\n]]+)")?.replace_all(EXAMPLE, r" ");
		let example = Regex::new(r"\n\n")?.replace_all(&example, r"\n");
		assert_eq!(super::part_1(&example), 33);
		Ok(())
	}
}
