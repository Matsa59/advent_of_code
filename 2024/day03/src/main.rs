use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn get_result(input: &str) -> i32 {
  let regex = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
  let mut result = 0;

  for (_, [num_a, num_b]) in regex.captures_iter(input).map(|c| c.extract()) {
      let num_a = num_a.parse::<i32>().unwrap();
      let num_b = num_b.parse::<i32>().unwrap();

      result += num_a * num_b
  }

  result
}

fn remove_invalids_mul(input: &str) -> String {
    let mut result = String::new();
    let mut ignore = false;
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        if i + 7 < chars.len() && &chars[i..i+7] == &['d', 'o', 'n', '\'', 't', '(', ')'] {
            ignore = true;
            i += 7;
        } else if i + 4 < chars.len() && &chars[i..i+4] == &['d', 'o', '(', ')'] {
            ignore = false;
            i += 4;
        } else if !ignore {
            result.push(chars[i]);
            i += 1;
        } else {
            i += 1;
        }
    }

    result
}

fn part_one() -> i32 {
    let inputs = read_file_inputs("./inputs/day03.txt");
    get_result(&inputs)
}

fn part_two() -> i32 {
    let inputs = read_file_inputs("./inputs/day03.txt");
    let inputs = remove_invalids_mul(&inputs);
    get_result(&inputs)
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
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
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(crate::get_result(input), 161);
    }

    #[test]
    fn part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let input = crate::remove_invalids_mul(input);
        assert_eq!(crate::get_result(&input), 48);
    }

}
