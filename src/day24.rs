use std::collections::{BTreeMap, HashSet};

pub fn run() {
    let txt = crate::common::get_input(24).unwrap();
    let init = txt
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c == '#')
        .collect::<Vec<_>>();

    //  0  1  2  3  4
    //  5  6  7  8  9
    // 10 11 12 13 14
    // 15 16 17 18 19
    // 20 21 22 23 24

    let adj = (0..25)
        .map(|i| {
            let mut t = vec![];
            if i > 4 {
                t.push(i - 5);
            }
            if i < 20 {
                t.push(i + 5);
            }
            if i % 5 != 0 {
                t.push(i - 1);
            }
            if i % 5 != 4 {
                t.push(i + 1);
            }
            t
        })
        .collect::<Vec<Vec<usize>>>();

    let next = |v: Vec<bool>| {
        (0..25)
            .map(|i| {
                let bugs = adj[i].iter().map(|&i| v[i] as i32).sum();
                match (v[i], bugs) {
                    (true, 1) => true,
                    (true, _) => false,
                    (false, 1) => true,
                    (false, 2) => true,
                    (false, _) => false,
                }
            })
            .collect::<Vec<_>>()
    };

    let mut saw = HashSet::new();
    let mut cur = init.clone();
    saw.insert(cur.clone());
    loop {
        cur = next(cur);
        if saw.contains(&cur) {
            break;
        }
        saw.insert(cur.clone());
    }
    let bio: u32 = cur
        .iter()
        .enumerate()
        .map(|(i, &b)| if b { 1 << i } else { 0 })
        .sum();
    dbg!(bio);

    // part two
    let adj2 = vec![
        (0, vec![(-1, 7), (-1, 11)]),
        (1, vec![(-1, 7)]),
        (2, vec![(-1, 7)]),
        (3, vec![(-1, 7)]),
        (4, vec![(-1, 7), (-1, 13)]),
        (5, vec![(-1, 11)]),
        (7, vec![(1, 0), (1, 1), (1, 2), (1, 3), (1, 4)]),
        (9, vec![(-1, 13)]),
        (10, vec![(-1, 11)]),
        (11, vec![(1, 0), (1, 5), (1, 10), (1, 15), (1, 20)]),
        (13, vec![(1, 4), (1, 9), (1, 14), (1, 19), (1, 24)]),
        (14, vec![(-1, 13)]),
        (15, vec![(-1, 11)]),
        (17, vec![(1, 20), (1, 21), (1, 22), (1, 23), (1, 24)]),
        (19, vec![(-1, 13)]),
        (20, vec![(-1, 11), (-1, 17)]),
        (21, vec![(-1, 17)]),
        (22, vec![(-1, 17)]),
        (23, vec![(-1, 17)]),
        (24, vec![(-1, 13), (-1, 17)]),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();

    let empty = vec![false; 25];
    let mut state: BTreeMap<_, _> = vec![(0, init)].into_iter().collect();
    for minute in 0..200 {
        state = (-minute - 1..=minute + 1)
            .map(|depth| {
                let cur = state.get(&depth).unwrap_or(&empty);
                let nxt = (0..25)
                    .map(|i| {
                        if i == 12 {
                            return false;
                        }
                        // count adjacent bugs at this level
                        let mut bugs = adj[i].iter().map(|&ii| (ii != 12 && cur[ii]) as i32).sum();
                        // count adjacent bugs at higher/lower level
                        for &(di, ii) in adj2.get(&i).unwrap_or(&vec![]) {
                            let v = state.get(&(depth + di)).unwrap_or(&empty);
                            bugs += v[ii] as i32;
                        }
                        match (cur[i], bugs) {
                            (true, 1) => true,
                            (true, _) => false,
                            (false, 1) => true,
                            (false, 2) => true,
                            (false, _) => false,
                        }
                    })
                    .collect();
                (depth, nxt)
            })
            .collect();
    }
    let bugs = state
        .values()
        .map(|v| v.iter().map(|&b| b as i32).sum::<i32>())
        .sum::<i32>();
    dbg!(bugs);
}
