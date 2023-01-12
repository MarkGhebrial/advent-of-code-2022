use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let input = input
        .trim()
        .split("\n")
        .map(|line| line.trim().parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    // Begin mixing

    let mut new_vec: Vec<isize> = vec![];
    input.iter().for_each(|num| new_vec.push(*num));

    for num in input.iter() {
        println!("{}", num);

        let mut index = 0;
        for (i, n) in new_vec.iter().enumerate() {
            if n == num {
                index = i as isize;
            }
            break;
        }

        println!("{}", ((index + num) % input.len() as isize) );

        if *num > 0 {
            new_vec.insert(((index + num) % input.len() as isize) as usize, *num);
            new_vec.remove(index as usize);
        } else {
            new_vec.remove(index as usize);
            new_vec.insert(-(((index + num) % input.len() as isize)) as usize, *num);
        }
    }

    let item1 = new_vec[1000 % new_vec.len()];
    let item2 = new_vec[2000 % new_vec.len()];
    let item3 = new_vec[3000 % new_vec.len()];
    println!("{}", item1 + item2 + item3);
}
