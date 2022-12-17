#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        error::Error,
    };

    use itertools::Itertools;

    use crate::common::tests::{get_input, get_sample_input};

    type NodeId = String;

    struct Graph {
        nodes: HashSet<String>,
        connections: HashMap<String, Vec<(String, usize)>>,
        rates: HashMap<String, usize>,
        valve_count: usize,
    }

    impl Graph {
        fn neighbours(&self, node: &str) -> &Vec<(String, usize)> {
            let r = self.connections.get(node).unwrap();
            r
        }

        fn rate_of(&self, node: &str) -> usize {
            self.rates.get(node).map_or(0, |r| r.clone())
        }
    }

    const MAX_COST: usize = 30;

    #[test]
    fn day16() -> Result<(), Box<dyn Error>> {
        //Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let regex = regex::Regex::new(
            "Valve (?P<name>.*) has flow rate=(?P<rate>\\d+); tunnel(s?) lead(s?) to valve(s?) (?P<connections>.*)"
        )?;

        let input = get_input(16)?;

        let input = get_sample_input(16)?;

        let mut names = HashSet::new();
        let mut rates = HashMap::new();
        let mut connections = HashMap::new();
        let mut valve_count = 0;

        for line in input.lines() {
            let cap = regex.captures(line).unwrap();
            let name = cap.name("name").unwrap().as_str();
            let rate: usize = cap.name("rate").unwrap().as_str().parse()?;
            let mut c = cap
                .name("connections")
                .unwrap()
                .as_str()
                .split(", ")
                .into_iter()
                .map(|s| (s.to_string(), 1))
                .collect_vec();


            // println!(
            //     "  {name} [fontsize={} label=\"{name} {rate}\"]",
            //     1 + 3 * rate
            // );
            // // println!("  \"{name}'\" [fontsize={} label=\"{name}' {rate}\"]", 1 + 3*rate );
            // for c in &c {
            // // println!("  {name} -> \"{name}'\"");
            // println!("  {name} -> {}", c);
            // // println!("  \"{name}'\" -> {}", c);
            // }

            names.insert(name.to_string());

            if rate > 0 {
                let opened_name = name.to_string() + "'";
                rates.insert(opened_name.clone(), rate);
                names.insert(opened_name.clone());
                connections.insert(opened_name.clone(), vec![(name.to_string(), 0)]);
                c.insert(0, (opened_name.clone(), 1));
                valve_count += 1;
            }
            connections.insert(name.to_string(), c.clone());
        }

        let graph = Graph {
            nodes: names,
            connections,
            rates,
            valve_count,
        };

        let mut path = Vec::with_capacity(60);
        path.push("AA".to_string());
        let mut visited_nodes = HashSet::with_capacity(graph.nodes.len() * 2);
        visited_nodes.insert("AA".to_string());
        let mut visited_tunnels = HashSet::with_capacity(500);

        let mut best_rate = 0;
        let mut best_path = Vec::with_capacity(60);

        explore_path(
            &mut path,
            0,
            0,
            0,
            &mut visited_nodes,
            &mut visited_tunnels,
            &graph,
            &mut best_rate,
            &mut best_path,
        );

        println!("Best flow: {best_rate}");
        println!("Best path: {best_path:?}");

        Ok(())
    }

    fn explore_path(
        current_path: &mut Vec<String>,
        current_cost: usize,
        current_pressure_reduction: usize,
        open_valves: usize,
        visited_nodes: &mut HashSet<String>,
        visited_tunnels: &mut HashSet<(String, String)>,
        graph: &Graph,
        best_reduction_so_far: &mut usize,
        best_path_so_far: &mut Vec<String>,
    ) {
        println!("exploring after {current_path:?}");
        let current = current_path.last().unwrap().as_str();
        let t = current.to_string();

        if &current_pressure_reduction > best_reduction_so_far {
            *best_reduction_so_far = current_pressure_reduction;
            *best_path_so_far = current_path.clone();
            println!(
                "new best path: {best_reduction_so_far}, {}:  {best_path_so_far:?}",
                best_path_so_far.len()
            );
        }
        if current_cost >= MAX_COST {
            //time's up,
            return;
        }

        let neighbours = graph.neighbours(current);
        let mut visit_count = 0;
        for neighbour in neighbours {
            let (neighbour, cost) = neighbour;

            let cost_to_go_to_neighbor = cost;
            if open_valves >= graph.valve_count {
                continue;
            }
            if (neighbour.ends_with("'") && visited_nodes.contains(neighbour))
                || (visited_tunnels.contains(&(t.clone(), neighbour.to_string()))
                    && visited_tunnels.contains(&(neighbour.to_string(), t.clone())))
            {
                continue;
            }

            visit_count += 1;

            // let t = current.to_string();
            // println!("{t} -> {neighbour}, current cost: {current_cost}, current reduction: {current_pressure_reduction}");
            // let start = if t.ends_with("'") {(&t[0..t.len()-1]).to_string()} else {t.clone()};
            visited_tunnels.insert((t.clone(), neighbour.to_string()));
            visited_nodes.insert(neighbour.to_string());
            current_path.push(neighbour.to_string());
            let neighbour_rate = graph.rate_of(neighbour);
            if current_cost + cost_to_go_to_neighbor > MAX_COST {
                continue;
            }
            let reduction = (MAX_COST - current_cost - cost_to_go_to_neighbor) * neighbour_rate;
            let valve_inc = if neighbour.ends_with("'") { 1 } else { 0 };
            explore_path(
                current_path,
                current_cost + cost_to_go_to_neighbor,
                current_pressure_reduction + reduction,
                open_valves + valve_inc,
                visited_nodes,
                visited_tunnels,
                graph,
                best_reduction_so_far,
                best_path_so_far,
            );
            visited_tunnels.remove(&(t.clone(), neighbour.to_string()));
            visited_nodes.remove(neighbour);
            current_path.pop();
        }

        if visit_count == 0 {
            // println!("Blocked!");
            return;
        }
    }
}
