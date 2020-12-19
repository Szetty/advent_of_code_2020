use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashSet;

pub fn part1(inp: String) {
    println!("{}", parse_input_and_simulate_cycles1(inp))
}

pub fn part2(inp: String) {
    println!("{}", parse_input_and_simulate_cycles2(inp))
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
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
    fn compute_neighbours(self) -> HashSet<Self> {
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

fn parse_input_and_simulate_cycles1(inp: String) -> i32 {
    let active_cubes = parse_input1(inp);
    simulate_cycles1(6, active_cubes)
}

fn parse_input1(inp: String) -> HashSet<Cube> {
    let mut active_cubes = HashSet::new();
    inp.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                active_cubes.insert(Cube::init(x as i32, y as i32));
            }
        })
    });
    active_cubes
}

fn simulate_cycles1(mut count: i32, mut active_cubes: HashSet<Cube>) -> i32 {
    while count > 0 {
        active_cubes = simulate_cycle1(active_cubes.clone());
        // display_cubes1(active_cubes.clone());
        count = count - 1;
    }
    active_cubes.len() as i32
}

fn simulate_cycle1(active_cubes: HashSet<Cube>) -> HashSet<Cube> {
    let mut new_active_cubes = HashSet::new();
    let mut neighbours_to_consider: HashSet<Cube> = HashSet::new();
    for cube in active_cubes.clone().into_iter() {
        let neighbours = cube.clone().compute_neighbours();
        neighbours_to_consider = neighbours_to_consider.union(&neighbours).cloned().collect();
        let active_neighbours_count = neighbours
            .iter()
            .cloned()
            .filter(|neighbour| active_cubes.contains(neighbour))
            .count();
        if active_neighbours_count == 2 || active_neighbours_count == 3 {
            new_active_cubes.insert(cube);
        }
    }
    for cube in neighbours_to_consider.into_iter() {
        let neighbours = cube.clone().compute_neighbours();
        let active_neighbours_count = neighbours
            .iter()
            .filter(|neighbour| active_cubes.contains(neighbour))
            .count();
        if active_neighbours_count == 3 {
            new_active_cubes.insert(cube);
        }
    }
    new_active_cubes
}

#[allow(dead_code)]
fn display_cubes1(active_cubes: HashSet<Cube>) {
    let (x_range, y_range, z_range) = active_cubes.iter().fold(
        (0..=0, 0..=0, 0..=0),
        |(x_range, y_range, z_range), cube| {
            let x_min = x_range.start().min(&cube.x);
            let x_max = x_range.end().max(&cube.x);
            let y_min = y_range.start().min(&cube.y);
            let y_max = y_range.end().max(&cube.y);
            let z_min = z_range.start().min(&cube.z);
            let z_max = z_range.end().max(&cube.z);
            (*x_min..=*x_max, *y_min..=*y_max, *z_min..=*z_max)
        },
    );
    for z in z_range {
        println!("z={}", z);
        for y in y_range.clone() {
            for x in x_range.clone() {
                if active_cubes.contains(&Cube::new(x, y, z)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
        println!()
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
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
    fn compute_neighbours(self) -> HashSet<Self> {
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

fn parse_input_and_simulate_cycles2(inp: String) -> i32 {
    let active_cubes = parse_input2(inp);
    simulate_cycles2(6, active_cubes)
}

fn parse_input2(inp: String) -> HashSet<HyperCube> {
    let mut active_cubes = HashSet::new();
    inp.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                active_cubes.insert(HyperCube::init(x as i32, y as i32));
            }
        })
    });
    active_cubes
}

fn simulate_cycles2(mut count: i32, mut active_cubes: HashSet<HyperCube>) -> i32 {
    while count > 0 {
        active_cubes = simulate_cycle2(active_cubes.clone());
        count = count - 1;
    }
    active_cubes.len() as i32
}

fn simulate_cycle2(active_cubes: HashSet<HyperCube>) -> HashSet<HyperCube> {
    let mut new_active_cubes = HashSet::new();
    let mut neighbours_to_consider: HashSet<HyperCube> = HashSet::new();
    for cube in active_cubes.clone().into_iter() {
        let neighbours = cube.clone().compute_neighbours();
        neighbours_to_consider = neighbours_to_consider.union(&neighbours).cloned().collect();
        let active_neighbours_count = neighbours
            .iter()
            .cloned()
            .filter(|neighbour| active_cubes.contains(neighbour))
            .count();
        if active_neighbours_count == 2 || active_neighbours_count == 3 {
            new_active_cubes.insert(cube);
        }
    }
    for cube in neighbours_to_consider.into_iter() {
        let neighbours = cube.clone().compute_neighbours();
        let active_neighbours_count = neighbours
            .iter()
            .filter(|neighbour| active_cubes.contains(neighbour))
            .count();
        if active_neighbours_count == 3 {
            new_active_cubes.insert(cube);
        }
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
