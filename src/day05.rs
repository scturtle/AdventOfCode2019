pub fn run() {
    let mut codes = crate::common::get_input(5)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    // let input = 1; // for part one
    let input = 5;
    let mut i = 0;
    loop {
        let c = codes[i];
        let inst = c % 100;
        let imme1 = (c / 100) % 10 == 1;
        let imme2 = (c / 1000) % 10 == 1;
        let val1 = if i + 1 >= codes.len() {
            0
        } else if imme1 {
            codes[i + 1]
        } else {
            let j = codes[i + 1] as usize;
            if j >= codes.len() {
                0
            } else {
                codes[j]
            }
        };
        let val2 = if i + 2 >= codes.len() {
            0
        } else if imme2 {
            codes[i + 2]
        } else {
            let j = codes[i + 2] as usize;
            if j >= codes.len() {
                0
            } else {
                codes[j]
            }
        };
        let out = if i + 3 >= codes.len() {
            0
        } else {
            codes[i + 3] as usize
        };
        // 5, 6, 7, 8 are for part two
        if inst == 1 {
            codes[out] = val1 + val2;
            i += 4;
        } else if inst == 2 {
            codes[out] = val1 * val2;
            i += 4;
        } else if inst == 3 {
            let out = codes[i + 1] as usize;
            codes[out] = input;
            i += 2;
        } else if inst == 4 {
            dbg!(val1);
            i += 2;
        } else if inst == 5 {
            if val1 != 0 {
                i = val2 as usize;
            } else {
                i += 3;
            }
        } else if inst == 6 {
            if val1 == 0 {
                i = val2 as usize;
            } else {
                i += 3;
            }
        } else if inst == 7 {
            codes[out] = if val1 < val2 { 1 } else { 0 };
            i += 4;
        } else if inst == 8 {
            codes[out] = if val1 == val2 { 1 } else { 0 };
            i += 4;
        } else {
            assert_eq!(codes[i], 99);
            break;
        }
    }
}
