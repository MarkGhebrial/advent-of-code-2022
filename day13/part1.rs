use std::cell::RefCell;
use std::fs;

#[derive(Debug)]
enum Packet {
    Value(usize),
    List(Vec<RefCell<Packet>>),
}

impl Packet {
    fn parse_string(s: &str) -> Packet {
        let mut num = String::new();
        let mut out = Packet::List(vec![]);
        let mut nest_level = 0;

        println!("{}", s);

        for (i, c) in s/*[1..s.len() - 2]*/.chars().enumerate() {
            if c == '[' {
                nest_level += 1;
                if let Packet::List(ref mut vec) = out {
                    let mut start = i + 1;
                    for (i, c) in s[i + 1..].chars().enumerate() {
                        if c == '[' {
                            start = i + 1;
                            break;
                        }
                    }
                    vec.push(RefCell::new(Packet::parse_string(&s[start..])));
                }
            }
            if c == ']' {
                nest_level -= 1;
                if nest_level < 0 {
                    nest_level = 0;
                    //continue;
                }
                continue;
            }

            println!("Char: {} Level: {}", c, nest_level);

            if nest_level != 0 /*|| nest_level < 0*/ {
                println!("Continued");
                continue;
            }

            if c == ',' {
                if let Packet::List(ref mut vec) = out {
                    println!("{}", num);
                    vec.push(RefCell::new(Packet::Value(num.parse::<usize>().unwrap())));
                    num = String::from("");
                    println!("Parsed number");
                }
            } else {
                num += &String::from(c);
            }
        }

        println!("Returned {:?}", out);

        out
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    for pair in input.split("\n\n") {
        let packets = pair.split("\n").map(|s| s.trim()).collect::<Vec<&str>>();
        let first = Packet::parse_string(packets[0]);
        let second = Packet::parse_string(packets[1]);

        println!("{:?}", first);
    }
}
