use std::fs;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
enum LineDir {
    Up,
    Right,
}
use LineDir::*;

#[derive(Debug)]
struct Line {
    start: Point,
    len: usize,
    dir: LineDir,
}

impl Line {
    fn end(&self) -> Point {
        match &self.dir {
            Up => Point {
                x: self.start.x,
                y: self.start.y + self.len - 1,
            },
            Right => Point {
                x: self.start.x + self.len - 1,
                y: self.start.y,
            },
        }
    }

    fn intersects(&self, line: &Line) -> bool {
        if self.start == line.start {
            return true;
        }

        return ((self.start.y >= line.start.y && self.start.y <= line.end().y)
            || (self.end().y >= line.start.y && self.end().y <= line.end().y))
            && ((self.start.x >= line.start.x && self.start.x <= line.end().x)
                || (self.end().x >= line.start.x && self.end().x <= line.end().x));

        // true || true
        // false ||
        // return (self.start.x >= line.start.x || self.end().x <= line.end().x)
        //     && (self.start.y >= line.start.y || self.end().y <= line.end().y);
    }
}

#[derive(Debug, Clone)]
enum RockShape {
    Minus,
    Cross,
    Ell, // It looks like an "L", which is pronounced "ell"
    Pipe,
    Square,
}

impl RockShape {
    fn height(&self) -> usize {
        match self {
            RockShape::Minus => 1,
            RockShape::Cross => 3,
            RockShape::Ell => 3,
            RockShape::Pipe => 4,
            RockShape::Square => 2,
        }
    }

    fn width(&self) -> usize {
        match self {
            RockShape::Minus => 4,
            RockShape::Cross => 3,
            RockShape::Ell => 3,
            RockShape::Pipe => 1,
            RockShape::Square => 2,
        }
    }
}

struct RockShapeIter {
    index: usize,
}

impl RockShapeIter {
    fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for RockShapeIter {
    type Item = RockShape;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.index % 5 {
            0 => RockShape::Minus,
            1 => RockShape::Cross,
            2 => RockShape::Ell,
            3 => RockShape::Pipe,
            4 => RockShape::Square,
            _ => panic!("Something's fishy"),
        };

        self.index += 1;

        Some(out)
    }
}

#[derive(Debug)]
struct Rock {
    shape: RockShape,
    position: Point,
    //at_rest: bool,
}

impl Rock {
    fn new(shape: RockShape, stack_height: usize) -> Self {
        let height = shape.height();
        Self {
            shape,
            position: Point {
                x: 2,
                y: stack_height + 2 + height,
            },
            //at_rest: false
        }
    }

    fn blow_left(&mut self) {
        if self.position.x > 0 {
            println!("Blew left");
            self.position.x -= 1;
        } else {
            println!("Tried to blow left");
        }
    }

    fn blow_right(&mut self) {
        if self.position.x + self.shape.width() < 7 {
            println!("Blew right");
            self.position.x += 1;
        } else {
            println!("Tried to blow right");
        }
    }

    fn move_down(&mut self) {
        self.position.y -= 1;
    }

    fn move_up(&mut self) {
        self.position.y += 1;
    }

    fn to_lines(&self) -> Vec<Line> {
        match self.shape {
            RockShape::Minus => vec![Line {
                start: Point {
                    x: self.position.x,
                    y: self.position.y,
                },
                len: 4,
                dir: LineDir::Right,
            }],
            RockShape::Cross => vec![
                Line {
                    start: Point {
                        x: self.position.x,
                        y: self.position.y - 1,
                    },
                    len: 3,
                    dir: LineDir::Right,
                },
                Line {
                    start: Point {
                        x: self.position.x + 1,
                        y: self.position.y - 2,
                    },
                    len: 3,
                    dir: LineDir::Up,
                },
            ],
            RockShape::Ell => vec![
                Line {
                    start: Point {
                        x: self.position.x,
                        y: self.position.y - 2,
                    },
                    len: 3,
                    dir: LineDir::Right,
                },
                Line {
                    start: Point {
                        x: self.position.x + 2,
                        y: self.position.y - 2,
                    },
                    len: 3,
                    dir: LineDir::Up,
                },
            ],
            RockShape::Pipe => vec![Line {
                start: Point {
                    x: self.position.x,
                    y: self.position.y - 3,
                },
                len: 4,
                dir: LineDir::Up,
            }],
            RockShape::Square => vec![
                Line {
                    start: Point {
                        x: self.position.x,
                        y: self.position.y,
                    },
                    len: 2,
                    dir: LineDir::Right,
                },
                Line {
                    start: Point {
                        x: self.position.x,
                        y: self.position.y - 1,
                    },
                    len: 2,
                    dir: LineDir::Right,
                },
            ],
        }
    }

    fn intersects(&self, rock: &Rock) -> bool {
        for line in self.to_lines() {
            for line2 in rock.to_lines() {
                if line.intersects(&line2) {
                    return true;
                }
            }
        }

        false
    }

    fn touching_ground(&self) -> bool {
        self.position.y < self.shape.height()
    }
}

fn main() {
    let input = fs::read_to_string("test.txt").unwrap();

    let mut rocks: Vec<Rock> = Vec::new();
    let mut stack_height = 0;

    let shape_iter = RockShapeIter::new();
    let jet_iter = Rc::new(input.trim().chars().cycle());

    for (i, shape) in (0..2022).zip(shape_iter) {
        let mut new_rock = Rock::new(shape, stack_height);

        println!("Stack height is {}", stack_height);
        println!(
            "Rock {} spawned at {:?}",
            i, new_rock.position
        );

        //'move_rock: for c in input.trim().chars().cycle() {
        'move_rock: for c in *Rc::clone(&jet_iter) {
            match c {
                '<' => new_rock.blow_left(),
                '>' => new_rock.blow_right(),
                _ => panic!("Expected '<' or '>', found '{}'", c),
            }
            //println!("{:?}", new_rock.position);

            if new_rock.touching_ground() {
                println!(
                    "Rock {} touched ground, ended at {:?}",
                    i, new_rock.position
                );
                stack_height = new_rock.position.y + 1;
                rocks.push(new_rock);
                break 'move_rock;
            }

            new_rock.move_down();
            //println!("{:?}", new_rock.position);
            for (j, rock) in rocks.iter().enumerate() {
                if new_rock.intersects(&rock) {
                    println!(
                        "Rock {} intersects with rock {}, ended at {:?}",
                        i, j, new_rock.position
                    );
                    new_rock.move_up();
                    stack_height = new_rock.position.y + 1;
                    rocks.push(new_rock);
                    break 'move_rock;
                }
            }
        }
    }

    println!("{}", stack_height);
}
