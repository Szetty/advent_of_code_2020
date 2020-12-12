use std::ops::Range;

pub fn part1(inp: String) {
    let numbers: Vec<i64> = inp.lines().map(|l| l.parse::<i64>().unwrap()).collect();

    let (_, number) = find_number_not_sum_of_two_from_25_numbers_before(numbers);
    println!("{}", number);
}

pub fn part2(inp: String) {
    let numbers: Vec<i64> = inp.lines().map(|l| l.parse::<i64>().unwrap()).collect();
    let (max_idx, number) = find_number_not_sum_of_two_from_25_numbers_before(numbers.clone());
    let (numbers_before, _) = numbers.split_at(max_idx);
    match find_contiguous_list_that_sums_up_to(
        numbers_before.iter().cloned().collect::<Vec<i64>>(),
        number,
    ) {
        Some(sum_list) => {
            let min = sum_list.iter().min().unwrap();
            let max = sum_list.iter().max().unwrap();
            println!("{}", min + max);
        }
        None => {}
    }
}

fn find_number_not_sum_of_two_from_25_numbers_before(numbers: Vec<i64>) -> (usize, i64) {
    for i in 25..numbers.len() {
        if !search_sum_with_index(numbers.clone(), numbers[i], (i - 25)..i) {
            return (i, numbers[i]);
        }
    }
    return (0, 0);
}

fn search_sum_with_index(numbers: Vec<i64>, sum: i64, interval: Range<usize>) -> bool {
    let end = interval.end;
    for i in interval {
        for j in (i + 1)..end {
            if numbers[i] + numbers[j] == sum {
                return true;
            }
        }
    }
    return false;
}

fn find_contiguous_list_that_sums_up_to(numbers: Vec<i64>, sum: i64) -> Option<Vec<i64>> {
    let mut sum_list_size = 2;
    while sum_list_size < numbers.len() {
        // println!("{}", sum_list_size);
        match find_sum_list_with_sum_equal_to(numbers.clone(), sum_list_size, sum) {
            Some(sum_list) => return Some(sum_list),
            None => {}
        }
        sum_list_size = sum_list_size + 1;
    }
    return None;
}

fn find_sum_list_with_sum_equal_to(
    numbers: Vec<i64>,
    sum_list_size: usize,
    sum: i64,
) -> Option<Vec<i64>> {
    for i in sum_list_size..(numbers.len() - sum_list_size) {
        let mut current_sum = 0;
        for j in i..(i + sum_list_size) {
            current_sum = current_sum + numbers[j];
        }
        if current_sum == sum {
            let (_, n) = numbers.split_at(i);
            let (r, _) = n.split_at(sum_list_size);
            return Some(r.iter().cloned().collect::<Vec<i64>>());
        }
    }
    return None;
}
