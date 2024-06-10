use std::{fs, collections::HashMap};

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn parse(input: char) -> Category {
        match input {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S, 
            _ => panic!("bad category")
        }
    }
}

#[derive(Debug)]
enum Operator {
    Less,
    Greater,
}
impl Operator {
    fn parse(input: char) -> Operator {
        match input {
            '>' => Self::Greater,
            '<' => Self::Less,
            _ => panic!("bad operator"),
        } 
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Comparison>,
    default_send: String,
}

impl Workflow {
    fn build(input: &str) -> Workflow {
        let (name, rules_str) = input.split_once("{").unwrap();
        let mut temp_vec: Vec<&str> = rules_str[..rules_str.len() - 1].split(",").collect();
        let default_send = temp_vec.pop().unwrap().to_owned();
        let rules: Vec<Comparison> = temp_vec.iter().map(|rule_str| Comparison::build(rule_str)).collect();
            
        Workflow {name: name.to_owned(), rules, default_send}
    }

    fn get_next_workflow(&self, part: &Part) -> String {
        let next = self.rules.iter().find_map(|comp| {
            comp.compare(part)
        });
        
        match next {
            Some(workflow) => workflow,
            None => self.default_send.to_owned(),
        }
    }

    fn workflow_get_ranges(&self) -> Vec<(PartRange, &str)> {
        todo!() 
    }
}

#[derive(Debug)]
struct Comparison {
    category: Category,
    operator: Operator,
    value: u32,
    send_to: String,
}
impl Comparison {
    fn build(rule_str: &str) -> Comparison {
        let (cmp_str, send_to) = rule_str.split_once(":").unwrap();
        let category = Category::parse(cmp_str.chars().nth(0).unwrap());
        let operator = Operator::parse(cmp_str.chars().nth(1).unwrap());
        let value: u32 = cmp_str[2..].parse().unwrap();

        Comparison {send_to: send_to.to_owned(), category, operator, value}
    }

    fn compare(&self, part: &Part) -> Option<String> {
        match (&self.category, &self.operator) {
            (Category::X, Operator::Less) => (part.x < self.value).then(|| self.send_to.to_owned()) ,
            (Category::X, Operator::Greater) => (part.x > self.value).then(|| self.send_to.to_owned()),
            (Category::M, Operator::Less) => (part.m < self.value).then(|| self.send_to.to_owned()),
            (Category::M, Operator::Greater) => (part.m > self.value).then(|| self.send_to.to_owned()),
            (Category::A, Operator::Less) => (part.a < self.value).then(|| self.send_to.to_owned()),
            (Category::A, Operator::Greater) => (part.a > self.value).then(|| self.send_to.to_owned()),
            (Category::S, Operator::Less) => (part.s < self.value).then(|| self.send_to.to_owned()),
            (Category::S, Operator::Greater) => (part.s > self.value).then(|| self.send_to.to_owned()),
        } 
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
impl Part {
    fn build(input: &str) -> Part {
        let temp: Vec<&str> = input[1..input.len() -1].split(",").collect();
        let x: u32 = temp[0][2..].parse().unwrap();
        let m: u32 = temp[1][2..].parse().unwrap();
        let a: u32 = temp[2][2..].parse().unwrap();
        let s: u32 = temp[3][2..].parse().unwrap();

        Part {x, m, a, s}
    }

    fn get_value(&self) -> u64 {
        u64::from(self.x + self.m +self.a + self.s)
    }
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn full() -> Range {
        Range {start: 0, end: 4001}
    }
}

struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn get_initial() -> PartRange {
        PartRange { x: Range::full(), m: Range::full(), a: Range::full(), s: Range::full() }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows_str);
    let parts = parse_parts(parts_str);

    dbg!(&workflows);
    dbg!(&parts);

    println!("{}", part1(&workflows, &parts));
    
}

fn part1(workflows: &HashMap<String, Workflow>, parts: &Vec<Part>) -> u64 {
    let mut accepted_parts: Vec<Part> = Vec::new();

    let starting_workflow = workflows.get("in").unwrap().name.as_str();
    
    for part in parts.iter() {
        workflow_send(starting_workflow, &workflows, part, &mut accepted_parts)
    };

    accepted_parts.iter().map(|part| part.get_value()).sum()
}


fn workflow_send(workflow: &str, workflows: &HashMap<String, Workflow>, part: &Part, accepted_parts: &mut Vec<Part>) {
    if workflow == "A" {
        accepted_parts.push(part.to_owned());
        return;
    } 
    if workflow == "R" {
        return;
    }

    let next_workflow = workflows.get(workflow).unwrap().get_next_workflow(part);

    workflow_send(next_workflow.as_str(), workflows, part, accepted_parts);

}


fn part2(workflows: &HashMap<String, Workflow>) {
    let starting_workflow = "in";

    let mut computed: Vec<(PartRange, bool)> = Vec::new();
    let part_range = PartRange::get_initial();

    

}

fn workflow_compute_ranges(workflow: &str, workflows: &HashMap<String, Workflow>, part_range: PartRange, computed: &mut Vec<(PartRange, bool)>) {
    if workflow == "A" {
        computed.push((part_range, true));
        return;
    }
    if workflow == "R" {
        computed.push((part_range, false));
        return;
    }

    let workflow_ranges = workflows.get(workflow).unwrap().workflow_get_ranges();

    for range in 


}

fn parse_workflows(input: &str) -> HashMap<String, Workflow> {
    let mut workflows = HashMap::new();

    input.lines().for_each(|line| {
        let workflow = Workflow::build(line);
        workflows.insert(workflow.name.to_owned(), workflow);
    });

    workflows
}

fn parse_parts(input: &str) -> Vec<Part> {
    input.lines().map(|line| {
        Part::build(line)
    }).collect()
}
