use std::convert::TryInto;
use std::ops::RangeInclusive;

pub fn part1(inp: String) {
    compute_valid_passwords_count(inp, is_valid_password1);
}

pub fn part2(inp: String) {
    compute_valid_passwords_count(inp, is_valid_password2);
}

type Policy = (RangeInclusive<i32>, char);
type PasswordValidator = fn(Policy, &str) -> bool;

fn compute_valid_passwords_count(inp: String, password_validator: PasswordValidator) {
    let valid_passwords_count = inp
        .lines()
        .filter(|line: &&str| {
            let splits: Vec<&str> = line.split(":").collect();
            match splits[..] {
                [policy, password] => password_validator(parse_policy(policy), password.trim()),
                _ => panic!("Found wrong entry {:?}", splits),
            }
        })
        .count();
    println!("{}", valid_passwords_count);
}

fn is_valid_password1(policy: Policy, password: &str) -> bool {
    let character_count: i32 = password
        .chars()
        .filter(|c| *c == policy.1)
        .count()
        .try_into()
        .unwrap();
    return policy.0.contains(&character_count);
}

fn is_valid_password2(policy: Policy, password: &str) -> bool {
    let character1 = password
        .chars()
        .nth((*policy.0.start() - 1) as usize)
        .unwrap();
    let character2 = password
        .chars()
        .nth((*policy.0.end() - 1) as usize)
        .unwrap();
    return (character1 == policy.1 && character2 != policy.1)
        || (character2 == policy.1 && character1 != policy.1);
}

fn parse_policy(policy: &str) -> Policy {
    match policy.split(" ").collect::<Vec<&str>>()[..] {
        [count, character] => (parse_count(count), character.chars().next().unwrap()),
        _ => panic!("Found wrong policy {:?}", policy),
    }
}

fn parse_count(count: &str) -> RangeInclusive<i32> {
    match count.split("-").collect::<Vec<&str>>()[..] {
        [start, end] => start.parse().unwrap()..=end.parse().unwrap(),
        _ => panic!("Found wrong count {:?}", count),
    }
}
