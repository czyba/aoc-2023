use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

struct Graph {
    nodes: Vec<Node>
}

struct Node {
    name: u32,
    connections: HashSet<u32>,
}

impl Graph {

    fn reachable(&self, start: u32) -> usize {
        let mut seen = HashSet::new();
        seen.insert(start);
        let mut  queue = VecDeque::with_capacity(self.nodes.len());
        queue.push_back(start);
        while let Some(n) = queue.pop_front() {
            for conn in &self.nodes[n as usize].connections {
                if seen.insert(*conn) {
                    queue.push_back(*conn);
                }
            }
        }
        seen.len()
    }

    fn shortest_path(&self, start: u32, end: u32) -> Option<Vec<(u32, u32)>> {
        let mut predecessors = vec![None; self.nodes.len()];
        predecessors[start as usize] = Some(start);

        let mut queue = VecDeque::with_capacity(self.nodes.len());
        queue.push_back(start);
        while let Some(n) = queue.pop_front() {
            for succ in &self.nodes[n as usize].connections {
                if predecessors[*succ as usize].is_some() {
                    continue;
                }
                predecessors[*succ as usize] = Some(n);
                queue.push_back(*succ);
                if *succ == end {
                    break;
                }
            }
        }

        predecessors[start as usize] = None;

        if predecessors[end as usize].is_none() {
            return None;
        }

        let mut last_node = end;
        let mut r = Vec::new();

        while let Some(pred) = predecessors[last_node as usize] {
            r.push((last_node, pred));
            last_node = pred;
        }
        Some(r)
    }

    fn remove_edge(&mut self, e: (u32, u32)) {
        self.nodes[e.0 as usize].connections.remove(&e.1);
        self.nodes[e.1 as usize].connections.remove(&e.0);
    }

    fn remove_edges(&mut self, edges: &Vec<(u32, u32)>) {
        for e in edges {
            self.remove_edge(*e);
        }
    }

    fn insert_edge(&mut self, e: (u32, u32)) {
        self.nodes[e.0 as usize].connections.insert(e.1);
        self.nodes[e.1 as usize].connections.insert(e.0);
    }

    fn insert_edges(&mut self, edges: &Vec<(u32, u32)>) {
        for e in edges {
            self.insert_edge(*e);
        }
    }

}

fn parse() -> Graph {
    let node_list: Vec<(String, Vec<String>)> = lines_from_file("src/day25.txt")
        .unwrap()
        .map(|line| parse_line(&line))
        .collect();

    let mut hm : HashMap<String, u32> = HashMap::new();
    let all_nodes = node_list.iter().flat_map(|(n, conns)| std::iter::once(n).chain(conns.iter()));
    let mut num = 0;
    let mut g = Graph { nodes: Vec::with_capacity(hm.len()) };
    for node in all_nodes {
        if let None = hm.get(node) {
            hm.insert(node.to_owned(), num);
            g.nodes.push(Node {
                name: num,
                connections: HashSet::new(),
            });
            num += 1;
        }
    }

    for (n, succs) in node_list {
        let node_num = hm.get(&n).unwrap();
        for succ in succs {
            let other_node_num = hm.get(&succ).unwrap();
            g.nodes[*node_num as usize].connections.insert(*other_node_num);
            g.nodes[*other_node_num as usize].connections.insert(*node_num);
        }
    }
    g

}

fn parse_line(line: &str) -> (String, Vec<String>) {
    let mut iter = line.split(':');
    let name = iter.next().unwrap().trim().to_owned();
    let successors = iter.next().unwrap().split_whitespace().map(str::trim).map(str::to_owned).collect();
    (name, successors)
}

fn _print_dot(graph: &Graph) {
    use std::fmt::Write;
    let mut s = String::new();
    s.push_str("graph {\n");
    for node in &graph.nodes {
        for succ in &node.connections {
            if node.name < *succ {
                writeln!(s, "__{} -- __{}", node.name, succ).unwrap();
            }
        }
    }
    s.push_str("}");
    println!("{}", s);
}

fn run_task_1(g: &mut Graph) -> Option<usize> {
    let start = g.nodes[0].name;

    // For each start and end node pair, try fo find
    // 4 disjoint shortest paths
    for potential_end in 1..g.nodes.len() {

        // Find completely disjoint shortest paths by removing the edges of the already found shortest paths.
        let sp1 = g.shortest_path(start, potential_end as u32).unwrap();
        g.remove_edges(&sp1);
        let sp2 = g.shortest_path(start, potential_end as u32).unwrap();
        g.remove_edges(&sp2);
        let sp3 = g.shortest_path(start, potential_end as u32).unwrap();
        g.remove_edges(&sp3);

        let sp4 = g.shortest_path(start, potential_end as u32);

        if sp4.is_none() {
            // Could only find three disjoint shortest paths.
            // start and end nodes must be in different components now.
            let num_reachable_from_start = g.reachable(start);
            let num_reachable_from_end = g.reachable(potential_end as u32);
            // _print_dot(&g);

            // Repair graph
            g.insert_edges(&sp3);
            g.insert_edges(&sp2);
            g.insert_edges(&sp1);

            assert_eq!(g.nodes.len(), num_reachable_from_start + num_reachable_from_end);
            return Some(num_reachable_from_start * num_reachable_from_end);
        }

        // Repair graph
        g.insert_edges(&sp3);
        g.insert_edges(&sp2);
        g.insert_edges(&sp1);
    }
    None
}

pub fn task1() -> crate::AOCResult<usize> {
    let mut g = parse();
    // print_dot(&g);
    let r = run_task_1(&mut g).unwrap();


    crate::AOCResult {
        day: 25,
        task: 1,
        r,
    }
}
