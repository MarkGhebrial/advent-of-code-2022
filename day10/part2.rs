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

    let mut output = String::new();

    for (i, sprite_pos) in cycles.iter().enumerate() {
        let col = (i % 40) as isize;

        if (col - *sprite_pos).abs() <= 1 {
            output += "#";
        } else {
            output += " ";
        }

        if col == 39 { output += "\n"; }
    }

    println!("{}", output);
}