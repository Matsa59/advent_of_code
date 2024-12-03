use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn part_one() -> usize {
    get_input_lines()
        .into_iter()
        .filter(|level| valid_levels(level.clone(), false))
        .count()
}

fn part_two() -> usize {
    get_input_lines()
        .into_iter()
        .filter(|level| valid_levels(level.clone(), true))
        .count()
}

fn valid_levels(levels: Vec<u32>, tolerance: bool) -> bool {
    if tolerance {
        let mut index = 0;
        while index < levels.len() {
            if do_valid_levels(levels.clone(), Some(index)) {
                return true;
            }
            index += 1;
        }
    } else {
        if do_valid_levels(levels.clone(), None) {
            return true;
        }
    }

    false
}

fn do_valid_levels(mut levels: Vec<u32>, exclude: Option<usize>) -> bool {
    if let Some(index) = exclude {
        levels.remove(index);
    }

    if levels.len() < 3 {
        return true;
    }

    let ordering = levels[0].cmp(&levels[1]);
    let mut index = 1;

    while index < levels.len() {
        let val_a = levels[index - 1];
        let val_b = levels[index];

        if val_a.cmp(&val_b) != ordering {
            return false;
        }

        if val_a.abs_diff(val_b) > 3 {
            return false;
        }

        index += 1;
    }

    true
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe() {
        let data = vec![7, 6, 4, 2, 1];
        assert!(valid_levels(data, true));

        let data = vec![1, 3, 2, 4, 5];
        assert!(valid_levels(data, true));

        let data = vec![8, 6, 4, 4, 1];
        assert!(valid_levels(data, true));

        let data = vec![1, 3, 6, 7, 9];
        assert!(valid_levels(data, true));
    }

    #[test]
    fn unsafes() {
        let data = vec![1, 2, 7, 8, 9];
        assert!(!valid_levels(data, true));

        let data = vec![9, 7, 6, 2, 1];
        assert!(!valid_levels(data, true));
    }
}
