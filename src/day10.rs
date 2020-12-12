use std::collections::HashMap;

pub fn part1(inp: String) {
    let mut adapter_jolts: Vec<i32> = inp.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    adapter_jolts.sort();
    let (_, one_jolt_diff_count, three_jolt_diff_count) =
        adapter_jolts.iter().fold((0, 0, 0), |acc, adapter_jolt| {
            let (previous_adapter_jolt, one_jolt_diff_count, three_jolt_diff_count) = acc;
            match adapter_jolt - previous_adapter_jolt {
                1 => (
                    *adapter_jolt,
                    one_jolt_diff_count + 1,
                    three_jolt_diff_count,
                ),
                3 => (
                    *adapter_jolt,
                    one_jolt_diff_count,
                    three_jolt_diff_count + 1,
                ),
                _ => (*adapter_jolt, one_jolt_diff_count, three_jolt_diff_count),
            }
        });
    println!("{}", one_jolt_diff_count * (three_jolt_diff_count + 1));
}

pub fn part2(inp: String) {
    let adapter_jolts: Vec<i32> = inp.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    println!("{}", sort_and_compute_distinct_ways(adapter_jolts));
}

static NEXT: [usize; 3] = [1, 2, 3];

fn sort_and_compute_distinct_ways(mut adapters: Vec<i32>) -> i64 {
    adapters.sort();
    adapters.insert(0, 0);
    return compute_distinct_ways(adapters, 0, HashMap::default())[&0];
}

fn compute_distinct_ways(
    adapters: Vec<i32>,
    current: usize,
    mut distinct_ways_map: HashMap<usize, i64>,
) -> HashMap<usize, i64> {
    let current_adapter = adapters[current];
    if distinct_ways_map.contains_key(&current) {
        return distinct_ways_map;
    }
    if current == adapters.len() - 1 {
        distinct_ways_map.insert(current, 1);
        return distinct_ways_map;
    }
    let next_indices: Vec<usize> = NEXT
        .iter()
        .map(|n| (current + n, adapters.get(current + n)))
        .filter(|(_, adapter_option)| {
            adapter_option
                .map(|adapter| adapter - current_adapter <= 3)
                .unwrap_or(false)
        })
        .map(|(index, _)| index)
        .collect();

    distinct_ways_map = next_indices
        .iter()
        .fold(distinct_ways_map, |distinct_ways_map, index| {
            compute_distinct_ways(adapters.clone(), *index, distinct_ways_map)
        });

    let result = next_indices.iter().map(|idx| distinct_ways_map[idx]).sum();
    distinct_ways_map.insert(current, result);
    distinct_ways_map
}

#[test]
fn test_compute_distinct_ways() {
    assert_eq!(
        8,
        sort_and_compute_distinct_ways(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4])
    );
    assert_eq!(
        19208,
        sort_and_compute_distinct_ways(vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3
        ])
    );
}
