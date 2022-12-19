use std::fs;
use std::collections::HashSet;
use std::collections::VecDeque;

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

    fn edges(&self) -> Vec<Self> {
        vec![self.up(), self.down(), self.left(), self.right(), self.front(), self.back()]
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut cubes = HashSet::new();

    for line in input.trim().split("\n") {
        let mut line = line.trim().split(",").map(|x| x.parse::<isize>().unwrap());

        cubes.insert(Point::new(line.next().unwrap(), line.next().unwrap(), line.next().unwrap()));
    }

    let mut air_cubes = HashSet::new();

    let mut sum = 0;
    for cube in &cubes {
        let mut exposed_sides = 6;
        for edge in cube.edges().into_iter() {
            if cubes.contains(&edge) { exposed_sides -= 1; }
            else { air_cubes.insert(edge); }
        }

        sum += exposed_sides;
    }

    let mut internal_cubes = HashSet::new();

    for x in 0..49 {
        for y in 0..49 {
            for z in 0..49 {
                let cube = Point::new(x, y, z);
                if cubes.contains(&cube) {
                    continue;
                }

                //BFS
                let mut queue = VecDeque::new();
                queue.push_back(Point::new(0, 0, 0));
                let mut visited = HashSet::new();
                visited.insert(Point::new(0, 0, 0));

                let mut is_internal = true;
                let mut iterations = 0;
                'bfs: while !queue.is_empty() && iterations < 50000 {
                    let current_node = queue.pop_front().unwrap();

                    if current_node == cube {
                        // We managed to find the cube, so it is not internal
                        is_internal = false;
                        break 'bfs;
                    }

                    for edge in current_node.edges().into_iter() {
                        if edge.x < 0 || edge.y < 0 || edge.z < 0 {
                            continue;
                        }
                        if !cubes.contains(&edge) && !visited.contains(&edge) {
                            visited.insert(Point::new(edge.x, edge.y, edge.z));
                            queue.push_back(edge);
                        }
                    }

                    iterations += 1;
                }
                // if iterations == 10000 {
                //     is_internal = true;
                // }

                if is_internal {
                    println!("{:?} is internal", cube);
                    internal_cubes.insert(cube);
                } else {
                    println!("{:?} is not internal", cube);
                }
            }
        }
    }

    for cube in air_cubes {
        // //BFS
        // let mut queue = VecDeque::new();
        // queue.push_back(Point::new(0, 0, 0));
        // let mut visited = HashSet::new();
        // visited.insert(Point::new(0, 0, 0));

        // let mut is_internal = true;
        // let mut iterations = 0;
        // while !queue.is_empty() && iterations < 1000 {
        //     let current_node = queue.pop_front().unwrap();

        //     if cube == current_node {
        //         is_internal = false;
        //     }

        //     for edge in cube.edges().into_iter() {
        //         if !cubes.contains(&edge) {
        //             visited.insert(Point::new(edge.x, edge.y, edge.z));
        //             queue.push_back(edge);
        //         }
        //     }

        //     iterations += 1;
        // }

        // if is_internal {
        //     internal_cubes.insert(cube);
        // }

        // println!("{}", iterations == 1000);
    }

    for cube in &internal_cubes {
        let mut exposed_sides = 6;
        for edge in cube.edges().into_iter() {
            if internal_cubes.contains(&edge) {
                exposed_sides -= 1;
            }
        }

        sum -= exposed_sides;
    }

    println!("{}", sum);
}