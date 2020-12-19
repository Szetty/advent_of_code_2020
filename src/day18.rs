use std::ops::{Add, Mul};
use std::str::Chars;

pub fn part1(inp: String) {
    println!(
        "{}",
        inp.lines().map(parse_line_and_compute_result1).sum::<i64>()
    )
}

pub fn part2(inp: String) {
    println!(
        "{}",
        inp.lines().map(parse_line_and_compute_result2).sum::<i64>()
    )
}

fn parse_line_and_compute_result1(inp: &str) -> i64 {
    compute_result1(&mut inp.replace(" ", "").chars())
}

fn compute_result1(chars: &mut Chars) -> i64 {
    let mut result = 0;
    let mut operation: Option<fn(i64, i64) -> i64> = None;
    loop {
        match chars.next() {
            Some('(') => result = apply_operation(operation, result, compute_result1(chars)),
            Some(')') => return result,
            Some('+') => operation = Some(Add::add),
            Some('*') => operation = Some(Mul::mul),
            None => return result,
            Some(digit_char) => {
                let digit = digit_char.to_digit(10).unwrap() as i64;
                result = apply_operation(operation, result, digit)
            }
        }
    }
}

fn parse_line_and_compute_result2(inp: &str) -> i64 {
    compute_result2(&mut inp.replace(" ", "").chars(), 0)
}

fn compute_result2(chars: &mut Chars, level: usize) -> i64 {
    let mut result = 0;
    let mut operation: Option<fn(i64, i64) -> i64> = None;
    loop {
        match chars.next() {
            Some('(') => {
                result = apply_operation(operation, result, compute_result2(chars, level + 1))
            }
            Some(')') => return result,
            Some('+') => operation = Some(Add::add),
            Some('*') => {
                if level > 0 {
                    return apply_operation(
                        Some(Mul::mul),
                        result,
                        compute_result2(chars, level + 1),
                    );
                } else {
                    result =
                        apply_operation(Some(Mul::mul), result, compute_result2(chars, level + 1))
                }
            }
            None => return result,
            Some(digit_char) => {
                let digit = digit_char.to_digit(10).unwrap() as i64;
                result = apply_operation(operation, result, digit)
            }
        }
    }
}

fn apply_operation(operation: Option<fn(i64, i64) -> i64>, operand1: i64, operand2: i64) -> i64 {
    match operation {
        Some(op) => op(operand1, operand2),
        None => operand2,
    }
}

#[test]
fn test_parse_line_and_compute_result1() {
    assert_eq!(26, parse_line_and_compute_result1("2 * 3 + (4 * 5)"));
    assert_eq!(
        437,
        parse_line_and_compute_result1("5 + (8 * 3 + 9 + 3 * 4 * 3)")
    );
    assert_eq!(
        12240,
        parse_line_and_compute_result1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
    );
    assert_eq!(
        13632,
        parse_line_and_compute_result1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
    );
}

#[test]
fn test_parse_line_and_compute_result2() {
    assert_eq!(
        51,
        parse_line_and_compute_result2("1 + (2 * 3) + (4 * (5 + 6))")
    );
    assert_eq!(46, parse_line_and_compute_result2("2 * 3 + (4 * 5)"));
    assert_eq!(
        1445,
        parse_line_and_compute_result2("5 + (8 * 3 + 9 + 3 * 4 * 3)")
    );
    assert_eq!(
        669060,
        parse_line_and_compute_result2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
    );
    assert_eq!(
        23340,
        parse_line_and_compute_result2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
    );
}
