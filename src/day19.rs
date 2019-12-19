use itertools::iproduct;
use std::collections::VecDeque;

fn get_addr(mem: &[i64], base: i32, i: usize, n: u32) -> usize {
    let d = (mem[i] / 10i64.pow(n + 1)) % 10;
    let i = i + n as usize;
    if d == 1 {
        i
    } else if d == 2 {
        (base + mem[i] as i32) as usize
    } else {
        mem[i] as usize
    }
}

fn step(mem: &mut [i64], i: &mut usize, b: &mut i32, input: &mut VecDeque<i64>) -> Option<i64> {
    loop {
        let c = mem[*i];
        let inst = c % 100;
        let val1 = || mem[get_addr(&mem, *b, *i, 1)];
        let val2 = || mem[get_addr(&mem, *b, *i, 2)];
        let out1 = || get_addr(&mem, *b, *i, 1);
        let out3 = || get_addr(&mem, *b, *i, 3);
        if inst == 1 {
            let out = out3();
            mem[out] = val1() + val2();
            *i += 4;
        } else if inst == 2 {
            let out = out3();
            mem[out] = val1() * val2();
            *i += 4;
        } else if inst == 3 {
            let out = out1();
            mem[out] = input.pop_front().unwrap();
            *i += 2;
        } else if inst == 4 {
            let output = val1();
            *i += 2;
            return Some(output);
        } else if inst == 5 {
            if val1() != 0 {
                *i = val2() as usize;
            } else {
                *i += 3;
            }
        } else if inst == 6 {
            if val1() == 0 {
                *i = val2() as usize;
            } else {
                *i += 3;
            }
        } else if inst == 7 {
            let out = out3();
            if val1() < val2() {
                mem[out] = 1;
            } else {
                mem[out] = 0;
            }
            *i += 4;
        } else if inst == 8 {
            let out = out3();
            if val1() == val2() {
                mem[out] = 1;
            } else {
                mem[out] = 0;
            }
            *i += 4;
        } else if inst == 9 {
            *b += val1() as i32;
            *i += 2;
        } else {
            assert_eq!(inst, 99);
            return None;
        }
    }
}

pub fn run() {
    let codes = crate::common::get_input(19)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut mem = vec![0i64; 10000];
    let mut input = VecDeque::new();
    let mut get = |x, y| {
        input.push_back(x);
        input.push_back(y);
        mem[..codes.len()].clone_from_slice(&codes[..]);
        let (mut i, mut b) = (0, 0);
        step(&mut mem, &mut i, &mut b, &mut input).unwrap()
    };

    // part one
    let affected: i64 = iproduct!(0..50, 0..50).map(|(x, y)| get(x, y)).sum();
    dbg!(affected);

    // part two
    let (mut miny, mut maxy) = (1000, 2000);
    let mut res = 0;
    // miny/maxy is offseted by 1 to find the result of the boundary
    while miny + 1 != maxy {
        let ty = (miny - 1 + maxy - 1) / 2;
        // find the left and right x boundary
        let mut lx = 0;
        while get(lx, ty) == 0 {
            lx += 1;
        }
        let mut rx = lx;
        while get(rx, ty) == 1 {
            rx += 1;
        }
        let mut ok = rx - lx >= 100;
        if ok {
            // step left 100 from the right x boundary
            lx = rx - 100;
            let mut by = ty;
            while (get(lx, by)) == 1 {
                by += 1;
            }
            // ensure the bellow 100 y are ok
            ok = by - ty >= 100;
        }
        if !ok {
            miny = ty + 1;
        } else {
            maxy = ty + 1;
            res = lx * 10000 + ty;
        }
    }
    dbg!(res);
}
