pub fn main() {
    let mut monkeys: Vec<Monkey> = vec![
        Monkey {
            items: vec![71, 86],
            operation: (2, 13),
            test: 19,
            target_monkey_1: 6,
            target_monkey_2: 7,
            inspection: 0,
        }, 
        Monkey {
            items: vec![66, 50, 90, 53, 88, 85],
            operation: (1, 3),
            test: 2,
            target_monkey_1: 5,
            target_monkey_2: 4,
            inspection: 0,
        }, 
        Monkey {
            items: vec![97, 54, 89, 62, 84, 80, 63],
            operation: (1, 6),
            test: 13,
            target_monkey_1: 4,
            target_monkey_2: 1,
            inspection: 0,
        }, 
        Monkey {
            items: vec![82, 97, 56, 92],
            operation: (1, 2),
            test: 5,
            target_monkey_1: 6,
            target_monkey_2: 0,
            inspection: 0,
        }, 
        Monkey {
            items: vec![50, 99, 67, 61, 86],
            operation: (3, 0),
            test: 7,
            target_monkey_1: 5,
            target_monkey_2: 3,
            inspection: 0,
        }, 
        Monkey {
            items: vec![61, 66, 72, 55, 64, 53, 72, 63],
            operation: (1, 4),
            test: 11,
            target_monkey_1: 3,
            target_monkey_2: 0,
            inspection: 0,
        }, 
        Monkey {
            items: vec![59, 79, 63],
            operation: (2, 7),
            test: 17,
            target_monkey_1: 2,
            target_monkey_2: 7,
            inspection: 0,
        }, 
        Monkey {
            items: vec![55],
            operation: (1, 7),
            test: 3,
            target_monkey_1: 2,
            target_monkey_2: 1,
            inspection: 0,
        }, 
    ];
    
    let mut monkeys2 = monkeys.clone();

    let level = monkey_business_level(&mut monkeys, false);
    println!("Part 1: The level of monkey business after 20 rounds: {:?}", level);
    let level = monkey_business_level(&mut monkeys2, true);
    println!("Part 2: The level of monkey business after 10000 rounds: {:?}", level);
}

fn monkey_business_level(monkeys: &mut Vec<Monkey>, round_two: bool) -> u128 {
    let mut round_index = 20;
    let least_common_multiple = monkeys.iter().map(|m| m.test).product::<u128>();
    if round_two {
        round_index = 10000;
    }
    let mut simulations: Vec<(usize, u128)> = Vec::new();
    for _round in 0..round_index {
        for i_monkey in 0..monkeys.len() {
            simulations.clear();
            let monkey = monkeys.get_mut(i_monkey).unwrap();
            monkey.items.iter_mut().for_each(|item| {
                monkey.inspection += 1;
                match monkey.operation.0 {
                    1 => {
                        if round_two {
                            *item = item.wrapping_add(monkey.operation.1) % least_common_multiple;
                        } else {
                            *item = item.wrapping_add(monkey.operation.1);
                        }
                    }
                    2 => {
                        if round_two {
                            *item = item.wrapping_mul(monkey.operation.1) % least_common_multiple;
                        } else {
                            *item = item.wrapping_mul(monkey.operation.1);
                        }
                    }
                    _ => {
                        if round_two {
                            *item = item.wrapping_mul(*item) % least_common_multiple;
                        } else {
                            *item = item.wrapping_mul(*item);
                        }
                    }
                }
                if !round_two {
                    *item /= 3;
                }
                if *item % monkey.test == 0 {
                    simulations.push((monkey.target_monkey_1, *item));
                } else {
                    simulations.push((monkey.target_monkey_2, *item));
                }
            });
            monkey.items.clear();
            for (target, item) in &simulations {
                monkeys[*target].items.push(*item);
            }
        }
    }

    let mut top_2 = monkeys
        .iter()
        .map(|monkey| monkey.inspection)
        .collect::<Vec<u128>>();
    top_2.sort();

    top_2
        .iter()
        .rev()
        .take(2)
        .product::<u128>()
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u128>,
    test: u128,
    operation: (u8, u128),
    target_monkey_1: usize,
    target_monkey_2: usize,
    inspection: u128,
}