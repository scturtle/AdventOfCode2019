use rand::{Rng, SeedableRng};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

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

#[allow(dead_code)]
pub fn draw_canvas(canvas: &BTreeMap<(i32, i32), i32>) {
    let (mut minx, mut miny) = (std::i32::MAX, std::i32::MAX);
    let (mut maxx, mut maxy) = (std::i32::MIN, std::i32::MIN);
    for &(kx, ky) in canvas.keys() {
        minx = minx.min(kx);
        miny = miny.min(ky);
        maxx = maxx.max(kx);
        maxy = maxy.max(ky);
    }
    let mut lines = vec![vec![b' '; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];
    for (&(kx, ky), &c) in canvas {
        lines[(ky - miny) as usize][(kx - minx) as usize] = {
            match c {
                0 => b'#',
                1 => b'.',
                2 => b'D',
                _ => unreachable!(),
            }
        }
    }
    for line in &lines {
        println!("{}", String::from_utf8_lossy(&line));
    }
}

pub fn run() {
    let codes = crate::common::get_input(15)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut mem = vec![0i64; 10000];
    mem[..codes.len()].clone_from_slice(&codes[..]);
    let (mut i, mut b) = (0, 0);
    let mut one_step = |input| step(&mut mem, &mut i, &mut b, input);

    let next = [(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut rng = rand::rngs::StdRng::from_seed([42; 32]);
    let mut canvas = BTreeMap::new();
    let mut xy = (0, 0);
    let mut goal = (0, 0);

    for _ in 0..1_000_000 {
        // random move
        let movement = (rng.gen::<u8>() % 4) + 1;
        let nxy = next[movement as usize];
        let nxy = (xy.0 + nxy.0, xy.1 + nxy.1);
        // avoid wall
        if canvas.get(&nxy) == Some(&0) {
            continue;
        }
        match one_step(movement as i64).unwrap() {
            0 => {
                canvas.insert(nxy, 0); // wall
            }
            1 => {
                canvas.insert(xy, 1); // space
                xy = nxy;
                // canvas.insert(xy, 2); // droid
            }
            2 => {
                canvas.insert(xy, 1); // space
                xy = nxy;
                // canvas.insert(xy, 2); // droid
                goal = xy;
            }
            _ => {
                panic!("bad out");
            }
        }
        // draw_canvas(&canvas);
        // println!();
    }
    draw_canvas(&canvas);
    dbg!(goal);

    let mut saw = BTreeSet::new();
    let mut q = VecDeque::new();
    q.push_back((0, 0, 0));
    saw.insert((0, 0));
    while !q.is_empty() {
        let (x, y, step) = q.pop_front().unwrap();
        if (x, y) == goal {
            dbg!(step);
            break;
        }
        for (nx, ny) in &next[1..] {
            let nxy = (x + nx, y + ny);
            if saw.contains(&nxy) || canvas.get(&nxy).unwrap_or(&0) == &0 {
                continue;
            }
            q.push_back((nxy.0, nxy.1, step + 1));
            saw.insert(nxy);
        }
    }

    // part two
    let mut saw = BTreeSet::new();
    let mut q = VecDeque::new();
    let mut minutes = 0;
    q.push_back((goal.0, goal.1, 0));
    saw.insert((goal.0, goal.1));
    while !q.is_empty() {
        let (x, y, step) = q.pop_front().unwrap();
        for (nx, ny) in &next[1..] {
            let nxy = (x + nx, y + ny);
            if saw.contains(&nxy) || canvas.get(&nxy).unwrap_or(&0) == &0 {
                continue;
            }
            q.push_back((nxy.0, nxy.1, step + 1));
            minutes = minutes.max(step + 1);
            saw.insert(nxy);
        }
    }
    dbg!(minutes);
}
