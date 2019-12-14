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

#[allow(dead_code)]
pub fn draw_canvas(canvas: &BTreeMap<(i32, i32), i32>) {
    let (mut maxx, mut maxy) = (std::i32::MIN, std::i32::MIN);
    for &(kx, ky) in canvas.keys() {
        maxx = maxx.max(kx);
        maxy = maxy.max(ky);
    }
    let mut lines = vec![vec![b' '; (maxx + 1) as usize]; (maxy + 1) as usize];
    for (&(kx, ky), &c) in canvas {
        lines[ky as usize][kx as usize] = {
            match c {
                0 => b' ',
                1 => b'#',
                2 => b'=',
                3 => b'_',
                4 => b'.',
                _ => unreachable!(),
            }
        }
    }
    for line in &lines {
        println!("{}", String::from_utf8_lossy(&line));
    }
}

pub fn run() {
    let codes = crate::common::get_input(13)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut mem = vec![0i64; 10000];
    mem[..codes.len()].clone_from_slice(&codes[..]);
    let (mut i, mut b) = (0, 0);

    mem[0] = 2; // play for part two

    let mut one_step = |input| step(&mut mem, &mut i, &mut b, input);
    let mut canvas = BTreeMap::new();
    let mut score = 0;
    let mut joystick = 0;
    let mut last_ball = (0, 0);
    let mut ball = (0, 0);
    let mut paddle = (0, 0);
    let mut changed = false;
    loop {
        let (x, y, tile);
        if let Some(out) = one_step(joystick) {
            x = out as i32;
        } else {
            break;
        }
        if let Some(out) = one_step(joystick) {
            y = out as i32;
        } else {
            break;
        }
        if let Some(out) = one_step(joystick) {
            tile = out as i32;
        } else {
            break;
        }
        if x == -1 && y == 0 {
            score = tile;
        } else {
            canvas.insert((x, y), tile);
            if tile == 3 {
                paddle = (x, y);
                changed = true;
            } else if tile == 4 {
                last_ball = ball;
                ball = (x, y);
                changed = true;
            }
            if canvas.len() == 24 * 40 && changed {
                changed = false;
                // draw_canvas(&canvas);
                // println!("ball: {:?} paddle: {:?}", ball, paddle);
                // println!("joystick: {} score: {}", joystick, score);
                // std::thread::sleep(std::time::Duration::from_millis(100));
                if ball.1 == 21 && ball.0 == paddle.0 {
                    // about to hit paddle, sit still
                    joystick = 0;
                } else if paddle.0 < ball.0 {
                    joystick = 1;
                } else if paddle.0 > ball.0 {
                    joystick = -1;
                } else {
                    // with same x, track by history
                    if last_ball.0 < ball.0 {
                        joystick = 1;
                    } else {
                        joystick = -1;
                    }
                }
            }
        }
    }
    dbg!(score);
}
