use std::include_str;

#[derive(Debug)]
enum Value {
    List(Box<Signal>),
    Integer(usize),
}
impl Value {
    fn get_value(self) -> Vec<Option<usize>> {
        let mut values: Vec<Option<usize>> = vec![];
        match self {
            Value::Integer(x) => values.push(Some(x)),
            Value::List(b) => values.extend(
                b.values
                    .into_iter()
                    .map(|x| x.get_value())
                    .flatten()
                    .collect::<Vec<_>>(),
            ),
        };
        values
    }
}
#[derive(Debug)]
struct Signal {
    values: Vec<Value>,
}
impl Signal {
    fn new(values: Vec<Value>) -> Self {
        Self { values }
    }
    fn is_right_order(&self, other: &Signal) -> bool {
        for (idx, value) in self.values.iter().enumerate() {
            match value {
                Value::Integer(x) => {}
                Value::List(values) => {}
            }
        }
        true
    }
}
impl Iterator for Signal {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn load_data(chars: &mut Vec<char>) -> Signal {
    let mut values: Vec<Value> = vec![];
    let mut current: Vec<char> = vec![];
    while chars.len() > 0 {
        let c = chars.remove(0);
        if c.is_digit(10) {
            current.push(c);
        } else {
            match c {
                '[' => values.push(Value::List(Box::new(load_data(chars)))),
                ']' => {
                    if current.len() > 0 {
                        let number = current.iter().collect::<String>().parse::<usize>().unwrap();
                        values.push(Value::Integer(number));
                        current.clear();
                    }
                    break;
                }
                ',' => {
                    if current.len() > 0 {
                        let number = current.iter().collect::<String>().parse::<usize>().unwrap();
                        values.push(Value::Integer(number));
                        current.clear();
                    }
                }
                _ => {
                    panic!()
                }
            };
        }
    }
    Signal::new(values)
}
fn part_1() {
    let mut pairs: Vec<(Vec<char>, Vec<char>)> = include_str!("input")
        .split("\n\n")
        .into_iter()
        .map(|z| {
            let (x, y) = z.split_once("\n").unwrap();
            (
                x.chars().collect::<Vec<char>>(),
                y.chars().collect::<Vec<char>>(),
            )
        })
        .collect();
    for (mut a, mut b) in pairs {
        let c = load_data(&mut a);
        let d = load_data(&mut b);
        break;
    }
}
fn part_2() {}
fn main() {
    part_1();
}
