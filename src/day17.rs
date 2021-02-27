use std::clone::Clone;
use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn part1(inp: String) {
    println!("{}", parse_input_and_simulate_cycles1(inp))
}

pub fn part2(inp: String) {
    println!("{}", parse_input_and_simulate_cycles2(inp))
}

trait CubeLike: Eq + PartialEq + Hash + Clone + Copy {
    fn compute_neighbours(&self) -> HashSet<Self>;
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn init(x: i32, y: i32) -> Self {
        Self::new(x, y, 0)
    }
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x: x, y: y, z: z }
    }
}

impl CubeLike for Cube {
    fn compute_neighbours(&self) -> HashSet<Self> {
        let mut neighbours = HashSet::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx != 0 || dy != 0 || dz != 0 {
                        neighbours.insert(Self::new(self.x + dx, self.y + dy, self.z + dz));
                    }
                }
            }
        }
        neighbours
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
struct HyperCube {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl HyperCube {
    fn init(x: i32, y: i32) -> Self {
        Self::new(x, y, 0, 0)
    }
    fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

impl CubeLike for HyperCube {
    fn compute_neighbours(&self) -> HashSet<Self> {
        let mut neighbours = HashSet::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                            neighbours.insert(Self::new(
                                self.x + dx,
                                self.y + dy,
                                self.z + dz,
                                self.w + dw,
                            ));
                        }
                    }
                }
            }
        }
        neighbours
    }
}

fn parse_input_and_simulate_cycles1(inp: String) -> i32 {
    let active_cubes = parse_input(inp, Cube::init);
    simulate_cycles(6, active_cubes)
}

fn parse_input_and_simulate_cycles2(inp: String) -> i32 {
    let active_cubes = parse_input(inp, HyperCube::init);
    simulate_cycles(6, active_cubes)
}

fn parse_input<T: CubeLike>(inp: String, initializer: fn(i32, i32) -> T) -> HashSet<T> {
    let mut active_cubes = HashSet::new();
    inp.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                active_cubes.insert(initializer(x as i32, y as i32));
            }
        })
    });
    active_cubes
}

fn simulate_cycles<T: CubeLike>(mut count: i32, mut active_cubes: HashSet<T>) -> i32 {
    while count > 0 {
        active_cubes = simulate_cycle(active_cubes.clone());
        count = count - 1;
    }
    active_cubes.len() as i32
}

fn simulate_cycle<T: CubeLike>(active_cubes: HashSet<T>) -> HashSet<T> {
    let mut new_active_cubes = HashSet::new();
    let mut non_active_cubes_with_active_neighbour_count: HashMap<T, usize> = HashMap::new();
    for cube in (&active_cubes).iter() {
        let neighbours = cube.compute_neighbours();
        let non_active_neighbours = neighbours
            .iter()
            .cloned()
            .filter(|neighbour| !active_cubes.contains(neighbour));
        let mut non_active_neighbours_count = 0;
        for white in non_active_neighbours {
            let current_black_count = *non_active_cubes_with_active_neighbour_count
                .get(&white)
                .unwrap_or(&0);
                non_active_cubes_with_active_neighbour_count.insert(white, current_black_count + 1);
            non_active_neighbours_count += 1;
        }
        let active_neighbours_count = neighbours.len() - non_active_neighbours_count;
        if active_neighbours_count == 2 || active_neighbours_count == 3 {
            new_active_cubes.insert(*cube);
        }
    }
    for (non_active_cube, _) in non_active_cubes_with_active_neighbour_count
        .into_iter()
        .filter(|(_, count)| *count == 3)
    {
        new_active_cubes.insert(non_active_cube);
    }
    new_active_cubes
}

#[test]
fn test_parse_input_and_simulate_cycles1() {
    assert_eq!(
        112,
        parse_input_and_simulate_cycles1(".#.\n..#\n###".to_string())
    );
}

#[test]
fn test_parse_input_and_simulate_cycles2() {
    assert_eq!(
        848,
        parse_input_and_simulate_cycles2(".#.\n..#\n###".to_string())
    );
}