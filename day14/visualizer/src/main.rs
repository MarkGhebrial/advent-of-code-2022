use std::fs;
use std::thread;
use std::time::Duration;
use std::io::Write;
use console::Term;
use console::style;

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn new() -> Self {
        let mut v = vec![];
        for _ in 0..750 {
            let mut v1 = vec![];
            for _ in 0..1000 {
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
    let input = fs::read_to_string("../input.txt").unwrap();

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

    let mut term = Term::buffered_stdout();
    let term_size = term.size_checked();
    let (y_size, x_size) = match term_size {
        Some(s) => s,
        None => (10, 10),
    };

    let mut curr_x: usize = 500;
    let mut curr_y: usize = 0;
    let mut index = 0;
    'new_sand: loop {
        // For each new sand

        'move_sand: loop {
            // Move the sand

            // Draw the grid
            let y_bounds = if curr_y >= (y_size/2) as usize {
                ((curr_y - (y_size/2) as usize), (curr_y + (y_size/2) as usize))
            } else {
                (0, y_size as usize)
            };

            term.clear_screen().unwrap();
            for y in y_bounds.0..y_bounds.1 {
                let mut line = String::new();
                for x in (curr_x - (x_size/2) as usize)..(curr_x + (x_size/2) as usize) {
                    if (x, y) == (curr_x, curr_y) {
                        line += &format!("{}", style("o").red());
                    } else {
                        line += &match *grid.get(x, y) {
                            'o' => format!("{}", style("o").yellow()),
                            '#' => format!("{}", style("#").bold()),
                            '+' => format!("{}", style("+").green()),
                            '.' => format!("{}", " "),
                            c => format!("{}", c),
                        };
                    }
                    if line.len() >= x_size as usize { continue }
                }
                write!(term, "{}", line).unwrap();
                term.move_cursor_down(1).unwrap();
                term.move_cursor_left(1000000000).unwrap();
            }
            term.flush().unwrap();
            thread::sleep(Duration::from_millis(50));
            term.move_cursor_to(0, 0);

            // We're in the void
            if curr_y == grid.len_y() - 1 {
                break 'new_sand;
            }

            let down = *grid.get(curr_x, curr_y + 1);
            let left = *grid.get(curr_x - 1, curr_y + 1);
            let right = *grid.get(curr_x + 1, curr_y + 1);

            if down == '.' {
                curr_y += 1;
            } else if left == '.' {
                curr_x -= 1;
                curr_y += 1;
            } else if right == '.' {
                curr_x += 1;
                curr_y += 1;
            } else {
                // Sand has landed
                *grid.get_mut(curr_x, curr_y) = 'o';
                index += 1;
                thread::sleep(Duration::from_millis(300));
                break 'move_sand;
            }
        }

        curr_x = 500;
        curr_y = 0;
    }

    //grid.print();

    //println!("{}", index);
}
