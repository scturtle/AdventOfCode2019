
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

fn step(mem: &mut [i64], i: &mut usize, b: &mut i32, input: i64) -> Option<i64> {
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
            mem[out] = input;
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
    let codes = crate::common::get_input(17)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut mem = vec![0i64; 10000];
    mem[..codes.len()].clone_from_slice(&codes[..]);
    let (mut i, mut b) = (0, 0);
    let mut one_step = |input| step(&mut mem, &mut i, &mut b, input);

    let mut ascii = vec![];
    while let Some(out) = one_step(0) {
        ascii.push(out as u8);
    }

    let map = String::from_utf8_lossy(&ascii);
    println!("{}", map);

    let m: Vec<_> = map.trim_end().as_bytes().split(|&b| b == b'\n').collect();
    let mut alignment = 0;
    for (y, l) in m.iter().enumerate() {
        for (x, &c) in l.iter().enumerate() {
            if c == b'#'
                && x > 0
                && y > 0
                && x + 1 < l.len()
                && y + 1 < m.len()
                && m[y - 1][x] == b'#'
                && m[y + 1][x] == b'#'
                && m[y][x - 1] == b'#'
                && m[y][x + 1] == b'#'
            {
                alignment += x * y;
            }
        }
    }
    dbg!(alignment);
}
