use std::collections::{HashMap, HashSet};

fn parse_wire(line: &str) -> Vec<(char, i32)> {
    line.split(',')
        .map(|i| (i.as_bytes()[0] as char, i[1..].parse::<i32>().unwrap()))
        .collect()
}

fn dir2sxsy(dir: char) -> (i32, i32) {
    match dir {
        'U' => (0, 1),
        'D' => (0, -1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => unreachable!(),
    }
}

pub fn run() {
    let txt = crate::common::get_input(3).unwrap();
    let lines: Vec<_> = txt.lines().collect::<Vec<_>>();
    let wire0 = parse_wire(lines[0]);
    let wire1 = parse_wire(lines[1]);
    let mut saw = HashSet::new();
    let (mut x, mut y) = (0, 0);
    for &(dir, steps) in &wire0 {
        let (sx, sy) = dir2sxsy(dir as char);
        for _ in 0..steps {
            x += sx;
            y += sy;
            saw.insert((x, y));
        }
    }
    let mut min_dist = std::i32::MAX;
    let (mut x, mut y) = (0, 0);
    for &(dir, steps) in &wire1 {
        let (sx, sy) = dir2sxsy(dir as char);
        for _ in 0..steps {
            x += sx;
            y += sy;
            if saw.contains(&(x, y)) && min_dist > x.abs() + y.abs() {
                min_dist = x.abs() + y.abs();
            }
        }
    }
    dbg!(min_dist);
    // part two
    let mut saw = HashMap::new();
    let mut cnt = 0;
    let (mut x, mut y) = (0, 0);
    for &(dir, steps) in &wire0 {
        let (sx, sy) = dir2sxsy(dir as char);
        for _ in 0..steps {
            x += sx;
            y += sy;
            cnt += 1;
            saw.insert((x, y), cnt);
        }
    }
    let mut min_cnt = std::i32::MAX;
    let mut cnt = 0;
    let (mut x, mut y) = (0, 0);
    for &(dir, steps) in &wire1 {
        let (sx, sy) = dir2sxsy(dir as char);
        for _ in 0..steps {
            x += sx;
            y += sy;
            cnt += 1;
            if let Some(cnt0) = saw.get(&(x, y)) {
                if min_cnt > cnt + cnt0 {
                    min_cnt = cnt + cnt0;
                }
            }
        }
    }
    dbg!(min_cnt);
}
