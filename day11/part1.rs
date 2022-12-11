use std::fs;
use std::collections::VecDeque;
use std::cell::RefCell;

struct Monkey<'a> {
    items: VecDeque<usize>,
    operation: &'a dyn Fn(usize) -> usize,
    condition: &'a dyn Fn(usize) -> bool,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize
}

/// For items in transit between monkeys
struct FlyingItem {
    destination: usize,
    worry_level: usize,
}

impl<'a> Monkey<'a> {
    fn take_turn(&mut self) -> Vec<FlyingItem> {
        let mut out = vec![];

        for item in self.items.iter() {
            let mut item = (self.operation)(*item);
            item /= 3;
            out.push(FlyingItem{
                destination: if (self.condition)(item) {
                    self.true_monkey
                } else {
                    self.false_monkey
                },
                worry_level: item,
            });

            self.inspections += 1;
        }

        self.items.clear();

        out
    }

    fn catch(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut monkey_list: Vec<RefCell<Monkey>> = vec![];

    for (i, monkey_str) in input.trim().split("\n\n").enumerate() {
        let monkey_str = monkey_str.trim().split("\n").collect::<Vec<&str>>();

        let mut items = VecDeque::new();
        let unparsed_items = monkey_str[1].trim().split(" ").collect::<Vec<&str>>();
        for num in &unparsed_items[2..] {
            items.push_back(num[..2].parse::<usize>().unwrap());
        }

        let monkey = match i {
            0 => Monkey {
                items,
                operation: &|old| old * 11,
                condition: &|x| x % 7 == 0,
                true_monkey: 6,
                false_monkey: 2,
                inspections: 0
            },
            1 => Monkey {
                items,
                operation: &|old| old + 1,
                condition: &|x| x % 11 == 0,
                true_monkey: 5,
                false_monkey: 0,
                inspections: 0
            },
            2 => Monkey {
                items,
                operation: &|old| old * 7,
                condition: &|x| x % 13 == 0,
                true_monkey: 4,
                false_monkey: 3,
                inspections: 0
            },
            3 => Monkey {
                items,
                operation: &|old| old + 3,
                condition: &|x| x % 3 == 0,
                true_monkey: 1,
                false_monkey: 7,
                inspections: 0
            },
            4 => Monkey {
                items,
                operation: &|old| old + 6,
                condition: &|x| x % 17 == 0,
                true_monkey: 3,
                false_monkey: 7,
                inspections: 0
            },
            5 => Monkey {
                items,
                operation: &|old| old + 5,
                condition: &|x| x % 2 == 0,
                true_monkey: 0,
                false_monkey: 6,
                inspections: 0
            },
            6 => Monkey {
                items,
                operation: &|old| old * old,
                condition: &|x| x % 5 == 0,
                true_monkey: 2,
                false_monkey: 4,
                inspections: 0
            },
            7 => Monkey {
                items,
                operation: &|old| old + 7,
                condition: &|x| x % 19 == 0,
                true_monkey: 5,
                false_monkey: 1,
                inspections: 0
            },
            _ => panic!("Invalid monkey index!"),
        };

        monkey_list.push(RefCell::new(monkey));
    }

    for _round in 0..20 {
        for monkey in monkey_list.iter() {
            let mut monkey = monkey.borrow_mut();
            //println!("{:?}", monkey.items);
            for flying_item in monkey.take_turn() {
                monkey_list[flying_item.destination].borrow_mut().catch(flying_item.worry_level);
            }
        }
        //println!("");
    }

    let mut max = 0;
    let mut second_max = 0;
    for monkey in monkey_list.iter() {
        let monkey = monkey.borrow();

        println!("{}", monkey.inspections);

        if monkey.inspections > max {
            second_max = max;
            max = monkey.inspections;
        } else if monkey.inspections > second_max {
            second_max = monkey.inspections;
        }
    }

    println!("{} * {} = {}", max, second_max, max * second_max);
}