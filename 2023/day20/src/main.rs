use std::{fs, collections::{HashMap, VecDeque}, process::exit};

#[derive(Clone, Copy, Debug)]
enum Pulse {
    LOW,
    HIGH,
}

trait SendPulse {
    fn send_pulse(&mut self, pulse: Pulse, input_module: &str) -> Option<Pulse>;
    fn get_name(&self) -> String;
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    destinations: Vec<String>,
    state: Pulse,
}
impl SendPulse for FlipFlop {
    fn send_pulse(&mut self, pulse: Pulse, input_module: &str) -> Option<Pulse> {
        if let Pulse::HIGH = pulse {
            return None
        }

        if let Pulse::LOW = self.state {
            self.state = Pulse::HIGH;
            return Some(Pulse::HIGH)
        } else {
            self.state = Pulse::LOW;
            return Some(Pulse::LOW)
        }
    }

    fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    destinations: Vec<String>,
    seen_inputs: HashMap<String, Pulse>
}
impl SendPulse for Conjunction {
    fn send_pulse(&mut self, pulse: Pulse, input_module: &str) -> Option<Pulse> {
        self.seen_inputs.insert(input_module.to_owned(), pulse.clone());

        for pulse in dbg!(self.seen_inputs.values()) {
            if let Pulse::LOW = pulse {
                return Some(Pulse::HIGH)
            }
        }
        return Some(Pulse::LOW)
    }

    fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

#[derive(Debug)]
struct Broadcaster {
    name: String,
    destinations: Vec<String>
}
impl SendPulse for Broadcaster {
    fn send_pulse(&mut self, pulse: Pulse, input_module: &str) -> Option<Pulse> {
        Some(pulse)
    }

    fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

#[derive(Debug)]
enum Module {
    FLIPFLOP(FlipFlop),
    CONJUNCTION(Conjunction),
    BROADCASTER(Broadcaster)
}

fn send_pulse(module: String, pulse: Pulse, modules: &mut HashMap<String, Module>) -> (u64, u64) {
    let mut queue: VecDeque<(String, Pulse, String)> = VecDeque::new();
    queue.push_back((module, pulse, String::from("button")));

    let mut low_count: u64 = 0;
    let mut high_count: u64 = 0;

    while queue.len() != 0 {
        dbg!(&queue);
        let (mod_name, source_pulse, prev_mod_name) = queue.pop_front().unwrap();
        if let None = modules.get_mut(&mod_name) {
            // match source_pulse {
            //     Pulse::LOW => low_count+=1,
            //     Pulse::HIGH => high_count+=1,
            // }
            continue;
        };

        let current = modules.get_mut(&mod_name).unwrap();

        match current {
            Module::FLIPFLOP(flipflop) => {
                if let Some(dest_pulse) = flipflop.send_pulse(source_pulse, prev_mod_name.as_str()) {
                    match dest_pulse {
                        Pulse::LOW => low_count+=flipflop.destinations.len() as u64,
                        Pulse::HIGH => high_count+=flipflop.destinations.len() as u64,
                    }
                    flipflop.destinations.iter().for_each(|module_name| queue.push_back((module_name.to_owned(), dest_pulse, mod_name.to_owned())))
                }
            },
            Module::CONJUNCTION(conjuction) => {
                if let Some(dest_pulse) = conjuction.send_pulse(source_pulse, prev_mod_name.as_str()) {
                    match dest_pulse {
                        Pulse::LOW => low_count+=conjuction.destinations.len() as u64,
                        Pulse::HIGH => high_count+=conjuction.destinations.len()as u64,
                    }
                    conjuction.destinations.iter().for_each(|module_name| queue.push_back((module_name.to_owned(), dest_pulse, mod_name.to_owned())))
                }
            },
            Module::BROADCASTER(broadcaster) => {
                if let Some(dest_pulse) = broadcaster.send_pulse(source_pulse, prev_mod_name.as_str()) {
                    match dest_pulse {
                        Pulse::LOW => low_count+=broadcaster.destinations.len() as u64,
                        Pulse::HIGH => high_count+=broadcaster.destinations.len()as u64,
                    }
                    broadcaster.destinations.iter().for_each(|module_name| queue.push_back((module_name.to_owned(), dest_pulse, mod_name.to_owned())));
                }
            },
        }

    }
    
    (low_count+1, high_count)
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut modules = parse_modules(&input);

    let mut low: u64 = 0;
    let mut high: u64 = 0;

    for _ in 0..1000 {
        let result = send_pulse(String::from("broadcaster"), Pulse::LOW, &mut modules);
        dbg!(result);
        low += result.0;
        high += result.1;
    }

    println!("{}", dbg!(low)*dbg!(high))
}

fn parse_modules(input: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = HashMap::new();
    
    input.lines().for_each(|line| {
        let (module_str, destination_str) = line.split_once(" -> ").unwrap();

        let destinations: Vec<String> = destination_str.split(",").map(|str| str.trim().to_owned()).collect();

        match (module_str.chars().nth(0).unwrap(), module_str[1..].to_owned()) {
            ('%', name) => modules.insert(name.clone(), Module::FLIPFLOP(FlipFlop {name, destinations, state: Pulse::LOW})),
            ('&', name) => {

                modules.insert(name.clone(), Module::CONJUNCTION(Conjunction {name, destinations, seen_inputs: HashMap::new()}))
            },
            ('b', _name) => modules.insert(String::from("broadcaster"), Module::BROADCASTER(Broadcaster {name: String::from("broadcaster"), destinations})),
            _ => panic!()
        };


    });
    
    let conj_keys: Vec<String> = modules.keys().filter_map(|mod_name| {
        if let Module::CONJUNCTION(_) = modules.get(mod_name).unwrap() {
            return Some(mod_name.to_owned())
        }
        return None
    }).collect();

    for key in conj_keys.iter() {
        let module = modules.get(key).unwrap();
        let mut input_names: Vec<String> = Vec::new();

        if let Module::CONJUNCTION(conj) = module {
            input_names = modules.values().filter_map(|value| {
                match value {
                    Module::FLIPFLOP(f) => {
                        if f.destinations.contains(&conj.name) {
                            return Some(f.get_name())
                        }
                        return None
                    },
                    Module::CONJUNCTION(c) => {
                        if c.destinations.contains(&conj.name) {
                            return Some(c.get_name())
                        }
                        return None
                    },
                    Module::BROADCASTER(b) => {
                        if b.destinations.contains(&conj.name) {
                            return Some(b.get_name())
                        }
                        return None
                    },
                }
            }).collect();
        }

        let module = modules.get_mut(key).unwrap();
        if let Module::CONJUNCTION(con) = module {
            input_names.iter().for_each(|name| {
                con.seen_inputs.insert(name.to_owned(), Pulse::LOW);
            });
        };

    }

    modules
}
