use itertools::Itertools;
use regex::Regex;
use std::clone::Clone;
use std::collections::{HashMap, HashSet};

pub fn part1(inp: String) {
    println!("{}", parse_input_and_compute_non_allergen_count(inp));
}

pub fn part2(inp: String) {
    println!(
        "{}",
        parse_input_and_compute_canonical_dangerous_ingredients(inp)
    );
}

#[derive(Debug, Clone)]
struct Food {
    ingredients: Ingredients,
    allergens: Allergens,
}
type Ingredients = HashSet<Ingredient>;
type Ingredient = String;
type Allergens = HashSet<Allergen>;
type Allergen = String;

lazy_static! {
    static ref FOOD_REGEX: Regex =
        Regex::new(r"^([a-z]+(?: [a-z]+)*)(?: \(contains ([a-z]+(?:, [a-z]+)*)\))?$").unwrap();
}

fn parse_input_and_compute_non_allergen_count(inp: String) -> i32 {
    let foods = parse_input(inp);
    compute_non_allergen_count(foods)
}

fn parse_input_and_compute_canonical_dangerous_ingredients(inp: String) -> String {
    let foods = parse_input(inp);
    compute_canonical_dangerous_ingredients(foods)
}

fn parse_input(inp: String) -> Vec<Food> {
    inp.lines().map(parse_food).collect()
}

fn parse_food(food_str: &str) -> Food {
    let captures = FOOD_REGEX.captures(food_str).unwrap();
    let ingredients = captures
        .get(1)
        .unwrap()
        .as_str()
        .split(" ")
        .map(|ingredient| ingredient.to_string())
        .collect();
    let allergens = captures
        .get(2)
        .unwrap()
        .as_str()
        .split(", ")
        .map(|allergen| allergen.to_string())
        .collect();
    Food {
        ingredients: ingredients,
        allergens: allergens,
    }
}

fn compute_non_allergen_count(foods: Vec<Food>) -> i32 {
    let ingredients_with_allergen = match_allergens_to_ingredients(foods.clone());
    calculate_non_allergen_count(foods, ingredients_with_allergen.keys().cloned().collect())
}

fn compute_canonical_dangerous_ingredients(foods: Vec<Food>) -> String {
    let mut ingredients_with_allergen: Vec<(Ingredient, Allergen)> =
        match_allergens_to_ingredients(foods.clone())
            .into_iter()
            .collect();
    ingredients_with_allergen.sort_by(|(_, allergen1), (_, allergen2)| allergen1.cmp(allergen2));
    ingredients_with_allergen
        .iter()
        .map(|(ingredient, _)| ingredient)
        .join(",")
}

fn match_allergens_to_ingredients(foods: Vec<Food>) -> HashMap<Ingredient, Allergen> {
    let mut ingredients_by_allergen: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();
    for food in foods.iter() {
        for allergen in food.allergens.clone().iter() {
            match ingredients_by_allergen.get(allergen) {
                Some(ingredients) => {
                    let new_ingredients = ingredients
                        .intersection(&food.ingredients.clone())
                        .cloned()
                        .collect();
                    ingredients_by_allergen.insert(allergen.clone(), new_ingredients);
                }
                None => {
                    ingredients_by_allergen.insert(allergen.clone(), food.ingredients.clone());
                }
            }
        }
    }
    let mut ingredients_with_allergen: HashMap<Ingredient, Allergen> = HashMap::new();
    while ingredients_by_allergen.len() > 0 {
        let ingredients_by_allergen_clone = ingredients_by_allergen.clone();
        let (allergen, ingredients) = ingredients_by_allergen_clone
            .iter()
            .find(|(_, ingredients)| ingredients.len() == 1)
            .unwrap();
        let ingredient = ingredients.iter().next().unwrap();
        ingredients_with_allergen.insert(ingredient.clone(), allergen.clone());
        ingredients_by_allergen.remove(allergen);
        for ingredients in ingredients_by_allergen.values_mut() {
            ingredients.remove(ingredient);
        }
    }
    ingredients_with_allergen
}

fn calculate_non_allergen_count(
    foods: Vec<Food>,
    ingredients_with_allergen: HashSet<Ingredient>,
) -> i32 {
    foods
        .iter()
        .map(|food| {
            food.ingredients
                .iter()
                .filter(|ingredient| !ingredients_with_allergen.contains(ingredient.clone()))
                .count() as i32
        })
        .sum()
}

#[test]
fn test_parse_input_and_compute_non_allergen_count() {
    assert_eq!(
        5,
        parse_input_and_compute_non_allergen_count(TEST_INPUT.to_string())
    );
}

#[test]
fn test_parse_input_and_compute_canonical_dangerous_ingredients() {
    assert_eq!(
        "mxmxvkd,sqjhc,fvjkl",
        parse_input_and_compute_canonical_dangerous_ingredients(TEST_INPUT.to_string())
    );
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;
