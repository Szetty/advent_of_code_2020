pub fn part1(inp: String) {  
    let map: Vec<Vec<char>> = inp.lines().map(|l| l.chars().collect()).collect();
    println!("{}", count_trees(map, (1, 3)));
}

pub fn part2(inp: String) {
    let map: Vec<Vec<char>> = inp.lines().map(|l| l.chars().collect()).collect();
    let steps = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let prod: i64 = steps.iter().map(|step| count_trees(map.clone(), *step)).product();
    println!("{}", prod);
}

fn count_trees(map: Vec<Vec<char>>, step: (usize, usize)) -> i64 {
    let x_len = map.len() as usize;
    let y_len = map[0].len() as usize;
    let mut pos = (0, 0);
    let mut count = 0;
    while pos.0 + step.0 < x_len {
        pos = (pos.0 + step.0, (pos.1 + step.1) % y_len);
        if map[pos.0][pos.1] == '#' {
            count = count + 1;
        }
    }
    count
}