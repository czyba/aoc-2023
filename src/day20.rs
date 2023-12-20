use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PulseType {
    Low,
    High,
}

impl PulseType {
    fn flip(self) -> Self {
        match self {
            PulseType::Low => PulseType::High,
            PulseType::High => PulseType::Low,
        }
    }
}

struct Pulse {
    source: String,
    pulse_type: PulseType,
    target: String,
}

fn _append_dot_node(module: &dyn Module, s: &mut String) {
    if let Some(targets) = module.get_targets() {
        for target in targets {
            s.push_str(&format!("{} -> {};\n", module.get_name(), target));
        }
    }
}

trait Module {
    fn evaluate_pulse(&mut self, pulse: &Pulse) -> Option<Vec<Pulse>>;
    fn get_name(&self) -> &str;
    fn get_targets(&self) -> Option<&Vec<String>>;
    fn reset(&mut self) {}
    fn broke(&mut self) -> Option<&mut BTreeMap<String, PulseType>> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FlipFlopModule {
    name: String,
    last_pulse: PulseType,
    targets: Vec<String>,
}

impl FlipFlopModule {
    fn new(name: String, targets: Vec<String>) -> Self {
        FlipFlopModule {
            name,
            last_pulse: PulseType::Low,
            targets,
        }
    }
}

fn targets_to_pulses(source: &str, targets: &[String], pulse: PulseType) -> Vec<Pulse> {
    targets
        .iter()
        .map(|target| Pulse {
            source: source.to_owned(),
            target: target.clone(),
            pulse_type: pulse,
        })
        .collect()
}

impl Module for FlipFlopModule {
    fn evaluate_pulse(&mut self, pulse: &Pulse) -> Option<Vec<Pulse>> {
        match pulse.pulse_type {
            PulseType::Low => {
                self.last_pulse = self.last_pulse.flip();
                Some(targets_to_pulses(
                    &self.name,
                    &self.targets,
                    self.last_pulse,
                ))
            }
            PulseType::High => None,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_targets(&self) -> Option<&Vec<String>> {
        Some(&self.targets)
    }

    fn reset(&mut self) {
        self.last_pulse = PulseType::Low;
    }
}

impl ModulePlus for FlipFlopModule {
    fn as_module(&self) -> &dyn Module {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BroadcastModule {
    name: String,
    targets: Vec<String>,
}

impl Module for BroadcastModule {
    fn evaluate_pulse(&mut self, pulse: &Pulse) -> Option<Vec<Pulse>> {
        Some(targets_to_pulses(
            &self.name,
            &self.targets,
            pulse.pulse_type,
        ))
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_targets(&self) -> Option<&Vec<String>> {
        Some(&self.targets)
    }
}

impl ModulePlus for BroadcastModule {
    fn as_module(&self) -> &dyn Module {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ConjunctionModule {
    name: String,
    last_inputs: BTreeMap<String, PulseType>,
    targets: Vec<String>,
}

impl ConjunctionModule {
    fn new(name: String, targets: Vec<String>) -> Self {
        ConjunctionModule {
            name,
            last_inputs: BTreeMap::new(),
            targets,
        }
    }
}

impl Module for ConjunctionModule {
    fn evaluate_pulse(&mut self, pulse: &Pulse) -> Option<Vec<Pulse>> {
        if let Some(a) = self.last_inputs.get_mut(&pulse.source) {
            *a = pulse.pulse_type;
        }
        let pulse = if self.last_inputs.values().all(|&v| v == PulseType::High) {
            PulseType::Low
        } else {
            PulseType::High
        };
        Some(targets_to_pulses(&self.name, &self.targets, pulse))
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_targets(&self) -> Option<&Vec<String>> {
        Some(&self.targets)
    }

    fn reset(&mut self) {
        for (_, v) in self.last_inputs.iter_mut() {
            *v = PulseType::Low;
        }
    }

    fn broke(&mut self) -> Option<&mut BTreeMap<String, PulseType>> {
        Some(&mut self.last_inputs)
    }
}

impl ModulePlus for ConjunctionModule {
    fn as_module(&self) -> &dyn Module {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SinkModule {}

impl Module for SinkModule {
    fn evaluate_pulse(&mut self, _: &Pulse) -> Option<Vec<Pulse>> {
        None
    }

    fn get_name(&self) -> &str {
        ""
    }

    fn get_targets(&self) -> Option<&Vec<String>> {
        None
    }
}

impl ModulePlus for SinkModule {
    fn as_module(&self) -> &dyn Module {
        self
    }
}

enum ModuleType {
    Sink,
    Broadcast,
    FlipFlop,
    Conjunction,
}

struct ModuleInfo {
    name: String,
    module_type: ModuleType,
    targets: Vec<String>,
}

fn parse_line(line: &str) -> ModuleInfo {
    let mut iter = line.split("->");
    let module_id = iter.next().unwrap().trim();
    let (name, module_type) = if module_id == "broadcaster" {
        (module_id.to_owned(), ModuleType::Broadcast)
    } else if module_id.starts_with('%') {
        (
            module_id[1..module_id.len()].to_owned(),
            ModuleType::FlipFlop,
        )
    } else if module_id.starts_with('&') {
        (
            module_id[1..module_id.len()].to_owned(),
            ModuleType::Conjunction,
        )
    } else {
        (module_id.to_owned(), ModuleType::Sink)
    };
    let targets = iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim())
        .map(|s| s.to_owned())
        .collect();
    ModuleInfo {
        name,
        module_type,
        targets,
    }
}

trait ModulePlus: Module + Debug {
    fn as_module(&self) -> &dyn Module;
}

type Network = HashMap<String, Box<dyn ModulePlus>>;
type PredecessorMap = HashMap<String, Vec<String>>;

fn parse() -> (Network, PredecessorMap) {
    let module_infos: Vec<ModuleInfo> = lines_from_file("src/day20.txt")
        .unwrap()
        .map(|line| parse_line(&line))
        .collect();

    let mut modules: Network = HashMap::new();

    let mut inputs = HashMap::new();

    module_infos.iter().for_each(|mi| {
        mi.targets.iter().for_each(|t| {
            inputs
                .entry(t.clone())
                .or_insert_with(Vec::new)
                .push(mi.name.clone());
        })
    });

    for mi in module_infos {
        match mi.module_type {
            ModuleType::Sink => {
                modules.insert(mi.name, Box::new(SinkModule {}));
            }
            ModuleType::Broadcast => {
                modules.insert(
                    mi.name.clone(),
                    Box::new(BroadcastModule {
                        name: mi.name,
                        targets: mi.targets,
                    }),
                );
            }
            ModuleType::FlipFlop => {
                modules.insert(
                    mi.name.clone(),
                    Box::new(FlipFlopModule::new(mi.name, mi.targets)),
                );
            }
            ModuleType::Conjunction => {
                let inputs = inputs.get(&mi.name).cloned().unwrap_or_else(Vec::new);
                let mut conjuction_module =
                    Box::new(ConjunctionModule::new(mi.name.clone(), mi.targets));
                for input in inputs {
                    conjuction_module.last_inputs.insert(input, PulseType::Low);
                }
                modules.insert(mi.name, conjuction_module);
            }
        }
    }

    for k in inputs.keys() {
        modules.entry(k.clone()).or_insert(Box::new(SinkModule {}));
    }

    (modules, inputs)
}

fn push_button(network: &mut Network) -> (u64, u64) {
    let mut pulses = VecDeque::new();
    pulses.push_back(Pulse {
        source: "button".to_owned(),
        pulse_type: PulseType::Low,
        target: "broadcaster".to_owned(),
    });

    let mut num_pulses = (0, 0);

    while let Some(pulse) = pulses.pop_front() {
        let module = network.get_mut(&pulse.target).unwrap();
        match pulse.pulse_type {
            PulseType::Low => num_pulses.0 += 1,
            PulseType::High => num_pulses.1 += 1,
        }

        if let Some(targets) = module.evaluate_pulse(&pulse) {
            pulses.extend(targets);
        }
    }
    num_pulses
}

pub fn task1() -> crate::AOCResult<u64> {
    let (mut network, _) = parse();

    let mut sum = (0, 0);
    for _ in 0..1000 {
        let pulses = push_button(&mut network);
        sum.0 += pulses.0;
        sum.1 += pulses.1;
    }

    crate::AOCResult {
        day: 20,
        task: 1,
        r: sum.0 * sum.1,
    }
}

fn push_button_was_node_low(network: &mut Network, last_node: &str) -> bool {
    let mut pulses = VecDeque::new();
    pulses.push_back(Pulse {
        source: "button".to_owned(),
        pulse_type: PulseType::Low,
        target: "broadcaster".to_owned(),
    });

    let mut num_pulses = (0, 0);

    let mut res = false;
    while let Some(pulse) = pulses.pop_front() {
        if pulse.target == last_node && pulse.pulse_type == PulseType::Low {
            res = true;
        }
        let module = network.get_mut(&pulse.target).unwrap();
        match pulse.pulse_type {
            PulseType::Low => num_pulses.0 += 1,
            PulseType::High => num_pulses.1 += 1,
        }

        if let Some(targets) = module.evaluate_pulse(&pulse) {
            pulses.extend(targets);
        }
    }
    res
}

fn _print_dot_graph(network: &Network) {
    let mut s = String::new();
    s.push_str("digraph {\n");
    s.push_str("button -> broadcaster;\n");
    for e in network {
        let module: &dyn ModulePlus = e.1.as_ref();
        _append_dot_node(module.as_module(), &mut s);
    }

    s.push('}');

    println!("{}", s);
}

fn iterator_broadcast_single(network: &mut Network, last_id: String, pre_last_id: String) -> u64 {
    let mut num_cnt: u64 = 1;
    let reset_values = network
        .get_mut(&pre_last_id)
        .unwrap()
        .broke()
        .unwrap()
        .clone();

    for keys in reset_values.keys() {
        network
            .get_mut(&pre_last_id)
            .unwrap()
            .broke()
            .unwrap()
            .clear();
        network
            .get_mut(&pre_last_id)
            .unwrap()
            .broke()
            .unwrap()
            .insert(keys.clone(), PulseType::Low);

        let mut cnt = 0;
        loop {
            cnt += 1;
            if push_button_was_node_low(network, &last_id) {
                break;
            }
        }

        network.iter_mut().for_each(|(_, v)| v.reset());

        num_cnt *= cnt;
    }

    *network.get_mut(&pre_last_id).unwrap().broke().unwrap() = reset_values;

    num_cnt
}

fn find_last_and_pre_last_node_ids(
    network: &Network,
    predecessors: &PredecessorMap,
) -> (String, String) {
    let last_id = predecessors
        .keys()
        .find(|id| network.get(*id).unwrap().get_targets().is_none())
        .unwrap();
    let pre_last_id = &predecessors.get(last_id).unwrap()[0];
    (last_id.clone(), pre_last_id.clone())
}

pub fn task2() -> crate::AOCResult<u64> {
    let (mut network, predecessors) = parse();

    // _print_dot_graph(&_network);

    let nodes_to_adjust = find_last_and_pre_last_node_ids(&network, &predecessors);
    let result = iterator_broadcast_single(&mut network, nodes_to_adjust.0, nodes_to_adjust.1);
    println!("{}", result);

    crate::AOCResult {
        day: 20,
        task: 2,
        r: result,
    }
}
