use numtoa::NumToA;
use regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;

lazy_static! {
    static ref MASK_REGEX: Regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    static ref MEM_REGEX: Regex = Regex::new(r"^mem\[([\d]+)\] = (\d+)$").unwrap();
    
}

struct State {
    mask: [u8; 36],
    memory: HashMap<i64, i64>,
}

pub fn part1(inp: String) {
    let value = iterate_lines_and_apply_operations(
        inp,
        |state, mask| state.mask = mask,
        |state, index, value| { state.memory.insert(index, apply_mask_on_value(value, state.mask)); },
    );
    println!("{}", value);
}

pub fn part2(inp: String) {
    let value = iterate_lines_and_apply_operations(
        inp,
        |state, mask| state.mask = mask,
        |state, index, value| {
            let indices = apply_mask_on_index(index, state.mask);
            for index in indices {
                state.memory.insert(index, value);
            }
        },
    );
    println!("{}", value);
}

fn iterate_lines_and_apply_operations(inp: String, mask_fn: fn(&mut State, [u8; 36]), mem_fn: fn(&mut State, i64, i64)) -> i64 {
    let mut state = State {
        mask: [0u8; 36],
        memory: HashMap::new()
    };
    for line in inp.lines() {
        if MASK_REGEX.is_match(line) {
            let captures = MASK_REGEX.captures(line).unwrap();
            let mask_str = captures.get(1).unwrap().as_str();
            let mask = parse_mask(mask_str);
            mask_fn(&mut state, mask);
        } else if MEM_REGEX.is_match(line) {
            let captures = MEM_REGEX.captures(line).unwrap();
            let index = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let value = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            mem_fn(&mut state, index, value);
        } else {
            panic!("NO mask and mem regexps matched {}", line)
        }
    }
    state.memory.values().cloned().sum::<i64>()
}

fn parse_mask(mask_str: &str) -> [u8; 36] {
    let mut mask = [0u8; 36];
    let mask_slice = mask_str.chars().map(|c| match c{
        'X' => 2u8,
        '0' => 0u8,
        '1' => 1u8,
        _ => panic!("Unknown character {}", c),
    }).collect::<Vec<u8>>();
    mask.copy_from_slice(&mask_slice[0..36]);
    return mask
}

fn apply_mask_on_value(number: i64, mask: [u8; 36]) -> i64 {
    let mut number_a = transform_number_to_bit_array(number);
    for (i, &bit) in mask.iter().enumerate() {
        if bit == 0 {
            number_a[i] = 0;
        }
        if bit == 1 {
            number_a[i] = 1;
        }
    }
    transform_bit_array_to_number(number_a)
}

fn apply_mask_on_index(index: i64, mask: [u8; 36]) -> Vec<i64> {
    let mut bit_arrays: Vec<[u8; 36]> = vec![transform_number_to_bit_array(index as i64)];
    for (i, &bit) in mask.iter().enumerate() {
        if bit == 1 {
            for idx in 0..bit_arrays.len() {
                let mut bit_array_copy = bit_arrays[idx].clone();
                bit_array_copy[i] = 1;
                bit_arrays.remove(idx);
                bit_arrays.insert(idx, bit_array_copy);
            }
        }
        if bit == 2 {
            for idx in 0..bit_arrays.len() {
                let mut bit_array_copy = bit_arrays[idx].clone();
                bit_array_copy[i] = 0;
                bit_arrays.remove(idx);
                bit_arrays.insert(idx, bit_array_copy);
                let mut bit_array_copy1 = bit_arrays[idx].clone();
                bit_array_copy1[i] = 1;
                bit_arrays.push(bit_array_copy1);
            }
        }
    }
    bit_arrays.iter().cloned().map(transform_bit_array_to_number).collect()
}

fn transform_number_to_bit_array(number: i64) -> [u8; 36] {
    let mut bit_array = [0u8; 36];
    number.numtoa(2, &mut bit_array);
    for i in 0..36 {
        if bit_array[i] > 0 {
            bit_array[i] = bit_array[i] - 48
        }
    }
    bit_array
}

fn transform_bit_array_to_number(bit_array: [u8; 36]) -> i64 {
    let number_string = bit_array.iter().map(|&x| if x == 0 {"0"} else {"1"}).intersperse("").collect::<String>();
    let number_str: &str = number_string.as_str();
    i64::from_str_radix(number_str, 2).unwrap()
}

#[test]
fn test_apply_mask_on_value() {
    assert_eq!(73, apply_mask_on_value(11, parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")));
    assert_eq!(101, apply_mask_on_value(101, parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")));
    assert_eq!(64, apply_mask_on_value(0, parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")));
    assert_eq!(65082372640, apply_mask_on_value(122879146, parse_mask("11110X1XXX11001X01X00011001X00X00000")));
    assert_eq!(0, apply_mask_on_value(1023, parse_mask("000000000000000000000000000000000000")));
    assert_eq!(68719476735, apply_mask_on_value(68719476735, parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")));
    assert_eq!(68719476735, apply_mask_on_value(19839428832, parse_mask("111111111111111111111111111111111111")));
    assert_eq!(68719476735, apply_mask_on_value(0, parse_mask("111111111111111111111111111111111111")));
}

#[test]
fn test_apply_mask_on_index() {
    assert_eq!(vec![0, 1], apply_mask_on_index(0, parse_mask("00000000000000000000000000000000000X")));
    assert_eq!(vec![26, 58, 27, 59], apply_mask_on_index(42, parse_mask("000000000000000000000000000000X1001X")));
}