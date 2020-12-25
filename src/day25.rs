use itertools::Itertools;
use modpow::modpow;
use num::ToPrimitive;

const M: i32 = 20201227;

pub fn part1(inp: String) {
    println!("{}", parse_input_and_find_encryption_key(inp));
}

fn parse_input_and_find_encryption_key(inp: String) -> i32 {
    let (public_key1, public_key2) = parse_input(inp);
    find_encryption_key(public_key1, public_key2)
}

fn parse_input(inp: String) -> (i32, i32) {
    inp.lines().map(|l| l.parse().unwrap()).next_tuple().unwrap()
}

fn find_encryption_key(public_key1: i32, public_key2: i32) -> i32 {
    let mut current_loop_size = 1;
    let mut current_value = 7;
    loop {
        if current_value == public_key1 {
            return modpow(&public_key2, &current_loop_size, &M).to_i32().unwrap();
        }
        if current_value == public_key2 {
            return modpow(&public_key1, &current_loop_size, &M).to_i32().unwrap();
        }
        current_loop_size += 1;
        current_value = (current_value * 7) % M;
    }
}

#[test]
fn test_parse_input_and_find_encryption_key() {
    assert_eq!(14897079, parse_input_and_find_encryption_key(TEST_INPUT.to_string()));
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"5764801
17807724"#;