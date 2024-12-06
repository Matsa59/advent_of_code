use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let (orders, updates) = get_inputs();

    let part_one: i32 = updates.iter()
        .filter(|update| is_valid_update(update, &orders))
        .filter_map(|update| get_mid_value(update))
        .sum();

    let part_two: i32 = updates.iter()
        .filter(|update| !is_valid_update(update, &orders))
        .map(|x| fix_update(x, &orders))
        .filter_map(|update| get_mid_value(&update))
        .sum();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

}

fn parse_order_input(input: &str) -> HashMap<i32, Vec<i32>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("|").unwrap();
        let a: i32 = a.parse().unwrap();
        let b: i32 = b.parse().unwrap();

        map.entry(a).or_insert_with(Vec::new).push(b);
    }

    map
}

fn parse_updates(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| {
            line.split(",").map(|x| x.parse().unwrap()).collect()
        }).collect()
}

fn is_valid_update(update: &Vec<i32>, orders: &HashMap<i32, Vec<i32>>) -> bool {
    let mut history = HashSet::new();

    for page in update {
        if let Some(dependencies) = orders.get(page) {
            if dependencies.iter().any(|x| history.contains(&x)) {
                return false;
            }
        }

        history.insert(page);
    }

    true
}

fn fix_update(update: &Vec<i32>, orders: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut update = update.clone();
    let mut result: Vec<i32> = Vec::new();
    let mut reversed_orders = HashMap::new();

    for (value, dependencies) in orders {
        for dependency in dependencies {
            reversed_orders.entry(dependency).or_insert(Vec::new()).push(value);
        }
    }

    while !update.is_empty() {
        for i in 0..update.len() {
            let value = update[i];

            if let Some(dependants) = reversed_orders.get(&value) {
                if has_dependants(&dependants, &update) {
                    continue;
                }
            }

            update.remove(i);
            result.push(value);
            break;
        }
    }

    result
}

fn has_dependants(dependants: &Vec<&i32>, values: &Vec<i32>) -> bool {
    for dependency in dependants {
        if values.contains(dependency) {
            return true;
        }
    }

    false
}

fn get_mid_value(update: &Vec<i32>) -> Option<i32> {
    if update.len() % 2 == 0 {
        return None;
    }

    let position = update.len() / 2;

    update.get(position).copied()
}

fn get_inputs() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let input = read_to_string("./inputs/day05.txt").unwrap();
    let (orders, updates) = input
        .split_once("\n\n")
        .expect("Invalid inputs, must be empty lines between order and updates");


    let orders = parse_order_input(&orders);
    let updates = parse_updates(&updates);

    (orders, updates)
}

#[cfg(test)]
mod tests {
    // const INPUT: &str = "75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
    const ORDER_INPUT: &str = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n\
                               97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13";

    use super::*;

    #[test]
    fn parse_order_input_test() {
        let map = parse_order_input("47|53\n97|13\n97|61");
        assert_eq!(map.get(&47), Some(&vec![53]));
        assert_eq!(map.get(&97), Some(&vec![13, 61]));
    }

    #[test]
    fn is_valid_update_test() {
        let orders = parse_order_input(ORDER_INPUT);

        // Valids
        assert!(is_valid_update(&vec![75,47,61,53,29], &orders));
        assert!(is_valid_update(&vec![97,61,53,29,13], &orders));
        assert!(is_valid_update(&vec![75,29,13], &orders));

        // Invalids
        assert!(!is_valid_update(&vec![75,97,47,61,53], &orders));
        assert!(!is_valid_update(&vec![61,13,29], &orders));
        assert!(!is_valid_update(&vec![97,13,75,29,47], &orders));
    }

    #[test]
    fn fix_update_test() {
        let orders = parse_order_input(ORDER_INPUT);

        // assert_eq!(fix_update(&vec![75,97,47,61,53], &orders), vec![97,75,47,61,53]);
        assert_eq!(fix_update(&vec![61,13,29], &orders), vec![61,29,13]);
        // assert_eq!(fix_update(&vec![97,13,75,29,47], &orders), vec![47,75,47,29,13]);
    }

    #[test]
    fn get_mid_value_test() {
        assert_eq!(get_mid_value(&vec![75, 47, 61, 53, 29]), Some(61));
        assert_eq!(get_mid_value(&vec![97, 61, 53, 29, 13]), Some(53));
        assert_eq!(get_mid_value(&vec![75, 29, 13]), Some(29));
    }
}
