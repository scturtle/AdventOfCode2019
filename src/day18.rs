use itertools::iproduct;
use std::collections::{HashMap, HashSet, VecDeque};

fn find_entry(map: &mut [Vec<u8>]) -> (i32, i32) {
    let n = map.len();
    let m = map[0].len();
    for (i, j) in iproduct!(0..n, 0..m) {
        if map[i][j] == b'@' {
            map[i][j] = b'.';
            return (i as i32, j as i32);
        }
    }
    unreachable!();
}

fn find_door(door: u8, map: &[Vec<u8>]) -> Option<(usize, usize)> {
    let n = map.len();
    let m = map[0].len();
    for (i, j) in iproduct!(0..n, 0..m) {
        if map[i][j] == door {
            return Some((i, j));
        }
    }
    None
}

fn accessible_keys(entry: (i32, i32), map: &[Vec<u8>]) -> Vec<(i32, i32, u8, i32)> {
    let n = map.len();
    let m = map[0].len();
    let mut saw = HashSet::new();
    let mut keys = vec![];
    let mut q = VecDeque::new();
    q.push_back((entry.0, entry.1, 0));
    saw.insert(entry);
    while !q.is_empty() {
        let (i, j, cost) = q.pop_front().unwrap();
        for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (ni, nj) = (i + di, j + dj);
            if ni < 0 || nj < 0 || ni == n as i32 || nj == m as i32 || saw.contains(&(ni, nj)) {
                continue;
            }
            match map[ni as usize][nj as usize] {
                b'.' => {
                    q.push_back((ni, nj, cost + 1));
                    saw.insert((ni, nj));
                }
                key @ b'a'..=b'z' => {
                    keys.push((ni, nj, key, cost + 1));
                }
                _ => {}
            }
        }
    }
    keys
}

fn dfs(
    entry: (i32, i32),
    map: &mut Vec<Vec<u8>>,
    cache: &mut HashMap<Vec<(i32, i32)>, i32>,
) -> i32 {
    let keys = accessible_keys(entry, &map);
    if keys.is_empty() {
        return 0;
    }

    let mut k = keys.iter().map(|&(i, j, _, _)| (i, j)).collect::<Vec<_>>();
    k.sort();
    k.push(entry);
    if let Some(&v) = cache.get(&k) {
        return v;
    }

    let mut min_cost = std::i32::MAX;
    for &(i, j, key, cost_to_key) in &keys {
        let door = key - 32;
        let door_pos = find_door(door, &map);
        if let Some((di, dj)) = door_pos {
            map[di][dj] = b'.';
        }
        map[i as usize][j as usize] = b'.';
        let cost_later = dfs((i, j), map, cache);
        min_cost = min_cost.min(cost_to_key + cost_later);
        map[i as usize][j as usize] = key;
        if let Some((di, dj)) = door_pos {
            map[di][dj] = door;
        }
    }
    cache.insert(k, min_cost);
    min_cost
}

pub fn run() {
    let txt = crate::common::get_input(18).unwrap();
    let mut map: Vec<Vec<u8>> = txt
        .trim_end()
        .split('\n')
        .map(|l| l.as_bytes().to_vec())
        .collect();
    let entry = find_entry(&mut map);
    let mut cache = HashMap::new();
    let cost = dfs(entry, &mut map, &mut cache);
    dbg!(cost);
}
