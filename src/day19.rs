use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::Read;
use std::io::Result;

fn read_file_to_string(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(windows)]
const DOUBLE_LINE_ENDING: &str = "\r\n\r\n";
#[cfg(not(windows))]
const DOUBLE_LINE_ENDING: &str = "\n\n";

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn get_total_value(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl PartRange {
    fn get_total_value(&self) -> i64 {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Attribute {
    X,
    M,
    A,
    S,
}

impl Attribute {
    fn get_value(self, part: &Part) -> u64 {
        match self {
            Attribute::X => part.x,
            Attribute::M => part.m,
            Attribute::A => part.a,
            Attribute::S => part.s,
        }
    }

    fn get_range(self, part: &mut PartRange) -> &mut (i64, i64) {
        match self {
            Attribute::X => &mut part.x,
            Attribute::M => &mut part.m,
            Attribute::A => &mut part.a,
            Attribute::S => &mut part.s,
        }
    }
}

impl From<&str> for Attribute {
    fn from(value: &str) -> Self {
        match value {
            "x" => Attribute::X,
            "m" => Attribute::M,
            "a" => Attribute::A,
            "s" => Attribute::S,
            x => panic!("{}", x),
        }
    }
}

#[derive(Debug)]
struct Condition {
    var: Attribute,
    cmp: std::cmp::Ordering,
    value: u64,
    attribute_left: bool,
}

impl Condition {
    fn matches_part(&self, part: &Part) -> bool {
        let value = self.var.get_value(part);
        let cmp_res = if self.attribute_left {
            value.cmp(&self.value)
        } else {
            self.value.cmp(&value)
        };
        cmp_res == self.cmp
    }

    fn split_range(&self, part_range: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        let (lt_value, true_lower) = match (self.attribute_left, self.cmp) {
            (true, std::cmp::Ordering::Less) => (self.value, true),
            (false, std::cmp::Ordering::Less) => (self.value + 1, false),
            (true, std::cmp::Ordering::Greater) => (self.value + 1, false),
            (false, std::cmp::Ordering::Greater) => (self.value, true),
            x => panic!("{:?}", x),
        };
        let mut true_range = part_range.clone();
        let mut false_range = part_range.clone();
        let true_arr_range = self.var.get_range(&mut true_range);
        true_arr_range.1 = lt_value as i64 - 1;
        let false_arr_range = self.var.get_range(&mut false_range);
        false_arr_range.0 = lt_value as i64;
        let res = match (
            true_arr_range.1 >= true_arr_range.0,
            false_arr_range.1 >= false_arr_range.0,
        ) {
            (true, true) => (Some(true_range), Some(false_range)),
            (true, false) => (Some(true_range), None),
            (false, true) => (None, Some(false_range)),
            (false, false) => (None, None),
        };
        if true_lower {
            res
        } else {
            (res.1, res.0)
        }
    }
}

#[derive(Debug)]
struct ConditionalRule {
    condition: Condition,
    target: String,
}

impl ConditionalRule {
    fn try_match(&self, part: &Part) -> Option<String> {
        if self.condition.matches_part(part) {
            Some(self.target.clone())
        } else {
            None
        }
    }

    fn split_range(
        &self,
        part_range: &PartRange,
    ) -> ((Option<PartRange>, String), Option<PartRange>) {
        let r = self.condition.split_range(part_range);
        ((r.0, self.target.clone()), r.1)
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<ConditionalRule>,
    fallback_target: String,
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Accepted,
    Rejected,
    Redirected(String),
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        match value.as_str() {
            "A" => Status::Accepted,
            "R" => Status::Rejected,
            _ => Status::Redirected(value),
        }
    }
}

impl Workflow {
    fn evaluate(&self, part: &Part) -> Status {
        for rule in &self.rules {
            if let Some(target) = rule.try_match(part) {
                return Status::from(target);
            }
        }
        Status::from(self.fallback_target.clone())
    }

    fn evaluate_ranges(&self, part_range: &PartRange) -> Vec<(PartRange, Status)> {
        let mut next_range = part_range.clone();
        let mut ret = Vec::new();
        for rule in &self.rules {
            let ((trange, id), frange) = rule.split_range(&next_range);
            if let Some(range) = trange {
                ret.push((range, Status::from(id)));
            }
            if let Some(range) = frange {
                next_range = range;
                continue;
            }
            return ret;
        }
        ret.push((next_range, Status::from(self.fallback_target.clone())));
        ret
    }
}

fn parse() -> (HashMap<String, Workflow>, Vec<Part>) {
    let s = read_file_to_string("src/day19.txt").unwrap();
    let mut splits = s.split(DOUBLE_LINE_ENDING);
    let workflows = parse_workflows(splits.next().unwrap());
    let parts = parse_parts(splits.next().unwrap());
    (workflows, parts)
}

fn parse_workflows(workflows: &str) -> HashMap<String, Workflow> {
    workflows.lines().map(parse_workflow).collect()
}

fn parse_workflow(workflow: &str) -> (String, Workflow) {
    let mut iter = workflow.split('{');
    let name = iter.next().unwrap().to_owned();
    let wf = iter.next().unwrap();
    let wf = &wf[0..wf.len() - 1];
    let iter = wf.split(',');
    let mut rules = Vec::new();
    let mut fallback_target = "";
    for rule_str in iter {
        if let Some(condition_end_index) = rule_str.find(':') {
            let target = &rule_str[condition_end_index + 1..rule_str.len()];
            if let Some(operator_index) = rule_str.find('<') {
                if let Ok(value) = rule_str[0..operator_index].parse::<u64>() {
                    rules.push(ConditionalRule {
                        condition: Condition {
                            var: Attribute::from(
                                &rule_str[operator_index + 1..condition_end_index],
                            ),
                            cmp: std::cmp::Ordering::Less,
                            value,
                            attribute_left: false,
                        },
                        target: target.to_owned(),
                    })
                } else {
                    rules.push(ConditionalRule {
                        condition: Condition {
                            var: Attribute::from(&rule_str[0..operator_index]),
                            cmp: std::cmp::Ordering::Less,
                            value: rule_str[operator_index + 1..condition_end_index]
                                .parse::<u64>()
                                .unwrap(),
                            attribute_left: true,
                        },
                        target: target.to_owned(),
                    })
                }
            } else if let Some(operator_index) = rule_str.find('>') {
                if let Ok(value) = rule_str[0..operator_index].parse::<u64>() {
                    rules.push(ConditionalRule {
                        condition: Condition {
                            var: Attribute::from(
                                &rule_str[operator_index + 1..condition_end_index],
                            ),
                            cmp: std::cmp::Ordering::Greater,
                            value,
                            attribute_left: false,
                        },
                        target: target.to_owned(),
                    })
                } else {
                    rules.push(ConditionalRule {
                        condition: Condition {
                            var: Attribute::from(&rule_str[0..operator_index]),
                            cmp: std::cmp::Ordering::Greater,
                            value: rule_str[operator_index + 1..condition_end_index]
                                .parse::<u64>()
                                .unwrap(),
                            attribute_left: true,
                        },
                        target: target.to_owned(),
                    })
                }
            } else {
                panic!("{}", rule_str);
            }
        } else {
            fallback_target = rule_str;
        }
    }
    (
        name,
        Workflow {
            rules,
            fallback_target: fallback_target.to_owned(),
        },
    )
}

fn parse_parts(parts: &str) -> Vec<Part> {
    parts.lines().map(parse_part).collect()
}

fn parse_part(line: &str) -> Part {
    let mut iter = line.split(',');
    let part = iter.next().unwrap();
    let attr_split = part.split('=');
    let x = attr_split.last().unwrap().parse::<u64>().unwrap();
    let part = iter.next().unwrap();
    let attr_split = part.split('=');
    let m = attr_split.last().unwrap().parse::<u64>().unwrap();
    let part = iter.next().unwrap();
    let attr_split = part.split('=');
    let a = attr_split.last().unwrap().parse::<u64>().unwrap();
    let part = iter.next().unwrap();
    let attr_split = part.split('=');
    let val_str = attr_split.last().unwrap();
    let s = val_str[0..val_str.len() - 1].parse::<u64>().unwrap();
    Part { x, m, a, s }
}

pub fn task1() {
    let (workflows, parts) = parse();
    let sum: u64 = parts
        .into_iter()
        .filter(|part| {
            let mut status = Status::Redirected("in".to_owned());
            while let Status::Redirected(wf_name) = status {
                let wf = workflows.get(&wf_name).unwrap();
                status = wf.evaluate(part);
            }
            status == Status::Accepted
        })
        .map(|part| part.get_total_value())
        .sum();

    println!("Day 19, Task 1: {}", sum);
}

pub fn task2() {
    let (workflows, _) = parse();
    let mut worklist = Vec::new();
    worklist.push((
        PartRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        "in".to_owned(),
    ));
    let mut count = 0;
    while let Some((range, wfid)) = worklist.pop() {
        let wf = workflows.get(&wfid).unwrap();
        let r = wf.evaluate_ranges(&range);
        for (new_range, status) in r {
            match status {
                Status::Accepted => count += new_range.get_total_value(),
                Status::Rejected => (),
                Status::Redirected(x) => worklist.push((new_range, x)),
            }
        }
    }

    println!("Day 19, Task 2: {}", count);
}
