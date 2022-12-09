use std::fs;
use std::collections::HashSet;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Point(isize, isize);

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);
    let mut places_visited: HashSet<Point> = HashSet::new();

    for line in input.trim().split("\n") {
        let line = line.trim().split(" ").collect::<Vec<&str>>();
        let dir = line[0];
        let dist = line[1].parse::<isize>().unwrap();

        println!("{} {}", dir, dist);

        for _ in 0..dist {
            match dir {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                _ => panic!("Unknown direction"),
            };

            // Update the tail's position
            let adjacent = (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1;
            if !adjacent {
                let mut new_pos = head.clone();
                match dir {
                    "R" => new_pos.0 -= 1,
                    "L" => new_pos.0 += 1,
                    "U" => new_pos.1 -= 1,
                    "D" => new_pos.1 += 1,
                    _ => panic!("Unknown direction"),
                };
                tail = new_pos;
            }

            places_visited.insert(tail.clone());
        }
    }

    println!("{}", places_visited.len());
}