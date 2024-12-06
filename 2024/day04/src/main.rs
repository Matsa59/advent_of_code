use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = read_file_inputs("./inputs/day04.txt");
    println!("Part one: {}", get_result_part_one(&input));
    println!("Part two: {}", get_result_part_two(&input));
}

fn get_result_part_two(input: &str) -> u32 {
    let mut result: u32 = 0;
    let line_count: usize = input.lines().count();
    let input = input.replace("\n", "");
    let line_len = input.len() / line_count;
    let input = input.as_bytes();

    for row in 1..line_count - 1 {
        for col in 1..line_len - 1 {
            let char = input[col + row * line_len];
            if char != b'A' { continue; }

            let top_left = input[(col - 1) + (row - 1) * line_len];
            let bottom_right = input[(col + 1) + (row + 1) * line_len];
            if !((top_left == b'M' && bottom_right == b'S') || (top_left == b'S' && bottom_right == b'M')) {
                continue;
            }

            let top_right = input[(col + 1) + (row - 1) * line_len];
            let bottom_left = input[(col - 1) + (row + 1) * line_len];

            if !((top_right == b'M' && bottom_left == b'S') || (top_right == b'S' && bottom_left == b'M')) {
                continue;
            }

            result += 1;
        }
    }

    result
}

fn get_result_part_one(input: &str) -> u32 {
    let line_count: usize = input.lines().count();
    let line_len: usize = input.lines().into_iter().next().unwrap().len();
    let input = input.replace("\n", "");
    let input_bytes = input.as_bytes();

    let mut counter: u32 = 0;

    for row in 0..line_count {
        for col in 0..line_len {
            if check_horizontal(&input_bytes, line_len, col, row) {
                counter += 1;
            }

            if check_vertical(&input_bytes, line_len, col, row) {
                counter += 1;
            }

            if check_diag_right(&input_bytes, line_len, col, row) {
                counter += 1;
            }

            if check_diag_left(&input_bytes, line_len, col, row) {
                counter += 1;
            }
        }
    }

    counter
}

fn check_horizontal(input: &[u8], line_len: usize, col: usize, row: usize) -> bool {
    if col + 4 > line_len { return false }

    let start_idx = row * line_len + col;
    let letters = &input[start_idx..start_idx + 4];
    check_letters(&letters)
}

fn check_vertical(input: &[u8], line_len: usize, col: usize, row: usize) -> bool {
    if (row + 3) * line_len + col >= input.len() { return false; }

    let mut letters: Vec<u8> = Vec::new();

    for i in 0..4 {
        let char_index = (row + i) * line_len + col;
        letters.push(input[char_index]);
    }

    check_letters(&letters)
}

fn check_diag_right(input: &[u8], line_len: usize, col: usize, row: usize) -> bool {
    if col + 4 > line_len { return false; }
    if (row + 4) * line_len > input.len() { return false; }

    let mut letters: Vec<u8> = Vec::new();

    for i in 0..4 {
        let char_index = (i + row) * line_len + col + i;
        letters.push(input[char_index]);
    }

    check_letters(&letters)
}

fn check_diag_left(input: &[u8], line_len: usize, col: usize, row: usize) -> bool {
    if col < 3 { return false; }
    if (row + 4) * line_len > input.len() { return false; }

    let mut letters: Vec<u8> = Vec::new();

    for i in 0..4 {
        let char_index = (i + row) * line_len + col - i;
        letters.push(input[char_index]);
    }

    check_letters(&letters)
}

fn check_letters(letters: &[u8]) -> bool {
    letters == b"XMAS" || letters == b"SAMX"
}

fn read_file_inputs(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut inputs = String::new();
    file.read_to_string(&mut inputs).unwrap().to_string();

    inputs
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let input ="MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(crate::get_result_part_one(input), 18);
        assert_eq!(crate::get_result_part_two(input), 9);
    }
}

// 2517
