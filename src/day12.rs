use num::Complex;
use std::f64::consts::PI;

pub fn part1(inp: String) {
    let mut ship = Ship1 {
        pos: Complex::new(0, 0),
        dir: Complex::new(-1, 0),
    };
    ship = apply_commands1(ship, inp.lines().map(&str::to_string).collect());
    println!("{}", ship.pos.re.abs() + ship.pos.im.abs())
}

pub fn part2(inp: String) {
    let mut ship = Ship2 {
        pos: Complex::new(0, 0),
        waypoint: Complex::new(-10, 1),
    };
    ship = apply_commands2(ship, inp.lines().map(&str::to_string).collect());
    println!("{}", ship.pos.re.abs() + ship.pos.im.abs())
}

#[derive(Debug, std::cmp::PartialEq)]
struct Ship1 {
    pos: Complex<i32>,
    dir: Complex<i32>,
}

fn apply_commands1(mut ship: Ship1, commands: Vec<String>) -> Ship1 {
    for command in commands.iter() {
        match command.split_at(1) {
            ("N", value_str) => {
                ship.pos = move_pos1(ship.pos, Complex::new(0, value_str.parse::<i32>().unwrap()));
            }
            ("S", value_str) => {
                ship.pos = move_pos1(
                    ship.pos,
                    Complex::new(0, -value_str.parse::<i32>().unwrap()),
                );
            }
            ("E", value_str) => {
                ship.pos = move_pos1(
                    ship.pos,
                    Complex::new(-value_str.parse::<i32>().unwrap(), 0),
                );
            }
            ("W", value_str) => {
                ship.pos = move_pos1(ship.pos, Complex::new(value_str.parse::<i32>().unwrap(), 0));
            }
            ("L", value_str) => {
                ship.dir = turn_direction1(ship.dir, -value_str.parse::<i32>().unwrap());
            }
            ("R", value_str) => {
                ship.dir = turn_direction1(ship.dir, value_str.parse::<i32>().unwrap());
            }
            ("F", value_str) => {
                ship.pos = move_pos1(ship.pos, ship.dir.scale(value_str.parse::<i32>().unwrap()));
            }
            _ => panic!("Unknown command {}", command),
        }
    }
    ship
}

fn move_pos1(pos: Complex<i32>, delta_pos: Complex<i32>) -> Complex<i32> {
    pos + delta_pos
}

fn turn_direction1(dir: Complex<i32>, degree: i32) -> Complex<i32> {
    assert_eq!(0, degree % 90);
    let float_dir = Complex::new(dir.re as f64, dir.im as f64);
    let result = float_dir * Complex::from_polar(1.0, (degree as f64) * PI / 180.0);
    Complex::new(result.re.round() as i32, result.im.round() as i32)
}

#[test]
fn test_turn_direction1() {
    assert_eq!(
        Complex::new(0, -1),
        turn_direction1(Complex::new(-1, 0), 90)
    );
    assert_eq!(
        Complex::new(1, 0),
        turn_direction1(Complex::new(-1, 0), 180)
    );
    assert_eq!(
        Complex::new(0, 1),
        turn_direction1(Complex::new(-1, 0), 270)
    );
    assert_eq!(
        Complex::new(0, 1),
        turn_direction1(Complex::new(-1, 0), -90)
    );
    assert_eq!(
        Complex::new(1, 0),
        turn_direction1(Complex::new(-1, 0), -180)
    );
}

#[test]
fn test_apply_commands1() {
    assert_eq!(
        Ship1 {
            pos: Complex::new(-17, -8),
            dir: Complex::new(0, -1),
        },
        apply_commands1(
            Ship1 {
                pos: Complex::new(0, 0),
                dir: Complex::new(-1, 0),
            },
            vec!["F10", "N3", "F7", "R90", "F11"]
                .iter()
                .cloned()
                .map(&str::to_string)
                .collect(),
        )
    )
}

#[derive(Debug, std::cmp::PartialEq)]
struct Ship2 {
    pos: Complex<i32>,
    waypoint: Complex<i32>,
}

fn apply_commands2(mut ship: Ship2, commands: Vec<String>) -> Ship2 {
    for command in commands.iter() {
        match command.split_at(1) {
            ("N", value_str) => {
                ship.waypoint = move_waypoint(
                    ship.waypoint,
                    Complex::new(0, value_str.parse::<i32>().unwrap()),
                );
            }
            ("S", value_str) => {
                ship.waypoint = move_waypoint(
                    ship.waypoint,
                    Complex::new(0, -value_str.parse::<i32>().unwrap()),
                );
            }
            ("E", value_str) => {
                ship.waypoint = move_waypoint(
                    ship.waypoint,
                    Complex::new(-value_str.parse::<i32>().unwrap(), 0),
                );
            }
            ("W", value_str) => {
                ship.waypoint = move_waypoint(
                    ship.waypoint,
                    Complex::new(value_str.parse::<i32>().unwrap(), 0),
                );
            }
            ("L", value_str) => {
                ship.waypoint = turn_direction2(ship.waypoint, -value_str.parse::<i32>().unwrap());
            }
            ("R", value_str) => {
                ship.waypoint = turn_direction2(ship.waypoint, value_str.parse::<i32>().unwrap());
            }
            ("F", value_str) => {
                ship.pos = move_pos2(ship.pos, ship.waypoint, value_str.parse::<i32>().unwrap());
            }
            _ => panic!("Unknown command {}", command),
        }
    }
    ship
}

fn move_waypoint(waypoint: Complex<i32>, delta: Complex<i32>) -> Complex<i32> {
    waypoint + delta
}

fn move_pos2(pos: Complex<i32>, waypoint: Complex<i32>, times: i32) -> Complex<i32> {
    pos + waypoint.scale(times)
}

fn turn_direction2(waypoint: Complex<i32>, degree: i32) -> Complex<i32> {
    assert_eq!(0, degree % 90);
    let float_dir = Complex::new(waypoint.re as f64, waypoint.im as f64);
    let result = float_dir * Complex::from_polar(1.0, (degree as f64) * PI / 180.0);
    Complex::new(result.re.round() as i32, result.im.round() as i32)
}

#[test]
fn test_turn_direction2() {
    assert_eq!(
        Complex::new(0, -1),
        turn_direction1(Complex::new(-1, 0), 90)
    );
    assert_eq!(
        Complex::new(1, 0),
        turn_direction1(Complex::new(-1, 0), 180)
    );
    assert_eq!(
        Complex::new(0, 1),
        turn_direction1(Complex::new(-1, 0), 270)
    );
    assert_eq!(
        Complex::new(0, 1),
        turn_direction1(Complex::new(-1, 0), -90)
    );
    assert_eq!(
        Complex::new(1, 0),
        turn_direction1(Complex::new(-1, 0), -180)
    );
    assert_eq!(
        Complex::new(-4, -10),
        turn_direction1(Complex::new(-10, 4), 90)
    );
}

#[test]
fn test_apply_commands2() {
    assert_eq!(
        Ship2 {
            pos: Complex::new(-214, -72),
            waypoint: Complex::new(-4, -10),
        },
        apply_commands2(
            Ship2 {
                pos: Complex::new(0, 0),
                waypoint: Complex::new(-10, 1),
            },
            vec!["F10", "N3", "F7", "R90", "F11"]
                .iter()
                .cloned()
                .map(&str::to_string)
                .collect(),
        )
    )
}
