use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut cycles = vec![];
    let mut register = 1;

    for line in input.trim().split("\n") {
        let args = line.trim().split(" ").collect::<Vec<&str>>();
        let instruction = args[0];
        
        match instruction {
            "noop" => cycles.push(register),
            "addx" => {
                cycles.push(register);
                cycles.push(register);

                register += args[1].parse::<isize>().unwrap();
            },
            _ => panic!("Unknown command"),
        }
    }

    let mut sum = 0;
    for i in 0..6 {
        println!("{}", 20 + i * 40);
        sum += cycles[(20 + i * 40)-1] * (20 + i * 40) as isize;
    }

    println!("{:?}\n{}", cycles, sum);
}