use std::collections::HashMap;
use std::include_str;
use std::process;

const THRESHOLD: usize = 100_000;
const DISK_SIZE: usize = 70_000_000;
const NEED_EMPTY_SIZE: usize = 30_000_000;

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    size: usize,
}

impl Directory {
    fn new(name: String) -> Self {
        Self { name, size: 0 }
    }
}
fn vec_to_path_name(current_name: &Vec<String>) -> String {
    match current_name.len() {
        0 => "/".to_owned(),
        _ => current_name.join("/").to_owned(),
    }
}
fn parse_command(
    mut current_name: Vec<String>,
    mut file_system: HashMap<String, Directory>,
    line: &str,
) -> (Vec<String>, HashMap<String, Directory>) {
    match &line[..4] {
        "$ cd" => {
            match line {
                "$ cd .." => {
                    current_name.pop();
                }
                _ => {
                    let mut new_dir_name =
                        line.split(" ").last().unwrap().to_owned().replace("/", "");
                    current_name.push(new_dir_name);
                    let path_name = vec_to_path_name(&current_name);
                    file_system.insert(path_name.clone(), Directory::new(path_name));
                }
            };
        }
        "$ ls" => {}
        _ => {
            let (first, _) = line.split_once(" ").unwrap();
            match first {
                "dir" => {}
                _ => {
                    let path_name = vec_to_path_name(&current_name);
                    let mut dir = file_system.get_mut(&path_name).unwrap();
                    dir.size += first.parse::<usize>().unwrap();
                }
            }
        }
    };
    (current_name, file_system)
}
fn get_children_directories() -> HashMap<String, usize> {
    let mut current_name: Vec<String> = vec![];
    let mut file_system: HashMap<String, Directory> = HashMap::new();
    for line in include_str!("input").lines() {
        (current_name, file_system) = parse_command(current_name, file_system, line)
    }
    let mut sum_per_directory: HashMap<String, usize> = HashMap::new();
    for dir in file_system.keys() {
        for (key, value) in file_system.iter() {
            if key.contains(dir) || dir == "" {
                *sum_per_directory.entry(dir.to_string()).or_insert(0) += value.size;
            }
        }
    }
    sum_per_directory
}
fn part_1() {
    let sum_per_directory = get_children_directories();
    let total: usize = sum_per_directory
        .values()
        .filter(|x| **x <= THRESHOLD)
        .sum();
    println!(
        "The total size of directories under {} is {}.",
        THRESHOLD, total
    );
}

fn part_2() {
    let sum_per_directory = get_children_directories();
    let need_to_delete = NEED_EMPTY_SIZE - (DISK_SIZE - sum_per_directory.get("").unwrap());
    let smallest_dir: usize = *sum_per_directory
        .values()
        .filter(|x| **x >= need_to_delete)
        .min()
        .unwrap();
    println!(
        "The smallest directory we can delete is {} (bytes?) big.",
        smallest_dir
    );
}
fn main() {
    part_1();
    part_2();
}
