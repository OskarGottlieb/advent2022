use std::collections::HashSet;
use std::hash::Hash;
use std::include_str;

const LETTERS: &'static str = "SabcdefghijklmnopqrstuvwxyzE";
const WIDTH: usize = 64;
const HEIGHT: usize = 41;
const TOTAL_LENGTH: usize = HEIGHT * WIDTH;
const START: usize = 20 * 64;
const END: usize = START + 40;

fn get_neighbors(all_coordinates: &Vec<usize>, coordinate_start: usize) -> Vec<usize> {
    let mut coordinates: Vec<usize> = vec![];
    let down = coordinate_start + WIDTH;
    let right = coordinate_start + 1;

    let value_old = all_coordinates[coordinate_start] as isize;

    if coordinate_start >= WIDTH {
        let up = coordinate_start - WIDTH;
        let value_new = all_coordinates[up];
        if (value_new as isize - value_old) <= 1 {
            coordinates.push(up);
        }
    }
    if down < TOTAL_LENGTH {
        let value_new = all_coordinates[down];
        if (value_new as isize - value_old) <= 1 {
            coordinates.push(down);
        }
    }
    if (coordinate_start == 0) || (coordinate_start + 1) % WIDTH != 0 {
        let value_new = all_coordinates[right];
        if (value_new as isize - value_old) <= 1 {
            coordinates.push(right);
        }
    }
    if (coordinate_start > 0) && (coordinate_start) % WIDTH != 0 {
        let left = coordinate_start - 1;
        let value_new = all_coordinates[left];
        if (value_new as isize - value_old) <= 1 {
            coordinates.push(left);
        }
    }
    coordinates
}
fn get_coordinates() -> Vec<usize> {
    include_str!("input")
        .replace("\n", "")
        .chars()
        .map(|x| LETTERS.find(x).unwrap())
        .collect()
}
fn find_shortest_path(coordinates: Vec<usize>, start: usize) -> Option<usize> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut peak_found: bool = false;
    let mut counter = 0;
    let mut current_coordinates: Vec<usize> = vec![start];
    let mut new_coordinates: HashSet<usize> = HashSet::new();

    while !peak_found {
        counter += 1;
        new_coordinates = current_coordinates
            .into_iter()
            .map(|x| get_neighbors(&coordinates, x))
            .flatten()
            .collect();
        new_coordinates.retain(|x| !visited.contains(&x));
        visited.extend(new_coordinates.clone());
        current_coordinates = Vec::from_iter(new_coordinates);
        if current_coordinates.contains(&END) {
            peak_found = true;
        } else if current_coordinates.len() == 0 {
            return None;
        }
    }
    Some(counter)
}
fn part_1() {
    let coordinates = get_coordinates();
    let shortest_path = find_shortest_path(coordinates.clone(), START);
    println!(
        "The shortest path is {} steps long.",
        shortest_path.unwrap()
    )
}
fn part_2() {
    let coordinates = get_coordinates();
    let a: Vec<usize> = include_str!("input")
        .replace("\n", "")
        .chars()
        .enumerate()
        .filter(|(_, x)| x == &'a')
        .map(|(idx, _)| idx)
        .collect();
    let mut shortest_paths: Vec<usize> = a
        .into_iter()
        .filter_map(|x| find_shortest_path(coordinates.clone(), x))
        .collect();
    shortest_paths.sort();
    println!("The shortest path is {:#?} steps long.", shortest_paths[0]);
}

fn main() {
    part_1();
    part_2();
}
