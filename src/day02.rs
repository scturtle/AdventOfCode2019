use itertools::iproduct;

pub fn run() {
    let mut codes = crate::common::get_input(2)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let init = codes.clone();
    // part one
    codes[1] = 12;
    codes[2] = 2;
    for i in (0..codes.len()).step_by(4) {
        if codes[i] == 1 {
            let out = codes[i + 3];
            codes[out] = codes[codes[i + 1]] + codes[codes[i + 2]];
        } else if codes[i] == 2 {
            let out = codes[i + 3];
            codes[out] = codes[codes[i + 1]] * codes[codes[i + 2]];
        } else {
            assert_eq!(codes[i], 99);
            break;
        }
    }
    dbg!(&codes[0]);
    // part two
    for (noun, verb) in iproduct!(0..100, 0..100) {
        let mut codes = init.clone();
        codes[1] = noun;
        codes[2] = verb;
        let n = codes.len();
        let mut ok = false;
        for i in (0..n).step_by(4) {
            if codes[i] == 1 {
                if i + 3 >= n {
                    break;
                }
                let out = codes[i + 3];
                codes[out] = codes[codes[i + 1]] + codes[codes[i + 2]];
            } else if codes[i] == 2 {
                if i + 3 >= n {
                    break;
                }
                let out = codes[i + 3];
                codes[out] = codes[codes[i + 1]] * codes[codes[i + 2]];
            } else {
                if codes[i] == 99 {
                    ok = true;
                }
                break;
            }
        }
        if ok && codes[0] == 19690720 {
            dbg!(100 * noun + verb);
        }
    }
}
