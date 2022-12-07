use std::fs;

fn main() {
    let transmission = fs::read_to_string("input.txt").unwrap();

    let mut index = 0;

    for i in 0..transmission.len() {
        let mut has_double = false;        
        for c in transmission[i..i+4].chars() {
            if transmission[i..i+4].matches(c).collect::<Vec<&str>>().len() > 1 {
                has_double = true;
                break
            }
        }
        if !has_double { break; }
        index += 1;
    }

    println!("{}", index + 4);
}