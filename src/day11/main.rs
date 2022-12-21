use std::include_str;

const DENUMERATORS: usize = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;

enum Operation {
    Add(usize),
    Mult(usize),
    Square,
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisible: usize,
    good_monkey: usize,
    bad_monkey: usize,
    items_inspected: usize,
}
impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: Operation,
        divisible: usize,
        good_monkey: usize,
        bad_monkey: usize,
    ) -> Self {
        Self {
            items,
            operation,
            divisible,
            good_monkey,
            bad_monkey,
            items_inspected: 0,
        }
    }
}
impl Monkey {
    fn increase_worry(&self, worry_level: usize) -> usize {
        match self.operation {
            Operation::Add(n) => worry_level + n,
            Operation::Mult(n) => worry_level * n,
            Operation::Square => worry_level * worry_level,
        }
    }
    fn inspect_and_decrease(&mut self) -> Vec<(usize, usize)> {
        let mut other_monkeys: Vec<(usize, usize)> = vec![];
        while self.items.len() > 0 {
            let item = self.items.pop().unwrap();
            self.items_inspected += 1;
            let worry_level = self.increase_worry(item) / 3;
            if worry_level % self.divisible == 0 {
                other_monkeys.push((self.good_monkey, worry_level));
            } else {
                other_monkeys.push((self.bad_monkey, worry_level));
            }
        }
        other_monkeys
    }
    fn inspect(&mut self) -> Vec<(usize, usize)> {
        let mut other_monkeys: Vec<(usize, usize)> = vec![];
        while self.items.len() > 0 {
            let mut item = self.items.pop().unwrap();
            self.items_inspected += 1;
            item = item.min(DENUMERATORS + item % DENUMERATORS);
            let worry_level = self.increase_worry(item);
            if worry_level % self.divisible == 0 {
                other_monkeys.push((self.good_monkey, worry_level));
            } else {
                other_monkeys.push((self.bad_monkey, worry_level));
            }
        }
        other_monkeys
    }
}
fn parse_monkey_base(monkey_chunk: &str) -> Monkey {
    let chunks: Vec<&str> = monkey_chunk.split("\n").collect();
    let starting_items: Vec<_> = chunks[1][18..]
        .replace(" ", "")
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let k: Vec<_> = chunks[2][19..].splitn(3, " ").collect();
    let (first, op, second) = (k[0], k[1], k[2]);

    let operation = match op {
        "+" => Operation::Add(second.parse::<usize>().unwrap()),
        "*" => {
            if first == second {
                Operation::Square
            } else {
                Operation::Mult(second.parse::<usize>().unwrap())
            }
        }
        _ => panic!(),
    };
    let divisible_by = chunks[3][21..].parse::<usize>().unwrap();
    let good_monkey = chunks[4][29..].parse::<usize>().unwrap();
    let bad_monkey = chunks[5][30..].parse::<usize>().unwrap();
    Monkey::new(
        starting_items,
        operation,
        divisible_by,
        good_monkey,
        bad_monkey,
    )
}
fn part_1() {
    let input = include_str!("input").split("\n\n").collect::<Vec<_>>();
    let mut monkeys: Vec<Monkey> = input.iter().map(|x| parse_monkey_base(x)).collect();

    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            let throws = monkeys[idx].inspect_and_decrease();
            for (other_monkey, worry_level) in throws {
                monkeys[other_monkey].items.push(worry_level);
            }
        }
    }
    let mut monkey_business: Vec<_> = monkeys.iter().map(|x| x.items_inspected).collect();
    monkey_business.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!(
        "The level of monkey business after 20 rounds is: {}.",
        monkey_business[0] * monkey_business[1]
    );
}
fn part_2() {
    let input = include_str!("input").split("\n\n").collect::<Vec<_>>();
    let mut monkeys: Vec<Monkey> = input.iter().map(|x| parse_monkey_base(x)).collect();

    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            let throws = monkeys[idx].inspect();
            for (other_monkey, worry_level) in throws {
                monkeys[other_monkey].items.push(worry_level);
            }
        }
    }
    let mut monkey_business: Vec<_> = monkeys.iter().map(|x| x.items_inspected).collect();
    monkey_business.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!(
        "The level of monkey business after 10000 rounds is: {}.",
        monkey_business[0] * monkey_business[1]
    );
}
fn main() {
    part_1();
    part_2();
}
