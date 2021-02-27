use std::collections::HashMap;

pub fn part1(inp: String) {
    println!(
        "{}",
        simulate_and_count_occupied_seats(inp, build_neighbour_map1, apply_rules1)
    );
}

pub fn part2(inp: String) {
    println!(
        "{}",
        simulate_and_count_occupied_seats(inp, build_neighbour_map2, apply_rules2)
    );
}

type NeighbourMap = HashMap<(usize, usize), Vec<(usize, usize)>>;
type Seats = Vec<Vec<char>>;

fn simulate_and_count_occupied_seats(
    seats: String,
    neighbour_map_builder: fn(&Seats) -> NeighbourMap,
    rules_applier: fn(seat: char, adjacent_seats: Vec<char>) -> char,
) -> usize {
    let seats: Seats = seats.lines().map(|l| l.chars().collect()).collect();
    let neighbour_map = neighbour_map_builder(&seats);
    return simulate_seatings_until_no_change_and_count_occupied_seats(
        seats,
        neighbour_map,
        rules_applier,
    );
}

fn simulate_seatings_until_no_change_and_count_occupied_seats(
    mut seats: Seats,
    neighbour_map: NeighbourMap,
    rules_applier: fn(seat: char, adjacent_seats: Vec<char>) -> char,
) -> usize {
    loop {
        let new_seats = simulate_seating(seats.clone(), &neighbour_map, rules_applier);
        if new_seats == seats {
            return seats
                .iter()
                .map(|seat_line| {
                    seat_line
                        .iter()
                        .cloned()
                        .filter(|seat| *seat == '#')
                        .count()
                })
                .sum();
        }
        seats = new_seats;
    }
}

fn simulate_seating(
    seats: Seats,
    neighbour_map: &NeighbourMap,
    rules_applier: fn(seat: char, adjacent_seats: Vec<char>) -> char,
) -> Seats {
    seats
        .iter()
        .enumerate()
        .map(|(i, seat_line)| {
            seat_line
                .iter()
                .enumerate()
                .map(|(j, seat)| {
                    let adjacent_seats = neighbour_map[&(i, j)]
                        .iter()
                        .map(|(i, j)| seats[*i][*j])
                        .collect();
                    rules_applier(*seat, adjacent_seats)
                })
                .collect()
        })
        .collect()
}

static ADJACENT_POSITIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn build_neighbour_map1(seats: &Seats) -> NeighbourMap {
    seats
        .iter()
        .enumerate()
        .map(|(i, seat_line)| {
            seat_line
                .iter()
                .enumerate()
                .map(|(j, _)| {
                    let neighbours = ADJACENT_POSITIONS
                        .iter()
                        .filter_map(|(di, dj)| {
                            let new_i = i as i32 + *di;
                            if (0..seats.len() as i32).contains(&new_i) {
                                let new_j = j as i32 + *dj;
                                if (0..seats[new_i as usize].len() as i32).contains(&new_j) {
                                    Some((new_i as usize, new_j as usize))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<(usize, usize)>>();
                    ((i, j), neighbours)
                })
                .collect::<Vec<((usize, usize), Vec<(usize, usize)>)>>()
        })
        .flatten()
        .collect()
}

fn apply_rules1(seat: char, adjacent_seats: Vec<char>) -> char {
    match seat {
        'L' => {
            if adjacent_seats.iter().any(|s| *s == '#') {
                'L'
            } else {
                '#'
            }
        }
        '#' => {
            if adjacent_seats.iter().cloned().filter(|s| *s == '#').count() >= 4 {
                'L'
            } else {
                '#'
            }
        }
        _ => seat,
    }
}

fn build_neighbour_map2(seats: &Seats) -> NeighbourMap {
    seats
        .iter()
        .enumerate()
        .map(|(i, seat_line)| {
            seat_line
                .iter()
                .enumerate()
                .map(|(j, _)| {
                    let neighbours = ADJACENT_POSITIONS
                        .iter()
                        .filter_map(|(di, dj)| find_seat(&seats, i as i32, j as i32, *di, *dj))
                        .collect::<Vec<(usize, usize)>>();
                    ((i, j), neighbours)
                })
                .collect::<Vec<((usize, usize), Vec<(usize, usize)>)>>()
        })
        .flatten()
        .collect()
}

fn find_seat(seats: &Seats, i: i32, j: i32, di: i32, dj: i32) -> Option<(usize, usize)> {
    let new_i = i as i32 + di;
    if (0..seats.len() as i32).contains(&new_i) {
        let new_j = j as i32 + dj;
        if (0..seats[new_i as usize].len() as i32).contains(&new_j) {
            let seat = seats[new_i as usize][new_j as usize];
            if seat != '.' {
                Some((new_i as usize, new_j as usize))
            } else {
                find_seat(seats, new_i, new_j, di, dj)
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn apply_rules2(seat: char, adjacent_seats: Vec<char>) -> char {
    match seat {
        'L' => {
            if adjacent_seats.iter().any(|s| *s == '#') {
                'L'
            } else {
                '#'
            }
        }
        '#' => {
            if adjacent_seats.iter().cloned().filter(|s| *s == '#').count() >= 5 {
                'L'
            } else {
                '#'
            }
        }
        _ => seat,
    }
}

#[test]
fn test_simulate_and_count_occupied_seats1() {
    let seats = r#"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"#;
    assert_eq!(
        37,
        simulate_and_count_occupied_seats(seats.to_string(), build_neighbour_map1, apply_rules1)
    );
}

#[test]
fn test_simulate_and_count_occupied_seats2() {
    let seats = r#"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"#;
    assert_eq!(
        26,
        simulate_and_count_occupied_seats(seats.to_string(), build_neighbour_map2, apply_rules2)
    );
}
