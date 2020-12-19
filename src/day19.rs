use regex::Regex;
use std::clone::Clone;
use std::collections::HashMap;

type Rules = HashMap<i32, Rule>;
type Messages = Vec<String>;
#[derive(Debug, Clone)]
enum Rule {
    Expr(Branches),
    Lit(char),
}
impl Rule {
    fn expr(self) -> Option<Branches> {
        match self {
            Rule::Expr(branches) => Some(branches),
            Rule::Lit(_) => None,
        }
    }
}
type Branches = Vec<Branch>;
type Branch = Vec<i32>;

lazy_static! {
    static ref RULE_REGEX: Regex =
        Regex::new(r#"^(\d+): (("[a-z]")|((\d+( \d+)*)( \| (\d+( \d+)*))*))$"#).unwrap();
}

pub fn part1(inp: String) {
    println!("{}", parse_input_and_count_matching_messages(inp));
}

pub fn part2(inp: String) {
    println!(
        "{}",
        parse_input_and_count_matching_messages(
            inp.replace("8: 42", "8: 42 | 42 8")
                .replace("11: 42 31", "11: 42 31 | 42 11 31")
        )
    );
}

fn parse_input_and_count_matching_messages(inp: String) -> usize {
    let (rules, messages) = parse_input(inp);
    count_matching_messages(rules, messages)
}

fn parse_input(inp: String) -> (Rules, Messages) {
    match inp.split("\n\n").collect::<Vec<&str>>()[..] {
        [rules_str, messages_str] => (
            rules_str.lines().map(parse_rule).collect(),
            messages_str.lines().map(|s| s.to_string()).collect(),
        ),
        _ => panic!("Unexpected input"),
    }
}

fn parse_rule(rule_str: &str) -> (i32, Rule) {
    let rule_captures = RULE_REGEX.captures(rule_str).unwrap();
    let rule_key = rule_captures.get(1).unwrap().as_str().parse().unwrap();
    let literal_capture = rule_captures.get(3);
    let expression_capture = rule_captures.get(4);
    match literal_capture {
        Some(literal) => (
            rule_key,
            Rule::Lit(literal.as_str().replace("\"", "").chars().next().unwrap()),
        ),
        None => (
            rule_key,
            Rule::Expr(
                expression_capture
                    .unwrap()
                    .as_str()
                    .split(" | ")
                    .map(|branch_str| branch_str.split(" ").map(|s| s.parse().unwrap()).collect())
                    .collect(),
            ),
        ),
    }
}

fn count_matching_messages(rules: Rules, messages: Messages) -> usize {
    messages
        .iter()
        .filter(|m| is_valid(rules.clone(), m.chars().collect()))
        .count()
}

fn is_valid(rules: Rules, message: Vec<char>) -> bool {
    let mut checkpoints: Vec<(Vec<i32>, usize)> = rules
        .get(&0)
        .unwrap()
        .clone()
        .expr()
        .unwrap()
        .into_iter()
        .map(|br| (br.iter().rev().cloned().collect::<Vec<i32>>(), 0))
        .collect();
    while checkpoints.len() > 0 {
        let (mut stack, mut idx) = checkpoints.pop().unwrap();
        while stack.len() > 0 {
            if idx >= message.len() {
                break;
            }
            let next_rule = rules.get(&(stack.pop().unwrap())).unwrap();
            match next_rule {
                Rule::Lit(c) => {
                    if message[idx] != *c {
                        break;
                    } else {
                        idx = idx + 1;
                    }
                }
                Rule::Expr(expr) => {
                    if expr.len() > 1 {
                        for i in 1..expr.len() {
                            let mut s = stack.clone();
                            s.append(&mut expr[i].iter().rev().cloned().collect::<Vec<i32>>());
                            checkpoints.push((s, idx));
                        }
                    }
                    stack.append(&mut expr[0].iter().rev().cloned().collect::<Vec<i32>>());
                }
            }
        }
        if stack.len() == 0 && idx >= message.len() {
            return true;
        }
    }
    false
}

#[test]
fn test_parse_input_and_count_matching_messages() {
    assert_eq!(
        2,
        parse_input_and_count_matching_messages(
            r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#
                .to_string()
        )
    )
}
