use std::collections::HashMap;

pub fn part1(inp: String) {
    println!("{}", compute_numbers_until(inp, 2020))
}

pub fn part2(inp: String) {
    println!("{}", compute_numbers_until(inp, 30000000))
}

fn compute_numbers_until(starting_numbers_str: String, max_count: usize) -> i32 {
    let numbers: Vec<i32> = starting_numbers_str
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut occurrences: HashMap<i32, usize> = HashMap::new();
    for (i, &number) in numbers.iter().enumerate() {
        occurrences.insert(number, i + 1);
    }
    let mut i = numbers.len() + 1;
    let mut previous_number: i32 = *numbers.last().unwrap();
    let mut current_number: i32 = 0;
    while i < max_count {
        occurrences.insert(previous_number, i - 1);
        previous_number = current_number;
        if occurrences.contains_key(&current_number) {
            current_number = i as i32 - occurrences[&current_number] as i32;
        } else {
            current_number = 0;
        }
        i = i + 1;
    }
    current_number
}

#[test]
fn test_compute_numbers_until() {
    assert_eq!(436, compute_numbers_until("0,3,6".to_string(), 2020));
    assert_eq!(1, compute_numbers_until("1,3,2".to_string(), 2020));
    assert_eq!(10, compute_numbers_until("2,1,3".to_string(), 2020));
    assert_eq!(27, compute_numbers_until("1,2,3".to_string(), 2020));
    assert_eq!(78, compute_numbers_until("2,3,1".to_string(), 2020));
    assert_eq!(438, compute_numbers_until("3,2,1".to_string(), 2020));
    assert_eq!(1836, compute_numbers_until("3,1,2".to_string(), 2020));
}
