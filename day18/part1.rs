use std::fs;
use std::collections::HashSet;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Point { x: isize, y: isize, z: isize }

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
            z: self.z,
        }
    }

    fn front(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        }
    }

    fn back(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
            z: self.z,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut cubes = HashSet::new();

    for line in input.trim().split("\n") {
        let mut line = line.trim().split(",").map(|x| x.parse::<isize>().unwrap());

        cubes.insert(Point::new(line.next().unwrap(), line.next().unwrap(), line.next().unwrap()));
    }

    let mut sum = 0;
    for cube in &cubes {
        let mut exposed_sides = 6;
        if cubes.contains(&cube.up()) { exposed_sides -= 1; }
        if cubes.contains(&cube.down()) { exposed_sides -= 1; }
        if cubes.contains(&cube.left()) { exposed_sides -= 1; }
        if cubes.contains(&cube.right()) { exposed_sides -= 1; }
        if cubes.contains(&cube.front()) { exposed_sides -= 1; }
        if cubes.contains(&cube.back()) { exposed_sides -= 1; }


        sum += exposed_sides;
    }

    println!("{}", sum);
}