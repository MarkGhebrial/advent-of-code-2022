use std::fs;

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn new() -> Self {
        let mut v = vec![];
        for _ in 0..200 {
            let mut v1 = vec![];
            for _ in 0..600 {
                v1.push('.');
            }
            v.push(v1);
        }

        Grid(v)
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut char {
        &mut self.0[y][x]
    }

    fn get(&self, x: usize, y: usize) -> &char {
        &self.0[y][x]
    }

    fn len_y(&self) -> usize {
        self.0.len()
    }

    fn len_x(&self) -> usize {
        self.0[0].len()
    }

    fn print(&self) {
        for v in self.0.iter() {
            for c in v {
                print!("{}", c);
            }
            println!("");
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut grid = Grid::new();

    for line in input.trim().split("\n") {
        let points = line
            .trim()
            .split(" -> ")
            .map(|p| {
                p.split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        println!("{:?}", points);

        for i in 0..points.len() - 1 {
            for j in points[i][0]..=points[i + 1][0] {
                *grid.get_mut(j, points[i][1]) = '#';
            }

            for j in points[i + 1][0]..=points[i][0] {
                *grid.get_mut(j, points[i][1]) = '#';
            }

            for k in points[i][1]..=points[i + 1][1] {
                *grid.get_mut(points[i][0], k) = '#';
            }

            for k in points[i + 1][1]..=points[i][1] {
                *grid.get_mut(points[i][0], k) = '#';
            }
        }
    }
    *grid.get_mut(500, 0) = '+';

    grid.print();

    let mut curr_x = 500;
    let mut curr_y = 0;
    let mut index = 0;
    'new_sand: loop {
        // For each new sand

        'move_sand: loop {
            // Move the sand
            println!("({}, {})", curr_x, curr_y);

            // We're in the void
            if curr_y == grid.len_y() - 1 {
                break 'new_sand;
            }

            let down = *grid.get(curr_x, curr_y + 1);
            let left = *grid.get(curr_x - 1, curr_y + 1);
            let right = *grid.get(curr_x + 1, curr_y + 1);

            println!("Down: {}, Left: {}, Right: {}", down, left, right);

            if down == '.' {
                println!("Moving down");
                curr_y += 1;
            } else if left == '.' {
                println!("Moving left");
                curr_x -= 1;
                curr_y += 1;
            } else if right == '.' {
                println!("Moving right");
                curr_x += 1;
                curr_y += 1;
            } else {
                // Sand has landed
                *grid.get_mut(curr_x, curr_y) = 'o';
                index += 1;
                break 'move_sand;
            }
        }

        curr_x = 500;
        curr_y = 0;
    }

    grid.print();

    println!("{}", index);
}
