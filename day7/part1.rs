use std::fs;
use std::rc::Rc;

#[derive(Debug)]
enum Node {
    File { size: usize },
    Directory { name: String, children: Rc<Vec<Node>> },
}

fn main() {
    let input = fs::read_to_string("test.txt").unwrap();

    let mut root = Node::Directory { name: "root".to_string(), children: Vec::new() };
    
    let mut curr_dir = "".to_string();

    for line in input.split("\n") {
        if line.trim() == "" { break; } // Don't parse blank lines!

        match &line[0..1] {
            "$" => match &line[2..=3] {
                "cd" => {
                    /*let new_dir = line.split(" ").collect::<Vec<&str>>()[2];
                    
                    if let Node::Directory{ name, children } = curr_dir {
                        for node in children {
                            if name == new_dir {
                                curr_dir = node;
                            }
                        }
                    }*/
                    let new_dir = line.split(" ").collect::<Vec<&str>>()[2];

                    match new_dir {
                        "/" => curr_dir = "".to_string(), // Go back to the root directory
                        ".." => { // Go up a directory
                            let path: Vec<&str> = curr_dir.split("/").collect();

                            let mut out = String::new();
                            for dir in path[0..path.len() - 1].iter() {
                                if dir != &"" {
                                    out += &("/".to_owned() + dir);
                                }
                            }
                            println!("{}", curr_dir);
                            curr_dir = out;
                        },
                        _ => curr_dir += &("/".to_owned() + new_dir),
                    }
                },
                "ls" => println!("LS"),
                c => println!("Command {} not valid", c),
            }
            _ => {
                let args: Vec<&str> = line.split(" ").collect();
                let path: Vec<&str> = curr_dir.split("/").collect();

                let mut working_node = Rc::clone(&root);
                for dir in path {
                    if let Node::Directory{ ref name, mut children } = *working_node {
                        for child in children {
                            if let Node::Directory{ ref name, ref children } = child {
                                if name == dir {
                                    working_node = &child;
                                    break;
                                }
                            } //BDELYGMIA IS NOT A WORD.
                        }
                    } else {
                        panic!("Uh Oh");
                    }
                }

                let mut working_node = &mut working_node;

                if let Node::Directory{ ref name, ref mut children } = working_node {
                    if args[0] == "dir" {
                        children.push(Node::Directory{ name: String::from(args[1]), children: Vec::new() });
                    } else {
                        children.push(Node::File { size: args[0].parse::<usize>().unwrap() });
                    }
                }
            }
        }
    }

    println!("{:?}", root);
}