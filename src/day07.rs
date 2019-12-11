use itertools::Itertools;
use std::collections::VecDeque;

fn amp(i: &mut usize, codes: &mut Vec<i32>, inputs: &mut VecDeque<i32>) -> Option<i32> {
    loop {
        let c = codes[*i];
        let inst = c % 100;
        let imme1 = (c / 100) % 10 == 1;
        let imme2 = (c / 1000) % 10 == 1;
        let val1 = if *i + 1 >= codes.len() {
            0
        } else if imme1 {
            codes[*i + 1]
        } else {
            let j = codes[*i + 1] as usize;
            if j >= codes.len() {
                0
            } else {
                codes[j]
            }
        };
        let val2 = if *i + 2 >= codes.len() {
            0
        } else if imme2 {
            codes[*i + 2]
        } else {
            let j = codes[*i + 2] as usize;
            if j >= codes.len() {
                0
            } else {
                codes[j]
            }
        };
        let out = if *i + 3 >= codes.len() {
            0
        } else {
            codes[*i + 3] as usize
        };
        if inst == 1 {
            codes[out] = val1 + val2;
            *i += 4;
        } else if inst == 2 {
            codes[out] = val1 * val2;
            *i += 4;
        } else if inst == 3 {
            let out = codes[*i + 1] as usize;
            codes[out] = inputs.pop_front().expect("input");
            *i += 2;
        } else if inst == 4 {
            *i += 2;
            return Some(val1);
        } else if inst == 5 {
            if val1 != 0 {
                *i = val2 as usize;
            } else {
                *i += 3;
            }
        } else if inst == 6 {
            if val1 == 0 {
                *i = val2 as usize;
            } else {
                *i += 3;
            }
        } else if inst == 7 {
            codes[out] = if val1 < val2 { 1 } else { 0 };
            *i += 4;
        } else if inst == 8 {
            codes[out] = if val1 == val2 { 1 } else { 0 };
            *i += 4;
        } else {
            assert_eq!(codes[*i], 99);
            return None;
        }
    }
}

pub fn run() {
    let codes = crate::common::get_input(7)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // part one
    let mut maxout = std::i32::MIN;
    for phases in (0..5).permutations(5) {
        let mut codes = codes.clone();
        let mut out = 0;
        let mut inputs = VecDeque::new();
        for i in 0..5 {
            inputs.extend(vec![phases[i], out]);
            out = amp(&mut 0, &mut codes, &mut inputs).unwrap();
        }
        maxout = maxout.max(out);
    }
    dbg!(maxout);

    // part two
    let mut maxout = std::i32::MIN;
    for phases in (5..10).permutations(5) {
        let mut is = vec![0; 5];
        let mut states = vec![codes.clone(); 5];
        let mut inputs = vec![VecDeque::new(); 5];
        for i in 0..5 {
            inputs[i].push_back(phases[i]);
        }
        inputs[0].push_back(0);
        loop {
            for i in 0..5 {
                if let Some(out) = amp(&mut is[i], &mut states[i], &mut inputs[i]) {
                    inputs[(i + 1) % 5].push_back(out);
                }
            }
            // test if the last one is stopped
            if states[4][is[4]] == 99 {
                let lastout = inputs[0].pop_front().unwrap();
                maxout = maxout.max(lastout);
                break;
            }
        }
    }
    dbg!(maxout);
}
