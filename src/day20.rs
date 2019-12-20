use itertools::iproduct;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

type Portal = (char, char, char); // char char i/o

fn find_all_portals(map: &[Vec<u8>]) -> BTreeMap<Portal, (i32, i32)> {
    let (n, m) = (map.len() as i32, map[0].len() as i32);
    let mut portals = BTreeMap::new();
    let at = |y, x| map[y as usize][x as usize];
    for (y, x) in iproduct!(0..n, 0..m) {
        // find '.' with an uppercase around
        if at(y, x) != b'.' {
            continue;
        }
        for (dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let c0 = at(y + dy, x + dx) as char;
            if !c0.is_ascii_uppercase() {
                continue;
            }
            let c1 = at(y + 2 * dy, x + 2 * dx) as char;
            let (c0, c1) = if c0 <= c1 { (c0, c1) } else { (c1, c0) };
            let in_out = if y == 2 || x == 2 || y == n - 3 || x == m - 3 {
                'o'
            } else {
                'i'
            };
            portals.insert((c0, c1, in_out), (y, x));
        }
    }
    portals
}

fn bfs(
    map: &[Vec<u8>],
    start: (i32, i32),
    pos_to_portals: &BTreeMap<(i32, i32), Portal>,
) -> Vec<(Portal, i32)> {
    let at = |y, x| map[y as usize][x as usize];
    let mut result = vec![];
    let mut saw = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    saw.insert(start);
    while let Some(((y, x), d)) = q.pop_front() {
        for (dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (ny, nx) = (y + dy, x + dx);
            if saw.contains(&(ny, nx)) || at(ny, nx) != b'.' {
                continue;
            }
            q.push_back(((ny, nx), d + 1));
            saw.insert((ny, nx));
            if let Some(&portal) = pos_to_portals.get(&(ny, nx)) {
                result.push((portal, d + 1));
            }
        }
    }
    result
}

pub fn run() {
    let txt = crate::common::get_input(20).unwrap();
    let map: Vec<Vec<u8>> = txt
        .trim_end_matches('\n')
        .split('\n')
        .map(|line| line.as_bytes().to_vec())
        .collect();

    // find all portals and build two way maps
    let portals = find_all_portals(&map);
    let pos_to_portals = portals
        .iter()
        .map(|(&portal, &pos)| (pos, portal))
        .collect();

    // find all ways and distances between portals with bfs
    let mut distance = BTreeMap::new();
    for (&pos, &start) in &pos_to_portals {
        for (end, d) in bfs(&map, pos, &pos_to_portals) {
            let e = distance.entry((start, end)).or_insert(std::i32::MAX);
            *e = d.min(*e);
            let e = distance.entry((end, start)).or_insert(std::i32::MAX);
            *e = d.min(*e);
        }
    }

    // travel rules for part one
    let mut dis = distance.clone();
    for &from in portals.keys() {
        dis.insert((from, from), 0);
        let to = (from.0, from.1, if from.2 == 'i' { 'o' } else { 'i' });
        if portals.contains_key(&to) {
            dis.insert((from, to), 1);
            dis.insert((to, from), 1);
        }
    }

    // Floyd-Warshall
    for &k in portals.keys() {
        for &i in portals.keys() {
            if !dis.contains_key(&(i, k)) {
                continue;
            }
            let &dik = dis.get(&(i, k)).unwrap();
            for &j in portals.keys() {
                if !dis.contains_key(&(k, j)) {
                    continue;
                }
                let &dkj = dis.get(&(k, j)).unwrap();
                let &dij = dis.get(&(i, j)).unwrap_or(&std::i32::MAX);
                if dij > dik + dkj {
                    dis.insert((i, j), dik + dkj);
                }
            }
        }
    }
    println!("{}", dis.get(&(('A', 'A', 'o'), ('Z', 'Z', 'o'))).unwrap());

    // part two
    let mut q = vec![];
    let mut saw = HashMap::new();
    q.push((('A', 'A', 'o'), 0, 0)); // portal, level, distance_so_far
    saw.insert((('A', 'A', 'o'), 0), 0);

    while let Some((from, level, d)) = q.pop() {
        if from == ('Z', 'Z', 'o') && level == 0 {
            println!("{}", d);
            break;
        }
        // move to same levels
        for &to in portals.keys() {
            if let Some(d2) = distance.get(&(from, to)) {
                let d = d + d2;
                if d < *saw.get(&(to, level)).unwrap_or(&std::i32::MAX) {
                    q.push((to, level, d));
                    saw.insert((to, level), d);
                }
            }
        }
        // move to inner or outer level
        let to = (from.0, from.1, if from.2 == 'i' { 'o' } else { 'i' });
        let to_level = if from.2 == 'i' { level + 1 } else { level - 1 };
        // no negative levels
        if to_level >= 0 && portals.contains_key(&to) {
            let d = d + 1;
            if d < *saw.get(&(to, to_level)).unwrap_or(&std::i32::MAX) {
                q.push((to, to_level, d));
                saw.insert((to, to_level), d);
            }
        }
        q.sort_by_key(|(_, _, d)| -d); // fake dijkstra
    }
}
