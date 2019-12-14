use std::collections::{BTreeMap, BTreeSet};

pub fn run() {
    let txt = crate::common::get_input(14).unwrap();
    let lines: Vec<&str> = txt.lines().collect();

    let mut unit: BTreeMap<&str, u64> = BTreeMap::new();
    let mut need: BTreeMap<&str, Vec<(u64, &str)>> = BTreeMap::new();
    let mut needby: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

    for line in &lines {
        let t = line.split(" => ").collect::<Vec<_>>();
        let (lhs, rh) = (t[0], t[1]);
        let fs = rh.split(' ').collect::<Vec<_>>();
        let prd = fs[1];
        unit.insert(prd, fs[0].parse().unwrap());
        for lh in lhs.split(", ") {
            let fs = lh.split(' ').collect::<Vec<_>>();
            let (cnt, raw) = (fs[0].parse().unwrap(), fs[1]);
            need.entry(prd).or_insert_with(|| vec![]).push((cnt, raw));
            needby.entry(raw).or_insert_with(|| vec![]).push(prd);
        }
    }

    let fuel2ore = |fuel: u64| -> u64 {
        let mut reqcnt = BTreeMap::new();
        let mut q = vec!["FUEL"];
        let mut done = BTreeSet::new();
        reqcnt.insert("FUEL", fuel);
        while !q.is_empty() {
            let prd = q.remove(0);
            let n = *reqcnt.get(prd).unwrap();
            let u = *unit.get(prd).unwrap();
            let multi = (n as f64 / u as f64).ceil() as u64;
            done.insert(prd);
            for (cnt, raw) in need.get(prd).unwrap() {
                *reqcnt.entry(raw).or_insert(0) += multi * cnt;
                if raw != &"ORE" && needby.get(raw).unwrap().iter().all(|p| done.contains(p)) {
                    q.push(raw);
                }
            }
        }
        *reqcnt.get("ORE").unwrap()
    };
    dbg!(fuel2ore(1));

    let (mut l, mut r) = (1e6 as u64, 1e8 as u64);
    let ore_limit = 1e12 as u64;
    while l + 1 != r {
        let m = (l + r) / 2;
        let ore = fuel2ore(m);
        if ore > ore_limit {
            r = m;
        } else {
            l = m;
        }
    }
    dbg!(l);
}
