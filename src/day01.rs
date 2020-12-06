pub fn part1(inp: String) {
    let numbers = transform_input(inp);
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!("{}", numbers[i] * numbers[j]);
                return;
            }
        }
    }
}

pub fn part2(inp: String) {
    let numbers = transform_input(inp);
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            for k in 0..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("{}", numbers[i] * numbers[j] * numbers[k]);
                    return;
                }
            }
        }
    }
}

fn transform_input(inp: String) -> Vec<i32> {
    return inp
        .lines()
        .map(|line: &str| line.parse().unwrap_or(0))
        .collect();
}
