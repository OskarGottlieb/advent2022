use std::include_str;

const IMPORTANT_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
const DISPLAY_SIZE: usize = 240;

struct CPU {
    cycle: usize,
    X: isize,
    signals: Vec<isize>,
    display: [char; DISPLAY_SIZE],
}
impl CPU {
    fn new() -> Self {
        Self {
            cycle: 0,
            X: 1,
            signals: vec![],
            display: ['.'; DISPLAY_SIZE],
        }
    }
}

fn process_load(cpu: &mut CPU, loads: Vec<(usize, isize)>) {
    for load in loads {
        let (cycle, value) = load;
        if cycle > 0 && (cpu.cycle as isize % 40 - cpu.X).abs() <= 1 {
            std::mem::replace(&mut cpu.display[cpu.cycle], '#');
        }

        cpu.cycle += cycle;
        if IMPORTANT_CYCLES.contains(&cpu.cycle) {
            cpu.signals.push(cpu.X * cpu.cycle as isize);
        }
        cpu.X += value;
    }
}

enum Instruction {
    Addx(isize),
    Noop,
}
impl Instruction {
    fn from_line(line: &str) -> Self {
        match line.len() {
            4 => Instruction::Noop,
            _ => {
                let (_, value) = line.split_once(" ").unwrap();
                Instruction::Addx(value.parse::<isize>().unwrap())
            }
        }
    }
    fn to_tuples(self) -> Vec<(usize, isize)> {
        match self {
            Instruction::Noop => vec![(1, 0)],
            Instruction::Addx(s) => vec![(1, 0), (1, s)],
        }
    }
}

fn main() {
    let mut cpu = CPU::new();
    for line in include_str!("input").lines() {
        process_load(&mut cpu, Instruction::from_line(line).to_tuples());
    }
    println!(
        "The sum of the signals is: {}. \n",
        cpu.signals.iter().sum::<isize>()
    );
    println!(
        "{}",
        cpu.display
            .chunks(40)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    )
}
