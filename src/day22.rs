use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn part1(inp: String) {
    println!(
        "{}",
        parse_player_decks_and_simulate_combat_and_compute_score(inp, simulate_combat)
    );
}

pub fn part2(inp: String) {
    println!(
        "{}",
        parse_player_decks_and_simulate_combat_and_compute_score(inp, simulate_recursive_combat)
    );
}

type Card = u8;
type Deck = VecDeque<Card>;

fn parse_player_decks_and_simulate_combat_and_compute_score(
    inp: String,
    simulator: fn(Deck, Deck) -> Deck,
) -> i32 {
    let (deck1, deck2) = parse_player_decks(inp);
    let winner_deck = simulator(deck1, deck2);
    compute_score(winner_deck)
}

fn parse_player_decks(inp: String) -> (Deck, Deck) {
    inp.split("\n\n")
        .map(|deck_str| {
            deck_str
                .lines()
                .skip(1)
                .map(|card_str| card_str.parse::<Card>().unwrap())
                .collect()
        })
        .next_tuple()
        .unwrap()
}

fn simulate_combat(mut deck1: Deck, mut deck2: Deck) -> Deck {
    loop {
        if deck1.len() == 0 {
            return deck2;
        } else if deck2.len() == 0 {
            return deck1;
        }
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else if card2 > card1 {
            deck2.push_back(card2);
            deck2.push_back(card1);
        } else {
            panic!("Card1 == Card2, {}, {}", card1, card2);
        }
    }
}

fn simulate_recursive_combat(deck1: Deck, deck2: Deck) -> Deck {
    let (_, deck) = do_simulate_recursive_combat(deck1, deck2);
    deck
}

fn do_simulate_recursive_combat(mut deck1: Deck, mut deck2: Deck) -> (u8, Deck) {
    let mut previous_configurations: HashSet<(Deck, Deck)> = HashSet::new();
    loop {
        let configuration = (deck1.clone(), deck2.clone());
        if deck1.len() == 0 {
            return (2, deck2);
        } else if deck2.len() == 0 {
            return (1, deck1);
        } else if previous_configurations.contains(&configuration) {
            return (1, deck1);
        }
        previous_configurations.insert(configuration);
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 as usize <= deck1.len() && card2 as usize <= deck2.len() {
            match do_simulate_recursive_combat(
                deck1.iter().take(card1 as usize).cloned().collect(),
                deck2.iter().take(card2 as usize).cloned().collect(),
            ) {
                (1, _) => {
                    deck1.push_back(card1);
                    deck1.push_back(card2);
                }
                (2, _) => {
                    deck2.push_back(card2);
                    deck2.push_back(card1);
                }
                res => panic!("Unrecognized result {:?}", res),
            }
        } else if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else if card2 > card1 {
            deck2.push_back(card2);
            deck2.push_back(card1);
        } else {
            panic!("Card1 == Card2, {}, {}", card1, card2);
        }
    }
}

fn compute_score(deck: Deck) -> i32 {
    let deck_size = deck.len();
    deck.iter()
        .enumerate()
        .map(|(i, card)| (deck_size - i) as i32 * *card as i32)
        .sum::<i32>()
}

#[test]
fn test_parse_player_decks_and_simulate_combat_and_compute_score() {
    assert_eq!(
        306,
        parse_player_decks_and_simulate_combat_and_compute_score(
            TEST_INPUT.to_string(),
            simulate_combat
        )
    )
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;
