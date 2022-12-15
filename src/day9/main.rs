use std::collections::HashSet;
use std::include_str;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}
impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    fn get_distance_with_other(&self, other: &Self) -> (isize, isize) {
        (self.x - other.x, self.y - other.y)
    }
    fn update_position(&mut self, adjustment: (isize, isize)) {
        self.x += adjustment.0;
        self.y += adjustment.1;
    }
}
struct Rope {
    nodes: Vec<Position>,
    history: Vec<Position>,
}
impl Rope {
    fn new(nodes: usize) -> Self {
        Self {
            nodes: vec![Position::new(); nodes],
            history: vec![Position::new()],
        }
    }
    fn get_node(&mut self, idx: usize) -> Position {
        self.nodes.iter().nth(idx).unwrap().clone()
    }
    fn get_node_mut_ref(&mut self, idx: usize) -> &mut Position {
        self.nodes.iter_mut().nth(idx).unwrap()
    }
    fn update_tail_position(&mut self, idx: usize, update_history: bool) -> bool {
        let head = self.get_node(idx - 1);
        let mut tail = self.get_node(idx);
        let (new_x, new_y): (isize, isize) = head.get_distance_with_other(&tail);
        if [-2, 2].contains(&new_x) {
            let x = if new_x == 2 { 1 } else { -1 };
            let y = if new_y == 2 {
                1
            } else if new_y == -2 {
                -1
            } else {
                new_y
            };
            tail.update_position((x, y));
            _ = std::mem::replace(&mut self.nodes[idx], tail);
            if update_history {
                self.history.push(tail.clone());
            }
            return true;
        }
        if [-2, 2].contains(&new_y) {
            let y = if new_y == 2 { 1 } else { -1 };
            let x = if new_x == 2 {
                1
            } else if new_x == -2 {
                -1
            } else {
                new_x
            };
            tail.update_position((x, y));
            _ = std::mem::replace(&mut self.nodes[idx], tail);
            if update_history {
                self.history.push(tail.clone())
            }
            return true;
        }
        false
    }
    fn update_tail_positions(&mut self) {
        for idx in (1..self.nodes.len()) {
            let updated: bool = self.update_tail_position(idx, idx + 1 == self.nodes.len());
            if !updated {
                break;
            }
        }
    }
    fn parse_instruction(&mut self, instruction: &str) {
        let (direction, size_string) = instruction.split_once(" ").unwrap();
        let size = size_string.parse::<usize>().unwrap();
        for _ in 0..size {
            match direction {
                "U" => {
                    self.get_node_mut_ref(0).y += 1;
                }
                "R" => {
                    self.get_node_mut_ref(0).x += 1;
                }
                "D" => {
                    self.get_node_mut_ref(0).y -= 1;
                }
                "L" => {
                    self.get_node_mut_ref(0).x -= 1;
                }
                _ => todo!(),
            }
            self.update_tail_positions();
        }
    }
}

fn part_1() {
    let mut rope = Rope::new(2);
    for line in include_str!("input").lines() {
        rope.parse_instruction(line);
    }
    println!(
        "The tail of the rope visits {} unique locations",
        rope.history.into_iter().collect::<HashSet<_>>().len()
    );
}
fn part_2() {
    let mut rope = Rope::new(10);
    for line in include_str!("input").lines() {
        rope.parse_instruction(line);
    }
    println!(
        "The tail of the rope visits {} unique locations",
        rope.history.into_iter().collect::<HashSet<_>>().len()
    );
}
fn main() {
    part_1();
    part_2();
}
