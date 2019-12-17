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
    let codes = crate::common::get_input(17)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut mem = vec![0i64; 10000];
    mem[..codes.len()].clone_from_slice(&codes[..]);
    let (mut i, mut b) = (0, 0);
    let mut input = VecDeque::new();

    let mut ascii = vec![];
    while let Some(out) = step(&mut mem, &mut i, &mut b, &mut input) {
        ascii.push(out as u8);
    }

    let map_str = String::from_utf8_lossy(&ascii);
    println!("{}", map_str);

    let map: Vec<_> = map_str.trim_end().as_bytes().split(|&b| b == b'\n').collect();
    let mut alignment = 0;
    let (mut sx, mut sy) = (0, 0);
    for (y, l) in map.iter().enumerate() {
        for (x, &c) in l.iter().enumerate() {
            if c == b'#'
                && x > 0
                && y > 0
                && x + 1 < l.len()
                && y + 1 < map.len()
                && map[y - 1][x] == b'#'
                && map[y + 1][x] == b'#'
                && map[y][x - 1] == b'#'
                && map[y][x + 1] == b'#'
            {
                alignment += x * y;
            }
            if c == b'^' {
                sx = x as i32;
                sy = y as i32;
            }
        }
    }
    dbg!(alignment);

    // part two
    let mut di = 0;
    let dxdy = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let (mut x, mut y) = (sx, sy);
    let is_valid = |x, y| x >= 0 && y >= 0 && x < map[0].len() as i32 && y < map.len() as i32;
    // find the full path
    let mut full_path = String::new();
    loop {
        let ldxy = dxdy[(di + 3) % 4];
        let (lx, ly) = (x + ldxy.0, y + ldxy.1);
        let lok = is_valid(lx, ly) && map[ly as usize][lx as usize] == b'#';
        let rdxy = dxdy[(di + 1) % 4];
        let (rx, ry) = (x + rdxy.0, y + rdxy.1);
        let rok = is_valid(rx, ry) && map[ry as usize][rx as usize] == b'#';
        if lok {
            di = (di + 3) % 4;
            full_path.push_str("L,");
        } else if rok {
            di = (di + 1) % 4;
            full_path.push_str("R,");
        } else {
            break;
        }
        let mut c = 0;
        let (dx, dy) = dxdy[di];
        loop {
            let (nx, ny) = (x + dx, y + dy);
            if is_valid(nx, ny) && map[ny as usize][nx as usize] == b'#' {
                x = nx;
                y = ny;
                c += 1;
            } else {
                break;
            }
        }
        full_path.push_str(&format!("{},", c));
    }
    println!("{}", &full_path[..full_path.len() - 1]);

    // well, this is some manual work based on the full path
    let program = "A,B,A,C,B,C,A,B,A,C\nR,10,L,8,R,10,R,4\nL,6,L,6,R,10\nL,6,R,12,R,12,R,10\nn\n";
    for &c in program.as_bytes() {
        input.push_back(c as i64);
    }

    // run
    mem[..codes.len()].clone_from_slice(&codes[..]);
    mem[0] = 2;
    let (mut i, mut b) = (0, 0);
    while let Some(out) = step(&mut mem, &mut i, &mut b, &mut input) {
        if out < 256 {
            print!("{}", out as u8 as char);
        } else {
            dbg!(out);
        }
    }
}
