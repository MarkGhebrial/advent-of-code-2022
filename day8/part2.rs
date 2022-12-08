use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut forest: Vec<Vec<usize>> = vec![];

    // Read the input
    for (i, line) in input.trim().split("\n").enumerate() {
        forest.push(vec![]);
        for c in line.chars() {
            forest[i].push(String::from(c).parse::<usize>().unwrap());
        }
    }

    let mut max_score = 0;

    for column in 0..forest[0].len() {
        for row in 0..forest.len() {
            if row == 0 || row == forest[0].len() - 1 { continue; }
            if column == 0 || column == forest.len() - 1 { continue; }

            let this_tree = forest[row][column];
            if this_tree == 0 { continue; }

            let mut west_dist = 0;
            for tree in forest[row][..column].iter().rev() {
                west_dist += 1;
                if *tree >= this_tree { break; }
            }

            let mut east_dist = 0;
            for tree in forest[row][column + 1..].iter() {
                east_dist += 1;
                if *tree >= this_tree { break; }
            }

            let mut north_dist = 0;
            for roww in forest[..row].iter().rev() {
                north_dist += 1;
                if roww[column] >= this_tree { break; }
            }

            let mut south_dist = 0;
            for roww in forest[row + 1..].iter() {
                south_dist += 1;
                if roww[column] >= this_tree { break; }
            }

            println!("{}", this_tree);

            let scenic_score = west_dist * east_dist * north_dist * south_dist;
            println!("At ({}, {}): {} * {} * {} * {} = {}", row, column, west_dist, east_dist, north_dist, south_dist, scenic_score);

            if scenic_score > max_score { max_score = scenic_score; }
        }
    }

    println!("{}", max_score);
}