use std::fs;

#[derive(PartialEq)]
enum CellState {
    Searched,
    Unsearched,
}

struct Grid(Vec<Vec<CellState>>);

impl Grid {
    fn new() -> Self {
        let mut v = Vec::with_capacity(50000);
        for _ in 0..50000 {
            let mut v1 = Vec::with_capacity(50000);
            for _ in 0..50000 {
                v1.push(CellState::Unsearched);
            }
            v.push(v1);
        }

        Grid(v)
    }

    fn get_mut(&mut self, x: isize, y: isize) -> &mut CellState {
        &mut self.0[(y+10000) as usize][(x+10000) as usize]
    }

    fn get(&self, x: isize, y: isize) -> &CellState {
        &self.0[(y+10000) as usize][(x+10000) as usize]
    }

    fn len_y(&self) -> usize {
        self.0.len()
    }

    fn len_x(&self) -> usize {
        self.0[0].len()
    }

    /*fn print(&self) {
        for v in self.0.iter() {
            for c in v {
                print!("{}", c);
            }
            println!("");
        }
    }*/
}

fn main() {
    let input = fs::read_to_string("test.txt").unwrap();

    println!("Allocating memory...");
    let mut grid = Grid::new();
    println!("Allocated");

    for line in input.trim().split("\n") {
        println!("line: {}", line);
        
        let line = line.trim().split(" ").collect::<Vec<&str>>();
        let sensor_x = line[2][2..line[2].len() - 1].parse::<isize>().unwrap();
        let sensor_y = line[3][2..line[3].len() - 1].parse::<isize>().unwrap();

        let beacon_x = line[8][2..line[8].len() - 1].parse::<isize>().unwrap();
        let beacon_y = line[9][2..line[9].len()].parse::<isize>().unwrap();

        println!("S: ({}, {}) B: ({}, {})", sensor_x, sensor_y, beacon_x, beacon_y);

        let dist = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();

        println!("Dist: {dist}");

        for (i, x) in (sensor_x-dist..sensor_x+dist).enumerate() {
            //println!("x = {}", x);
            let i = i as isize;
            for y in x-i..=x+i {
                println!("({}, {})", x, y);
                *grid.get_mut(x, y) = CellState::Searched;
            }
        }
    }

    let mut count = 0;
    for x in 0..grid.len_x() {
        if *grid.get(x as isize, 2000000) == CellState::Unsearched {
            count += 1;
        }
    }

    println!("{}", count);
}