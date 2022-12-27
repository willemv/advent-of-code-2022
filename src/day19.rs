#[cfg(test)]
mod tests {
    use std::{
        collections::{HashSet, VecDeque},
        error::Error,
    };

    use regex::Regex;

    use crate::common::tests::{get_input, get_sample_input};

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    struct State {
        minutes_left: u64,
        ore: u64,
        clay: u64,
        obsidian: u64,
        geodes: u64,

        ore_rate: u64,
        clay_rate: u64,
        obsidian_rate: u64,
        geode_rate: u64,
    }

    #[test]
    fn day19() -> Result<(), Box<dyn Error>> {
        // let input = get_sample_input(19)?;
        let input = get_input(19)?;

        // robots collect resources at 1 per minute
        // factory takes 1 minute to create robot
        // start: 1 ore-collecting robot
        // max number of geodes opened after 24 minutes

        let regex = Regex::new("Blueprint \\d+: Each ore robot costs (?P<ore_ore>\\d+) ore. Each clay robot costs (?P<clay_ore>\\d+) ore. Each obsidian robot costs (?P<obsidian_ore>\\d+) ore and (?P<obsidian_clay>\\d+) clay. Each geode robot costs (?P<geode_ore>\\d+) ore and (?P<geode_obsidian>\\d+) obsidian.")?;

        let mut quality_sum = 0;

        for (i, blueprint) in input.lines().enumerate().take(3) {
            let index = (i + 1) as u64;
            let captures = regex.captures(blueprint).unwrap();

            let ore_ore: u64 = captures.name("ore_ore").unwrap().as_str().parse()?;
            let clay_ore: u64 = captures.name("clay_ore").unwrap().as_str().parse()?;
            let obsidian_ore: u64 = captures.name("obsidian_ore").unwrap().as_str().parse()?;
            let obsidian_clay: u64 = captures.name("obsidian_clay").unwrap().as_str().parse()?;
            let geode_ore: u64 = captures.name("geode_ore").unwrap().as_str().parse()?;
            let geode_obsidian: u64 = captures.name("geode_obsidian").unwrap().as_str().parse()?;


            let max_ore_rate_needed = ore_ore.max(clay_ore).max(obsidian_ore).max(geode_ore);
            let max_clay_rate_needed = obsidian_clay;
            let max_obsidian_rate_needed = geode_obsidian;

            // the production of robots competes on ore for each robot, and nothing else.

            println!("{blueprint}:\n{ore_ore}, {clay_ore}, {obsidian_ore}, {obsidian_clay}, {geode_ore}, {geode_obsidian}");

            let state = State {
                minutes_left: 32,
                ore: 0,
                clay: 0,
                obsidian: 0,
                geodes: 0,

                ore_rate: 1,
                clay_rate: 0,
                obsidian_rate: 0,
                geode_rate: 0,
            };

            let mut frontier = VecDeque::new();
            frontier.push_back(state);
            let mut visited_states = HashSet::new();

            let mut max_geodes = 0;
            while !frontier.is_empty() {
                let current = frontier.pop_front().unwrap();
                if visited_states.contains(&current) {
                    // println!("pruned state that we already visited");
                    continue;
                } else {
                    // println!("new state");
                    visited_states.insert(current);
                }

                if current.minutes_left == 0 {
                    let score = current.geodes;
                    // println!("Bottom! Current: {current:?}");
                    max_geodes = max_geodes.max(score);
                } else {
                    let minutes_left = current.minutes_left - 1;
                    // the state where we don't build any robots
                    frontier.push_back(State {
                        minutes_left,
                        ore: current.ore + current.ore_rate,
                        clay: current.clay + current.clay_rate,
                        obsidian: current.obsidian + current.obsidian_rate,
                        geodes: current.geodes + current.geode_rate,

                        ..current
                    });

                    // are there any machines we can and should build?
                    if current.ore >= geode_ore && current.obsidian >= geode_obsidian {
                        frontier.push_back(State {
                            minutes_left,
                            ore: current.ore + current.ore_rate - geode_ore,
                            clay: current.clay + current.clay_rate,
                            obsidian: current.obsidian + current.obsidian_rate - geode_obsidian,
                            geodes: current.geodes + current.geode_rate,

                            geode_rate: current.geode_rate + 1,
                            ..current
                        });
                        continue; // if you can build an obsidian bot, assume that this is the best option
                    }

                    if current.obsidian_rate < max_obsidian_rate_needed && current.ore >= obsidian_ore && current.clay >= obsidian_clay {
                        frontier.push_back(State {
                            minutes_left,
                            ore: current.ore + current.ore_rate - obsidian_ore,
                            clay: current.clay + current.clay_rate - obsidian_clay,
                            obsidian: current.obsidian + current.obsidian_rate,
                            geodes: current.geodes + current.geode_rate,

                            obsidian_rate: current.obsidian_rate + 1,
                            ..current
                        });

                        continue; // same here?
                    }

                    if current.ore_rate < max_ore_rate_needed && current.ore >= ore_ore {
                        frontier.push_back(State {
                            minutes_left,
                            ore: current.ore + current.ore_rate - ore_ore,
                            clay: current.clay + current.clay_rate,
                            obsidian: current.obsidian + current.obsidian_rate,
                            geodes: current.geodes + current.geode_rate,

                            ore_rate: current.ore_rate + 1,
                            ..current
                        });
                    }

                    if current.clay_rate < max_clay_rate_needed && current.ore >= clay_ore {
                        frontier.push_back(State {
                            minutes_left,
                            ore: current.ore + current.ore_rate - clay_ore,
                            clay: current.clay + current.clay_rate,
                            obsidian: current.obsidian + current.obsidian_rate,
                            geodes: current.geodes + current.geode_rate,

                            clay_rate: current.clay_rate + 1,
                            ..current
                        });
                    }
                }
            }

            println!(
                "max geodes for blueprint {index}: {max_geodes} (quality level: {})",
                index * max_geodes
            );
            quality_sum += index * max_geodes;
        }
        println!("sum of quality levels: {quality_sum}");
        Ok(())
    }
}
