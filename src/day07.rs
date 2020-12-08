use regex::Regex;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

pub fn part1(inp: String) {
    let rules = parse_rules(inp);
    let count = compute_uniq_containers_count_containing(rules, "shiny gold");
    println!("{:?}", count);
}

pub fn part2(inp: String) {
    let rules = parse_rules(inp);
    let count = compute_contained_bags_count_of(rules, "shiny gold".to_string());
    println!("{:?}", count);
}

lazy_static!{
    static ref RULE_REGEX: Regex = Regex::new(r"^([a-z\s]+) bags contain ((\d+\s[a-z\s]+(,\s\d+[a-z\s]+)*)|(no other bags))\.$").unwrap();
    static ref CONTENT_REGEX: Regex = Regex::new(r"^(\d+)(\s[a-z\s]+)$").unwrap();
}

#[derive(Debug, Clone)]
struct Content {
    name: String,
    count: i32
}

fn parse_rules(inp: String) -> HashMap<String, Vec<Content>> {
    return 
        inp.lines()
        .map(|l| {
            let captures = RULE_REGEX.captures(l).unwrap();
            let container = captures.get(1).unwrap().as_str().trim().to_string();
            let contents: Vec<Content> = 
                captures.get(3).map(|c| 
                    c.as_str().split(", ").map(parse_content).collect()
                ).unwrap_or_default();
            (container, contents)
        })
        .collect();
}

fn parse_content(s: &str) -> Content {
    let captures = CONTENT_REGEX.captures(s).unwrap();
    Content {
        name: captures.get(2).unwrap().as_str().replace("bags", "").replace("bag", "").trim().to_string(),
        count: captures.get(1).unwrap().as_str().parse::<i32>().unwrap()
    }
}

fn compute_uniq_containers_count_containing(rules: HashMap<String, Vec<Content>>, content_name: &str) -> usize {
    let mut containers = HashSet::<String>::from_iter(vec![content_name.to_string()].iter().cloned());
    loop {
        let new_containers = HashSet::from_iter(rules.iter().filter(|(_, contents)| {
            contents.iter().any(|c| containers.contains(&c.name))
        }).map(|(name, _)| name.clone()));
        let old_count = containers.len();
        containers = containers.union(&new_containers).cloned().collect();
        if old_count == containers.len() {
            return old_count - 1;
        }
    }
}

fn compute_contained_bags_count_of(rules: HashMap<String, Vec<Content>>, container_name: String) -> i32 {
    match rules.get(&container_name) {
        None => 0,
        Some(contents) => {
            contents.iter().map(|c| c.count + c.count * compute_contained_bags_count_of(rules.clone(), c.name.clone())).sum()
        }
    }
}