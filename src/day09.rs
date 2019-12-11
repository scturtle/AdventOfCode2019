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

fn boost(codes: &[i64], input: i64) -> i64 {
    let mut base = 0;
    let mut mem = vec![0i64; 30000];
    mem[..codes.len()].clone_from_slice(&codes[..]); // memcpy
    let mut i = 0;
    loop {
        let c = mem[i];
        let inst = c % 100;
        let val1 = || mem[get_addr(&mem, base, i, 1)];
        let val2 = || mem[get_addr(&mem, base, i, 2)];
        let out1 = || get_addr(&mem, base, i, 1);
        let out3 = || get_addr(&mem, base, i, 3);
        if inst == 1 {
            let out = out3();
            mem[out] = val1() + val2();
            i += 4;
        } else if inst == 2 {
            let out = out3();
            mem[out] = val1() * val2();
            i += 4;
        } else if inst == 3 {
            let out = out1();
            mem[out] = input;
            i += 2;
        } else if inst == 4 {
            return val1(); // i += 2;
        } else if inst == 5 {
            if val1() != 0 {
                i = val2() as usize;
            } else {
                i += 3;
            }
        } else if inst == 6 {
            if val1() == 0 {
                i = val2() as usize;
            } else {
                i += 3;
            }
        } else if inst == 7 {
            let out = out3();
            if val1() < val2() {
                mem[out] = 1;
            } else {
                mem[out] = 0;
            }
            i += 4;
        } else if inst == 8 {
            let out = out3();
            if val1() == val2() {
                mem[out] = 1;
            } else {
                mem[out] = 0;
            }
            i += 4;
        } else if inst == 9 {
            base += val1() as i32;
            i += 2;
        } else {
            unreachable!();
        }
    }
}

pub fn run() {
    let codes = crate::common::get_input(9)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let out1 = boost(&codes, 1);
    dbg!(out1);
    let out2 = boost(&codes, 2);
    dbg!(out2);
}
