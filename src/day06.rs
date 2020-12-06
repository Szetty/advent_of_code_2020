use std::collections::HashSet;

pub fn part1(inp: String) {
    println!("{:?}", COUNT_ANY_CHARS_IN_GROUPS(inp));
}

pub fn part2(inp: String) {
    println!("{:?}", COUNT_ALL_CHARS_IN_GROUPS(inp));
}

fn count_chars_from_groups_with_combiner(inp: String, combiner: fn(&HashSet<char>, &HashSet<char>) -> HashSet<char>) -> usize {
    inp
    .split("\n\n")
    .map(|group_str| {
        group_str
        .split("\n")
        .map(|entry| entry.chars().collect::<HashSet<char>>())
        .fold(None, |acc: Option<HashSet<char>>, s| {
            match acc {
                Some(acc_s) => Some(combiner(&acc_s, &s)),
                None => Some(s),
            }
        })
        .unwrap_or_default()
        .len()
    })
    .sum()
}

static COUNT_ALL_CHARS_IN_GROUPS: fn(String) -> usize = |x| { 
    count_chars_from_groups_with_combiner(x, |s1, s2| s1.intersection(s2).map(|x| *x).collect()) 
};
static COUNT_ANY_CHARS_IN_GROUPS: fn(String) -> usize = |x| { 
    count_chars_from_groups_with_combiner(x, |s1, s2| s1.union(s2).map(|x| *x).collect())
};

#[test]
fn test_count_any_chars_in_groups() {
    assert_eq!(
        COUNT_ANY_CHARS_IN_GROUPS("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".to_string()),
        11
    )
}

#[test]
fn test_count_all_chars_in_groups() {
    assert_eq!(
        COUNT_ALL_CHARS_IN_GROUPS("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".to_string()),
        6
    )
}