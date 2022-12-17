use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

//#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    children: Vec<Rc<RefCell<Valve>>>,
}

impl Valve {
    fn new(name: &str, flow_rate: usize) -> Self {
        Self {
            name: String::from(name),
            flow_rate,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, valve: &Rc<RefCell<Valve>>) {
        self.children.push(Rc::clone(valve));
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut valves = HashMap::new();
    for line in input.trim().split("\n") {
        let line = line.split(" ").collect::<Vec<&str>>();

        let name = line[1];
        let flow_rate = line[4][5..line[4].len() - 1].parse::<usize>().unwrap();

        println!("{name}: {flow_rate}");

        valves.insert(String::from(name), Rc::new(RefCell::new(Valve::new(name, flow_rate))));
    }

    for line in input.trim().split("\n") {
        let line = line.split(" ").collect::<Vec<&str>>();
        let name = line[1];
        let downstream_valves = line[9..].iter().map(|s| &s[..2]);

        let valve = Rc::clone(valves[name]);
        let mut valve = valve.borrow_mut();

        for valve_name in downstream_valves {
            println!("Adding {} to {}", valve_name, valve.name);
            valve.add_child(&valves[valve_name]);
        }
    }

    let valve_aa = Rc::clone(valves["AA"]);

    let mut minutes = 30;
    let mut pressure_release = 0;
    let mut released_pressure = 0;
    while minutes > 0 {
        
    }
}