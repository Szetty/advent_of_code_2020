use std::collections::HashSet;

pub fn part1(inp: String) {
    let ops = parse_operations(inp);
    let (halted, acc) = execute_operations(ops);
    assert_eq!(false, halted);
    println!("{:?}", acc);
}

pub fn part2(inp: String) {
    let ops = parse_operations(inp);
    println!("{:?}", replace_corrupted_op_and_execute_operations(ops));
}

fn parse_operations(inp: String) -> Vec<(String, i32)> {
    inp.lines().map(|s| {
        match s.split(" ").collect::<Vec<&str>>()[..] {
            [op, param] => {
                (op.to_string(), param.parse().unwrap())
            },
            _ => panic!("Unexpected operation @ parse")
        }
    }).collect()
}

fn execute_operations(ops: Vec<(String, i32)>) -> (bool, i32) {
    let mut acc = 0;
    let mut op_counter = 0;
    let mut already_executed_ops = HashSet::<i32>::new();
    let mut halted = false;
    loop {
        if op_counter as usize >= ops.len() {
            halted = true;
            break
        }
        if already_executed_ops.contains(&op_counter) {
            break
        }
        already_executed_ops.insert(op_counter);
        let (op, param) = &ops[op_counter as usize];
        match op.as_str() {
            "acc" => { acc = acc + param; op_counter = op_counter + 1 },
            "jmp" => op_counter = op_counter + param,
            "nop" => op_counter = op_counter + 1,
            _ => panic!("Unexpected operation @ execute")
        }
    }
    return (halted, acc);
}

fn replace_corrupted_op_and_execute_operations(ops: Vec<(String, i32)>) -> i32 {
    for (i, (op, param)) in ops.iter().enumerate() {
        let replacement;
        match op.as_str() {
            "nop" => replacement = "jmp",
            "jmp" => replacement = "nop",
            _ => continue
        }
        let mut new_ops = ops.clone();
        new_ops[i] = (replacement.to_string(), *param);
        let (halted, acc) = execute_operations(new_ops);
        if halted {
            return acc
        }
    }
    return -1
}