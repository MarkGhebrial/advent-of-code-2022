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

    let mut sum = 0;

    for column in 0..forest.len() {
        for row in 0..forest[0].len() {
            if row == 0 || row == forest[0].len() - 1 { sum += 1; continue; }
            if column == 0 || column == forest.len() - 1 { sum += 1; continue; }

            let this_tree = forest[column][row];
            if this_tree == 0 { continue; }

            let mut west_max = 0;
            for tree in forest[column][0..row].iter() {
                if *tree > west_max { west_max = *tree; }
            }

            let mut east_max = 0;
            for tree in forest[column][row + 1..].iter() {
                if *tree > east_max { east_max = *tree; }
            }

            let mut north_max = 0;
            for col in forest[0..column].iter() {
                if col[row] > north_max { north_max = col[row]; }
            }

            let mut south_max = 0;
            for col in forest[column + 1..].iter() {
                if col[row] > south_max { south_max = col[row]; }
            }

            if this_tree > west_max || this_tree > east_max || this_tree > north_max || this_tree > south_max {
                sum += 1;
            }

            println!("{west_max} {east_max} {}", forest[column][row]);
        }
    }

    println!("{}", sum);
}