use regex::Regex;
use std::clone::Clone;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::RangeInclusive;

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
}

type Ticket = Vec<i32>;

#[derive(Clone)]
struct Field {
    name: String,
    rule: Rule,
}

#[derive(Clone)]
struct Rule {
    range1: RangeInclusive<i32>,
    range2: RangeInclusive<i32>,
}

#[derive(Clone)]
struct Validator {
    valid_numbers: HashSet<i32>,
}

impl Validator {
    fn is_valid(self, number: i32) -> bool {
        self.valid_numbers.contains(&number)
    }
}

pub fn part1(inp: String) {
    println!("{}", compute_ticket_scanning_error(inp));
}

pub fn part2(inp: String) {
    let (fields, validator, ticket, nearby_tickets) = parse_input(inp);
    let valid_nearby_tickets = nearby_tickets
        .iter()
        .filter(|ticket| ticket.iter().all(|n| validator.clone().is_valid(*n)))
        .cloned()
        .collect();
    let field_indices = build_field_indices(fields.clone(), valid_nearby_tickets);
    let result: i64 = fields
        .iter()
        .filter_map(|field| {
            if field.name.starts_with("departure") {
                Some(ticket[*field_indices.get(&field.name).unwrap()] as i64)
            } else {
                None
            }
        })
        .product();
    println!("{}", result);
}

fn compute_ticket_scanning_error(inp: String) -> i32 {
    let (_, validator, _, nearby_tickets) = parse_input(inp);
    nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .cloned()
                .filter(|number| !validator.clone().is_valid(*number))
                .sum::<i32>()
        })
        .sum()
}

fn parse_input(inp: String) -> (Vec<Field>, Validator, Ticket, Vec<Ticket>) {
    match inp.split("\n\n").collect::<Vec<&str>>()[..] {
        [rules_str, ticket_str, nearby_tickets_str] => {
            let fields: Vec<Field> = rules_str.lines().map(parse_field).collect();
            let validator = rules_to_validator(fields.clone());
            let ticket: Ticket = ticket_str.lines().skip(1).map(parse_ticket).next().unwrap();
            let nearby_tickets: Vec<Ticket> = nearby_tickets_str
                .lines()
                .skip(1)
                .map(parse_ticket)
                .collect();
            return (fields, validator, ticket, nearby_tickets);
        }
        _ => panic!("Invalid input"),
    }
}

fn parse_field(field_str: &str) -> Field {
    let captures = RULE_REGEX.captures(field_str).unwrap();
    let name = captures.get(1).unwrap().as_str().to_string();
    let start1 = captures.get(2).unwrap().as_str().parse().unwrap();
    let end1 = captures.get(3).unwrap().as_str().parse().unwrap();
    let start2 = captures.get(4).unwrap().as_str().parse().unwrap();
    let end2 = captures.get(5).unwrap().as_str().parse().unwrap();
    Field {
        name: name,
        rule: Rule {
            range1: start1..=end1,
            range2: start2..=end2,
        },
    }
}

fn rules_to_validator(fields: Vec<Field>) -> Validator {
    let mut valid_numbers = HashSet::new();
    for field in fields {
        for number in field.rule.range1 {
            valid_numbers.insert(number);
        }
        for number in field.rule.range2 {
            valid_numbers.insert(number);
        }
    }
    Validator {
        valid_numbers: valid_numbers,
    }
}

fn parse_ticket(line: &str) -> Ticket {
    line.split(",")
        .map(|number_str| number_str.parse().unwrap())
        .collect()
}

fn build_field_indices(fields: Vec<Field>, tickets: Vec<Ticket>) -> HashMap<String, usize> {
    let mut potential_indices: HashMap<String, HashSet<usize>> = HashMap::new();
    for field in fields.clone() {
        potential_indices.insert(field.name, HashSet::new());
    }
    let column_size = tickets[0].len();
    for j in 0..column_size {
        for field in &fields {
            let all_valid = (0..tickets.len()).all(|i| {
                field.rule.range1.contains(&tickets[i][j])
                    || field.rule.range2.contains(&tickets[i][j])
            });
            if all_valid {
                potential_indices.get_mut(&field.name).unwrap().insert(j);
            }
        }
    }
    let mut sorted_fields_by_indices_size: Vec<String> =
        potential_indices.keys().cloned().collect();
    sorted_fields_by_indices_size.sort_by(|name1, name2| {
        potential_indices
            .get(name1)
            .unwrap()
            .len()
            .cmp(&potential_indices.get(name2).unwrap().len())
    });
    let mut names_left: HashSet<String> =
        HashSet::from_iter(sorted_fields_by_indices_size.iter().cloned());
    let mut indices = HashMap::new();
    for name in sorted_fields_by_indices_size {
        let potential_indices_for_name = potential_indices.get_mut(&name).unwrap();
        assert!(potential_indices_for_name.len() == 1);
        let index = *potential_indices_for_name.iter().next().unwrap();
        indices.insert(name.clone(), index);
        names_left.remove(&name);
        for name_left in &names_left {
            potential_indices.get_mut(name_left).unwrap().remove(&index);
        }
    }
    indices
}
