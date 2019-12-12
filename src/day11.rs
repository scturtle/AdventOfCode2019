use std::collections::BTreeMap;

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
    let codes = crate::common::get_input(11)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut mem = vec![0i64; 3000];
    mem[..codes.len()].clone_from_slice(&codes[..]);
    let (mut i, mut b) = (0, 0);
    let mut one_step = |input| step(&mut mem, &mut i, &mut b, input);

    let mut canvas = BTreeMap::new();
    let mut dir: usize = 0; // 0: up, 1: right, 2: down, 3: left
    let (mut x, mut y) = (0, 0); // y axis to down, x axis to right
    let ds = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    canvas.insert((x, y), 1); // NOTE: for part two, start with white panel
    loop {
        let &input = canvas.get(&(x, y)).unwrap_or(&0); // default to black
        let output = one_step(input);
        if let Some(c) = output {
            assert!(c == 0 || c == 1);
            canvas.insert((x, y), c);
        } else {
            break;
        }
        let output = one_step(input);
        if let Some(d) = output {
            assert!(d == 0 || d == 1);
            if d == 0 {
                dir = (dir + 3) % 4;
            } else {
                dir = (dir + 1) % 4;
            }
            x += ds[dir].0;
            y += ds[dir].1;
        } else {
            break;
        }
    }
    dbg!(canvas.len());
    // part two
    let (mut minx, mut miny) = (std::i32::MAX, std::i32::MAX);
    let (mut maxx, mut maxy) = (std::i32::MIN, std::i32::MIN);
    for &(kx, ky) in canvas.keys() {
        minx = minx.min(kx);
        miny = miny.min(ky);
        maxx = maxx.max(kx);
        maxy = maxy.max(ky);
    }
    let mut lines = vec![vec![b' '; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];
    for (&(kx, ky), &c) in &canvas {
        lines[(ky - miny) as usize][(kx - minx) as usize] = if c == 0 { b' ' } else { b'#' };
    }
    for line in &lines {
        println!("{}", String::from_utf8_lossy(&line));
    }
}
