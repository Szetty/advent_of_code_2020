use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use phf::{phf_map, phf_set};
use regex::Regex;
use std::ops::RangeInclusive;

pub fn part1(inp: String) {  
    println!("{}", count_valid_passport(inp, is_valid1));
}

pub fn part2(inp: String) {
    println!("{}", count_valid_passport(inp, is_valid2));
}

fn count_valid_passport(inp: String, is_valid: fn(Vec<&str>) -> bool) -> usize {
    return
        inp
        .split("\n\n")
        .filter(|passport| {
            is_valid(passport.split(['\n', ' ',].as_ref()).collect())
        })
        .count();
}

static REQUIRED_FIELDS: phf::Set<&str> = phf_set! { "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" };

fn is_valid1(passport: Vec<&str>) -> bool {
    let required_fields: HashSet<&str> = HashSet::from_iter(REQUIRED_FIELDS.iter().cloned());
    let passport_fields = passport.iter().map(|entry| entry.split(":").next().unwrap()).collect();
    required_fields.difference(&passport_fields).count() == 0
}

lazy_static!{
    static ref HGT_REGEX: Regex = Regex::new(r"^(\d+)cm|(\d+)in$").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"^\#[0-9a-f]{6}$").unwrap();
}

static VALIDATORS: phf::Map<&str, fn(&str) -> bool> = phf_map! {
    "byr" => |s| validate_number_between(s, 1920..=2002),
    "iyr" => |s| validate_number_between(s, 2010..=2020),
    "eyr" => |s| validate_number_between(s, 2020..=2030),
    "hgt" => validate_hgt,
    "hcl" => validate_hcl,
    "ecl" => |s| phf_set!{"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}.contains(s),
    "pid" => validate_pid,
};

fn is_valid2(raw_passport: Vec<&str>) -> bool {
    let passport = parse_passport(raw_passport);
    return VALIDATORS.entries().all(|(key, is_valid)|{
        match passport.get(key) {
            Some(value) => is_valid(value),
            None => false
        }
    });
}

fn parse_passport(passport: Vec<&str>) -> HashMap<&str, &str> {
    return passport.iter().map(|entry| { 
        match entry.split(":").collect::<Vec<&str>>()[..] {
            [key, value] => (key, value),
            _ => panic!("Invalid key-value {:?}", entry),
        }
    }).collect();
}

fn validate_number_between(s: &str, r: RangeInclusive<i64>) -> bool {
    s.parse::<i64>().map(|x| if r.contains(&x) { true } else { false } ).unwrap_or(false)
}

fn validate_hgt(s: &str) -> bool {
    let captures = HGT_REGEX.captures(s);
    return captures.and_then(|c| { 
        let cm_validation = 
            c.get(1)
            .and_then(|n| {
                let n = n.as_str().parse::<i32>().unwrap();
                if n >= 150 && n <= 193 { Some(true) } else { None } 
            });
        let in_validation =
            c.get(2)
            .and_then(|n| {
                let n = n.as_str().parse::<i32>().unwrap();
                if n >= 59 && n <= 76 { Some(true) } else { None } 
            });
        cm_validation.or(in_validation)
    }).unwrap_or(false);
}

fn validate_hcl(s: &str) -> bool {
    HCL_REGEX.is_match(s)
}

fn validate_pid(s: &str) -> bool {
    s.len() == 9 && validate_number_between(s, 1..=999_999_999)
}

#[test]
fn test_validate_hgt() {
    assert_eq!(true, validate_hgt("190cm"));
    assert_eq!(true, validate_hgt("60in"));
    assert_eq!(false, validate_hgt("190in"));
    assert_eq!(false, validate_hgt("190"));
}

#[test]
fn test_validate_hcl() {
    assert_eq!(true, validate_hcl("#123abc"));
    assert_eq!(false, validate_hcl("#123abz"));
    assert_eq!(false, validate_hcl("123abc"));
}

#[test]
fn test_validate_pid() {
    assert_eq!(true, validate_pid("000000001"));
    assert_eq!(false, validate_pid("0123456789"));
    assert_eq!(false, validate_pid("77110462"));
}