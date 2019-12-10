use std::collections::{HashMap, VecDeque};

pub fn run() {
    let txt = crate::common::get_input(6).unwrap();
    let maps: HashMap<_, _> = txt
        .trim_end()
        .lines()
        .map(|l| {
            let v = l.split(')').collect::<Vec<_>>();
            (v[1], v[0])
        })
        .collect();
    let mut cnt = HashMap::new();
    for a in maps.values() {
        let c = cnt.entry(a).or_insert(0);
        *c += 1;
    }
    let mut orbits = HashMap::new();
    let mut q = VecDeque::new();
    for b in maps.keys() {
        if cnt.get(b).unwrap_or(&0) == &0 {
            orbits.insert(b, 0);
            q.push_back(b);
        }
    }
    while let Some(b) = q.pop_front() {
        let a = maps.get(b).unwrap();
        let ob = *orbits.get(b).unwrap();
        let oa = orbits.entry(a).or_insert(0);
        *oa += 1 + ob;
        let c = cnt.get_mut(a).unwrap();
        *c -= 1;
        if *c == 0 && a != &"COM" {
            q.push_back(a);
        }
    }
    dbg!(orbits.values().sum::<i32>());
    // part two
    let mut you_path = HashMap::new();
    let mut cur = "YOU";
    while cur != "COM" {
        let nxt = maps.get(cur).unwrap();
        you_path.insert(nxt, you_path.get(&cur).unwrap_or(&0) + 1);
        cur = nxt;
    }
    let mut cnt = 0;
    let mut cur = "SAN";
    while cur != "COM" {
        let nxt = maps.get(cur).unwrap();
        cnt += 1;
        if let Some(pre) = you_path.get(nxt) {
            dbg!(pre + cnt - 2);
            break;
        }
        cur = nxt;
    }
}
