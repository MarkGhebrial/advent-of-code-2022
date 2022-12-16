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

    let searched = |p: &Point| -> bool {
        for sensor in sensors.iter() {
            if *p == sensor.beacon {
                continue;
            }
            if sensor.dist_to(&p) <= sensor.dist_to_beacon() || *p == sensor.position {
                //println!("To point: {}, to Beacon: {}", sensor.dist_to(&point), sensor.dist_to_beacon());
                return true;
            }
        }
        
        false
    };

    for sensor in sensors.iter() {
        let lower_x = sensor.position.x - sensor.dist_to_beacon() - 1;
        let upper_x = sensor.position.x + sensor.dist_to_beacon() + 1;
        for (i, x) in (lower_x..=upper_x).enumerate() {
            let i = i as isize;

            let y1 = sensor.position.y - i;
            let y2 = sensor.position.y + i;

            let p1 = Point {
                x,
                y: y1,
            };
            let p2 = Point {
                x,
                y: y2,
            };

            if p1.x >= 0 && p1.x <= 4000000 && p1.y >= 0 && p1.y <= 4000000 && !searched(&p1) {
                println!("Point 1's ({:?}) frequency is {}", p1, p1.x * 4000000 + p1.y);
                return;
            }
            if p2.x >= 0 && p2.x <= 4000000 && p2.y >= 0 && p2.y <= 4000000 && !searched(&p2) {
                println!("Point 2's ({:?}) frequency is {}", p2, p2.x * 4000000 + p2.y);
                return;
            }
        }
    }
}