use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Ordering;

fn main() {
    println!("Part one: {}", part_one());
}

fn part_one() -> usize {
    get_input_lines()
        .into_iter()
        .filter(|level| valid_level(level))
        .count()
}

fn valid_level(level: &Vec<u32>) -> bool {
    let mut level = level.iter();
    let first = *level.next().unwrap();

    level.scan((first, None), |(previous, direction), value| {
        let (valid, new_direction) = valid_diff(*previous, *value, *direction);

        if !valid {
            return Some(false);
        } else {
            *previous = *value;
            *direction = new_direction;
            Some(true)
        }
    }).all(|valid| valid)
}

fn valid_diff(num1: u32, num2: u32, direction: Option<Ordering>) -> (bool, Option<Ordering>) {
    if num1.abs_diff(num2) > 3 {
        return (false, None);
    }

    if let Some(direction) = direction {
        (num1.cmp(&num2) == direction, Some(direction))
    } else {
        let direction = num1.cmp(&num2);
        (true, Some(direction))
    }
}

fn get_input_lines() -> Vec<Vec<u32>> {
    let file = File::open("./inputs/day02.txt").unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}
