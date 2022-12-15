use std::fs;

#[derive(Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Sensor {
    position: Point,
    beacon: Point,
}

impl Sensor {
    fn parse_from_str(line: &str) -> Self {
        let line = line.trim().split(" ").collect::<Vec<&str>>();
        let x = line[2][2..line[2].len() - 1].parse::<isize>().unwrap();
        let y = line[3][2..line[3].len() - 1].parse::<isize>().unwrap();

        let beacon_x = line[8][2..line[8].len() - 1].parse::<isize>().unwrap();
        let beacon_y = line[9][2..line[9].len()].parse::<isize>().unwrap();

        Self {
            position: Point {
                x,
                y,
            },
            beacon: Point {
                x: beacon_x,
                y: beacon_y,
            },
        }
    }

    fn dist_to(&self, p: &Point) -> isize {
        (self.position.x - p.x).abs() + (self.position.y - p.y).abs()
    }

    fn dist_to_beacon(&self) -> isize {
        self.dist_to(&self.beacon)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut sensors: Vec<Sensor> = Vec::new();

    for line in input.trim().split("\n") {
        
        let sensor = Sensor::parse_from_str(&line.trim());
        sensors.push(sensor);
    }

    let mut count = 0;
    for x in -7000000..=12000000 {
        let point = Point {
            x,
            y: 2000000,
        };
        'fdsa: for sensor in sensors.iter() {
            if point == sensor.beacon {
                continue;
            }
            if sensor.dist_to(&point) <= sensor.dist_to_beacon() || point == sensor.position {
                println!("To point: {}, to Beacon: {}", sensor.dist_to(&point), sensor.dist_to_beacon());
                count += 1;
                break 'fdsa;
            }
        }
    }

    println!("{}", count);
}