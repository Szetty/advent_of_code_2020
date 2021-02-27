use std::collections::HashMap;
use std::iter::FromIterator;

struct Cups {
    next_map: HashMap<Cup, Cup>,
    first: Option<Cup>,
    last: Option<Cup>,
}
impl Cups {
    fn new() -> Self {
        Self {
            next_map: HashMap::new(),
            first: None,
            last: None,
        }
    }
    fn push_all<T: IntoIterator<Item = Cup>>(&mut self, data: T) {
        for elem in data.into_iter() {
            match self.last {
                Some(last_elem) => {
                    let first = self.first.unwrap();
                    self.next_map.insert(last_elem, elem);
                    self.last = Some(elem);
                    self.next_map.insert(elem, first);
                }
                None => {
                    self.first = Some(elem);
                    self.last = Some(elem);
                }
            }
        }
    }
    fn pop_next_of(&mut self, cup: Cup) -> Cup {
        let next = *self.next_map.get(&cup).unwrap();
        let next_of_next = *self.next_map.get(&next).unwrap();
        self.next_map.insert(cup, next_of_next);
        self.next_map.remove(&next);
        next
    }
    fn push_next_of(&mut self, cup1: Cup, cup2: Cup) {
        let old_next = *self.next_map.get(&cup1).unwrap();
        self.next_map.insert(cup1, cup2);
        self.next_map.insert(cup2, old_next);
        if self.last.unwrap() == old_next {
            self.last = Some(cup2);
        }
    }
    fn peek_next_of(&mut self, cup: Cup) -> Cup {
        *self.next_map.get(&cup).unwrap()
    }
}
impl FromIterator<Cup> for Cups {
    fn from_iter<T: IntoIterator<Item = Cup>>(t: T) -> Self {
        let mut cups = Cups::new();
        cups.push_all(t);
        cups
    }
}
type Cup = u32;
const MIN_CUP: Cup = 1;
const MAX_CUP: Cup = 9;

pub fn part1(inp: String) {
    let res = parse_input_and_simulate_moves_and_compute_result(inp, 100);
    assert_ne!("95486237", res);
    assert_ne!("95237648", res);
    assert_eq!("47382659", res);
    println!("{}", res);
}

pub fn part2(inp: String) {
    let res = parse_input_and_extend_cups_and_simulate_moves_and_compute_product_of_cups_with_star(
        inp, 10_000_000,
    );
    assert_eq!(42271866720, res);
    println!("{}", res);
}

fn parse_input_and_simulate_moves_and_compute_result(inp: String, move_nr: usize) -> String {
    let mut cups = parse_input(inp);
    simulate_moves(move_nr, &mut cups, MAX_CUP);
    compute_result(&mut cups)
}

fn parse_input_and_extend_cups_and_simulate_moves_and_compute_product_of_cups_with_star(
    inp: String,
    move_nr: usize,
) -> i64 {
    let max_cup = 1_000_000;
    let mut cups = parse_input(inp);
    extend_cups(&mut cups, MAX_CUP, max_cup);
    simulate_moves(move_nr, &mut cups, max_cup);
    compute_product_of_cups_with_star(&mut cups)
}

fn parse_input(inp: String) -> Cups {
    inp.chars()
        .map(|cup_str| cup_str.to_digit(10).unwrap() as Cup)
        .collect()
}

fn extend_cups(cups: &mut Cups, current_max: Cup, new_max: Cup) {
    cups.push_all(((current_max + 1)..=new_max).into_iter());
}

fn simulate_moves(mut move_nr: usize, cups: &mut Cups, max_cup: Cup) {
    let mut current_cup = cups.first.unwrap();
    while move_nr > 0 {
        let cup1 = cups.pop_next_of(current_cup);
        let cup2 = cups.pop_next_of(current_cup);
        let cup3 = cups.pop_next_of(current_cup);
        let mut destination_cup = decrement_cup(current_cup, max_cup);
        while destination_cup == cup1 || destination_cup == cup2 || destination_cup == cup3 {
            destination_cup = decrement_cup(destination_cup, max_cup);
        }
        cups.push_next_of(destination_cup, cup3);
        cups.push_next_of(destination_cup, cup2);
        cups.push_next_of(destination_cup, cup1);
        current_cup = cups.peek_next_of(current_cup);
        move_nr -= 1;
    }
}

fn compute_result(cups: &mut Cups) -> String {
    let mut current_cup = cups.peek_next_of(MIN_CUP);
    let mut res = "".to_string();
    while current_cup != MIN_CUP {
        res = format!("{}{}", res, current_cup);
        current_cup = cups.peek_next_of(current_cup);
    }
    res
}

fn compute_product_of_cups_with_star(cups: &mut Cups) -> i64 {
    let cup1 = cups.peek_next_of(MIN_CUP);
    let cup2 = cups.peek_next_of(cup1);
    cup1 as i64 * cup2 as i64
}

fn decrement_cup(a: Cup, max_cup: Cup) -> Cup {
    if a == MIN_CUP {
        return max_cup;
    }
    a - 1
}

#[test]
fn test_parse_input_and_simulate_moves_and_compute_result() {
    assert_eq!(
        "92658374",
        parse_input_and_simulate_moves_and_compute_result("389125467".to_string(), 10)
    );
    assert_eq!(
        "67384529",
        parse_input_and_simulate_moves_and_compute_result("389125467".to_string(), 100)
    );
}

#[test]
fn test_parse_input_and_extend_cups_and_simulate_moves_and_compute_product_of_cups_with_star() {
    let res = parse_input_and_extend_cups_and_simulate_moves_and_compute_product_of_cups_with_star(
        "389125467".to_string(),
        10_000_000,
    );
    assert_eq!(149245887792, res);
}
