use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
enum Node {
    File {
        size: usize,
    },
    Directory {
        name: String,
        children: RefCell<Vec<Rc<Node>>>,
    },
}

impl Node {
    fn get_size(&self) -> usize {
        match self {
            Node::File { size } => *size,
            Node::Directory { name: _, children } => {
                let mut size = 0;
                for child in children.borrow().iter() {
                    size += child.get_size();
                }
                size
            }
        }
    }
}

fn sum_size(dir: &Node) -> usize {
    let mut sum = 0;

    match dir {
        Node::File { size: _ } => {}
        Node::Directory { name: _, children } => {
            let size = dir.get_size();
            if size <= 100000 {
                sum += size;
            }

            for child in children.borrow().iter() {
                sum += sum_size(&child);
            }
        }
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let root = Rc::new(Node::Directory {
        name: "root".to_string(),
        children: RefCell::new(Vec::new()),
    });

    let mut curr_dir = "".to_string();

    for line in input.split("\n") {
        if line.trim() == "" {
            break;
        } // Don't parse blank lines!

        match &line[0..1] {
            "$" => match &line[2..=3] {
                "cd" => {
                    let new_dir = line.split(" ").collect::<Vec<&str>>()[2];

                    match new_dir {
                        "/" => curr_dir = "".to_string(), // Go back to the root directory
                        ".." => {
                            // Go up a directory
                            let path: Vec<&str> = curr_dir.split("/").collect();

                            let mut out = String::new();
                            for dir in path[0..path.len() - 1].iter() {
                                if dir != &"" {
                                    out += &("/".to_owned() + dir);
                                }
                            }
                            println!("{}", curr_dir);
                            curr_dir = out;
                        }
                        _ => curr_dir += &("/".to_owned() + new_dir),
                    }
                }
                "ls" => println!("LS"),
                c => println!("Command {} not valid", c),
            },
            _ => {
                let args: Vec<&str> = line.split(" ").collect();
                let path: Vec<&str> = curr_dir.split("/").collect();

                let mut working_node = Rc::clone(&root);
                for dir in path {
                    if let Node::Directory {
                        name: _,
                        ref children,
                    } = *working_node.clone()
                    {
                        for child in children.borrow().iter() {
                            if let Node::Directory {
                                ref name,
                                ref children,
                            } = **child
                            {
                                if name == dir {
                                    working_node = Rc::clone(&child);
                                    break;
                                }
                            } //BDELYGMIA IS NOT A WORD.
                        }
                    } else {
                        panic!("Uh Oh");
                    }
                }

                if let Node::Directory {
                    name: _,
                    ref children,
                } = *working_node
                {
                    if args[0] == "dir" {
                        children.borrow_mut().push(Rc::new(Node::Directory {
                            name: String::from(args[1]),
                            children: RefCell::new(Vec::new()),
                        }));
                    } else {
                        children.borrow_mut().push(Rc::new(Node::File {
                            size: args[0].parse::<usize>().unwrap(),
                        }));
                    }
                }
            }
        }
    }

    let sum = sum_size(&root);
    println!("{}", sum);
}
