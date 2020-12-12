pub fn part1(inp: String) {
    let numbers = transform_input(inp);
    println!("{}", sorted_search_sum(numbers, 2020));
}

pub fn part2(inp: String) {
    let numbers = transform_input(inp);
    println!("{}", search_sum_of_3(numbers, 2020));
}

fn transform_input(inp: String) -> Vec<i32> {
    return inp
        .lines()
        .map(|line: &str| line.parse().unwrap_or(0))
        .collect();
}

#[allow(dead_code)]
fn search_sum(numbers: Vec<i32>, sum: i32) -> i32 {
    search_sum_with_index(numbers, sum, 0)
}

fn sorted_search_sum(mut numbers: Vec<i32>, sum: i32) -> i32 {
    numbers.sort();
    sorted_search_sum_with_start_index(numbers, sum, 0)
}

fn sorted_search_sum_with_start_index(numbers: Vec<i32>, sum: i32, start_index: usize) -> i32 {
    let mut i1 = (start_index + numbers.len() - 1) / 2;
    let mut i2 = (start_index + numbers.len() - 1) / 2 + 1;
    while i1 >= start_index && i2 < numbers.len() && i1 < i2 {
        if numbers[i1] + numbers[i2] == sum {
            return numbers[i1] * numbers[i2];
        } else if numbers[i1] + numbers[i2] > sum {
            if numbers[i1 - 1] + numbers[i2] < sum {
                i2 = i2 - 1;
            } else {
                i1 = i1 - 1;
            }
        } else {
            if numbers[i1] + numbers[i2 + 1] > sum {
                i1 = i1 + 1;
            } else {
                i2 = i2 + 1;
            }
        }
    }
    return 0;
}

fn search_sum_with_index(numbers: Vec<i32>, sum: i32, start_index: usize) -> i32 {
    for i in start_index..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == sum {
                return numbers[i] * numbers[j];
            }
        }
    }
    return 0;
}

fn search_sum_of_3(numbers: Vec<i32>, sum: i32) -> i32 {
    for i in 0..numbers.len() {
        let product = search_sum_with_index(numbers.clone(), sum - numbers[i], i + 1);
        if product != 0 {
            return numbers[i] * product;
        }
    }
    return 0;
}
