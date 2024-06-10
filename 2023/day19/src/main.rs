use std::{collections::{HashMap, VecDeque}, fs};

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn full() -> Range {
        Range {start: 1, end: 4001}
    }

    fn contains(&self, value: u32) -> bool {
        value >= self.start && value < self.end
    }

    fn contains_range(&self, range: Range) {

    }
}

struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn default() -> PartRange {
        PartRange { x: Range::full(), m: Range::full(), a: Range::full(), s: Range::full() }
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

    fn compare(&self, part_range: &PartRange) -> Option<String> {
        match (&self.category, &self.operator) {
            (Category::X, Operator::Less) => (),
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

    fn get_workflow_partranges(&self, part_range: PartRange) {
    }
}


impl PartRange {
    fn get_initial() -> PartRange {
        PartRange { x: Range::full(), m: Range::full(), a: Range::full(), s: Range::full() }
    }
}

fn parse_workflows(input: &str) -> HashMap<String, Workflow> {
    let mut workflows = HashMap::new();

    input.lines().for_each(|line| {
        let workflow = Workflow::build(line);
        workflows.insert(workflow.name.to_owned(), workflow);
    });

    workflows
}

fn part2(workflows: &HashMap<String, Workflow>) {
    let mut queue = VecDeque::new();
    queue.push_back(("in", PartRange::default()));

    let mut computed: Vec<(PartRange, bool)> = Vec::new();

    while !queue.is_empty() {
        let (wf_name, current_range) = queue.pop_front().unwrap();
        let current_workflow = workflows.get(wf_name).unwrap();
    }
    

}

fn main() {
    let input = fs::read_to_string("sample").unwrap();
    let workflows = parse_workflows(&input);

}
