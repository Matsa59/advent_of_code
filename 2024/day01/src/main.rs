use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn part_one() -> u32 {
    let (mut left, mut right) = get_inputs();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part_two() -> u32 {
    let (left, right) = get_inputs();

    let mut id_values = HashMap::<u32, u32>::new();

    for id in right {
        id_values.entry(id).and_modify(|x| *x += 1).or_insert(1);
    }

    left.iter()
        .map(|x| x * id_values.get(x).unwrap_or(&0))
        .sum()
}

fn get_inputs() -> (Vec<u32>, Vec<u32>) {
    let inputs = read_file_inputs("./inputs/day01.txt");
    let mut left = Vec::with_capacity(inputs.lines().count());
    let mut right = Vec::with_capacity(inputs.lines().count());

    for line in inputs.lines() {
        let numbers : Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        left.push(numbers[0]);
        right.push(numbers[1])
    }

    (left, right)
}

fn read_file_inputs(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut inputs = String::new();
    file.read_to_string(&mut inputs).unwrap().to_string();

    inputs
}
