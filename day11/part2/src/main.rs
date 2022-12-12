use num_bigint::{BigUint, ToBigUint};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;

struct Monkey<'a> {
    items: VecDeque<BigUint>,
    operation: &'a dyn Fn(&BigUint) -> BigUint,
    condition: &'a dyn Fn(&BigUint) -> bool,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize,
}

/// For items in transit between monkeys
struct FlyingItem {
    destination: usize,
    worry_level: BigUint,
}

fn big(num: usize) -> BigUint {
    ToBigUint::to_biguint(&num).unwrap()
}

impl<'a> Monkey<'a> {
    fn take_turn(&mut self) -> Vec<FlyingItem> {
        let mut out = vec![];

        for item in self.items.iter() {
            let mut item = (self.operation)(item);
            //item /= 3;
            out.push(FlyingItem {
                destination: if (self.condition)(&item) {
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

    fn catch(&mut self, item: BigUint) {
        self.items.push_back(item);
    }

    // fn reduce(&mut self) {
    //     for item in self.items.iter_mut() {
    //         if item % big(7) == big(0) && item % big(11) == big(0) && item % big(13) == big(0) && item % big(3) == big(0) && item % big(17) == big(0) && item % big(2) == big(0) && item % big(5) == big(0) && item % big(19) == big(0) {
    //             *item = big(7) * big(11) * big(13) * big(3) * big(17) * big(2) * big(5) * big(19);
    //         }
    //     }
    // }
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    let mut monkey_list: Vec<RefCell<Monkey>> = vec![];

    for (i, monkey_str) in input.trim().split("\n\n").enumerate() {
        let monkey_str = monkey_str.trim().split("\n").collect::<Vec<&str>>();

        let mut items = VecDeque::new();
        let unparsed_items = monkey_str[1].trim().split(" ").collect::<Vec<&str>>();
        for num in &unparsed_items[2..] {
            items.push_back(num[..2].parse::<BigUint>().unwrap());
        }

        //let big = |x| ToBigUint::to_biguint(&x).unwrap();

        let monkey = match i {
            0 => Monkey {
                items,
                operation: &|old| old * big(11),
                condition: &|x| x % big(7) == big(0),
                true_monkey: 6,
                false_monkey: 2,
                inspections: 0,
            },
            1 => Monkey {
                items,
                operation: &|old| old + big(1),
                condition: &|x| x % big(11) == big(0),
                true_monkey: 5,
                false_monkey: 0,
                inspections: 0,
            },
            2 => Monkey {
                items,
                operation: &|old| old * big(7),
                condition: &|x| x % big(13) == big(0),
                true_monkey: 4,
                false_monkey: 3,
                inspections: 0,
            },
            3 => Monkey {
                items,
                operation: &|old| old + big(3),
                condition: &|x| x % big(3) == big(0),
                true_monkey: 1,
                false_monkey: 7,
                inspections: 0,
            },
            4 => Monkey {
                items,
                operation: &|old| old + big(6),
                condition: &|x| x % big(17) == big(0),
                true_monkey: 3,
                false_monkey: 7,
                inspections: 0,
            },
            5 => Monkey {
                items,
                operation: &|old| old + big(5),
                condition: &|x| x % big(2) == big(0),
                true_monkey: 0,
                false_monkey: 6,
                inspections: 0,
            },
            6 => Monkey {
                items,
                operation: &|old| old * old,
                condition: &|x| x % big(5) == big(0),
                true_monkey: 2,
                false_monkey: 4,
                inspections: 0,
            },
            7 => Monkey {
                items,
                operation: &|old| old + big(7),
                condition: &|x| x % big(19) == big(0),
                true_monkey: 5,
                false_monkey: 1,
                inspections: 0,
            },
            _ => panic!("Invalid monkey index!"),
        };

        monkey_list.push(RefCell::new(monkey));
    }

    let reduce = |monkey: &mut Monkey| {
        for item in monkey.items.iter_mut() {
            let mut is_divisible = false;
            is_divisible = &*item % &big(7 * 11 * 13 * 3 * 17 * 2 * 5 * 19) == big(0);

            if is_divisible {
                *item = big(7 * 11 * 13 * 3 * 17 * 2 * 5 * 19);
            }
        }
    };

    for round in 0..10000 {
        for monkey in monkey_list.iter() {
            let mut monkey = monkey.borrow_mut();
            reduce(&mut monkey);
            for flying_item in monkey.take_turn() {
                monkey_list[flying_item.destination]
                    .borrow_mut()
                    .catch(flying_item.worry_level);
            }
        }
        println!("{}", round);
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
