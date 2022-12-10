use std::fs;
use std::collections::HashSet;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Point(isize, isize);

fn move_knot(head: &Point, tail: &Point) -> Point {
    let mut new_tail = tail.clone();

    let adjacent = (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1;

    if !adjacent && head.0 != tail.0 && head.1 != tail.1 { // The points are diagonally separate
        new_tail.0 += if head.0 > tail.0 { 1 } else { -1 };
        new_tail.1 += if head.1 > tail.1 { 1 } else { -1 };
        return new_tail;
    }

    if head.0 - tail.0 > 1 {
        new_tail.0 += 1;
    } else if tail.0 - head.0 > 1 {
        new_tail.0 -= 1;
    }

    if head.1 - tail.1 > 1 {
        new_tail.1 += 1;
    } else if tail.1 - head.1 > 1 {
        new_tail.1 -= 1;
    }

    return new_tail;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut rope = vec![];
    for _ in 1..=10 {
        rope.push(Point(0, 0));
    }

    let mut places_visited: HashSet<Point> = HashSet::new();

    for line in input.trim().split("\n") {
        let line = line.trim().split(" ").collect::<Vec<&str>>();
        let dir = line[0];
        let dist = line[1].parse::<isize>().unwrap();

        for _ in 0..dist {
            match dir {
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                _ => panic!("Unknown direction"),
            };

            // Update the knots' positions
            for i in 0..rope.len() - 1 {
                rope[i + 1] = move_knot(&rope[i], &rope[i + 1]);
            }

            places_visited.insert(rope[rope.len() - 1].clone());
        }
    }

    println!("{}", places_visited.len());
}