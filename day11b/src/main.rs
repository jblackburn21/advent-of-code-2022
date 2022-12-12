use std::collections::VecDeque;

struct Monkey {
    item_worry_levels: VecDeque<u64>,
    op: fn(u64, u64) -> u64, // TODO: how to do partial fun?
    op_num: Option<u64>,
    test_divisible_by: u64,
    test_true_monkey: usize,
    test_false_monkey: usize,
    items_held: u64
}

impl Monkey {
    fn new(starting_item_worry_levels: Vec<u64>,
           op: fn(u64, u64) -> u64,
           op_num: Option<u64>,
           test_divisible_by: u64,
           test_true_monkey: usize,
           test_false_monkey: usize,
           init_items_held: u64) -> Monkey {

        Monkey {
            item_worry_levels: VecDeque::from(starting_item_worry_levels),
            op,
            op_num,
            test_divisible_by,
            test_true_monkey,
            test_false_monkey,
            items_held: init_items_held
        }
    }

    fn push_back(&mut self, item_worry_level: u64) {
        self.item_worry_levels.push_back(item_worry_level);
        self.items_held += 1;
    }

    fn pop_front(&mut self) -> Option<u64> {
        return self.item_worry_levels.pop_front();
    }
}

pub fn main() {


    let mut monkeys: Vec<Monkey> = include_str!("../input.txt")
        .split("\n\n")
        .map(|chunk| {

            let starting_items: Vec<u64> = chunk.lines().nth(1).unwrap()
                .split_once(":").unwrap().1.trim()
                .split(",")
                .map(|si| si.trim().parse::<u64>().unwrap())
                .collect();

            let op_line: Vec<&str> = chunk.lines().nth(2).unwrap()
                .split_once("=").unwrap().1.trim()
                .split(" ")
                .collect();

            let add = op_line[1].eq("+");

            let op = if add { add_op } else { mult_op };

            let op_num = op_line[2].parse::<u64>().ok();

            let divisible_by = chunk.lines().nth(3).unwrap()
                .split(" ").last()
                .map(|s| s.parse::<u64>().unwrap())
                .unwrap();

            let true_monkey = chunk.lines().nth(4).unwrap()
                .split(" ").last()
                .map(|s| s.parse::<usize>().unwrap())
                .unwrap();

            let false_monkey = chunk.lines().nth(5).unwrap()
                .split(" ").last()
                .map(|s| s.parse::<usize>().unwrap())
                .unwrap();

            let init_items_held = starting_items.len() as u64;

            Monkey::new(starting_items, op, op_num, divisible_by, true_monkey, false_monkey, init_items_held)

        }).collect();

    // least common multiple
    let lcm: u64 = monkeys
        .iter()
        .map(|m| m.test_divisible_by)
        .product();

    let monkeys_count = monkeys.len();

    for round in 0..10000 {

        println!("Round: {}", round + 1);

        for idx in 0..monkeys_count {

            let m = &mut monkeys[idx];

            // println!("  Monkey {idx}");

            let mut moves: Vec<(usize,u64)> = Vec::new();

            while let Some(item_worry_level) = m.pop_front() {

                let n = m.op_num.unwrap_or(item_worry_level);

                let new_worry_level = (m.op)(item_worry_level, n) % lcm;

                let is_divisible_by = new_worry_level.rem_euclid(m.test_divisible_by) == 0;

                let to_monkey = if is_divisible_by { m.test_true_monkey } else { m.test_false_monkey };

                moves.push((to_monkey,new_worry_level));

                // println!("    Old wl: {item_worry_level}, New wl: {new_worry_level}, Div 3: {div_by_3}, Is Div by {}: {is_divisible_by}, Throw to: {to_monkey}", m.test_divisible_by);
            }

            for m in moves {
                let monkey = &mut monkeys[m.0];

                monkey.push_back(m.1);
            }

            // println!();
        }

        for (idx, monkey) in monkeys.iter().enumerate() {

            print!("  Monkey {idx}: ");
            for wl in &monkey.item_worry_levels {
                print!("{wl},");
            }
            println!();
        }

        println!();
    }

    let mut levels: Vec<u64> = monkeys.iter()
        .map(|m|{
            // subtract the current levels since they haven't been inspected
            m.items_held - (m.item_worry_levels.len() as u64)
        })
        .collect();

    levels.sort_by(|a, b| b.cmp(a));

    let top_1 = levels[0];
    let top_2 = levels[1];
    let monkey_business_level = top_1 * top_2;

    println!("Monkey business level: {monkey_business_level}, 1: {}, 2: {}", top_1, top_2);
}

fn add_op(old: u64, num: u64) -> u64 {
    old + num
}

fn mult_op(old: u64, num: u64) -> u64 {
    old * num
}

// Round 1
// Monkey 0: 20, 23, 27, 26
// Monkey 1: 2080, 25, 167, 207, 401, 1046
// Monkey 2:
// Monkey 3:

// After round 2, the monkeys are holding items with these worry levels:
// Monkey 0: 695, 10, 71, 135, 350
// Monkey 1: 43, 49, 58, 55, 362
// Monkey 2:
// Monkey 3:

// After round 20, the monkeys are holding items with these worry levels:
// Monkey 0: 10, 12, 14, 26, 34
// Monkey 1: 245, 93, 53, 199, 115
// Monkey 2:
// Monkey 3: