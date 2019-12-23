use std::collections::VecDeque;

struct Intcode {
    mem: Vec<i64>,
    i: usize, // pc
    base: i32,
    input: VecDeque<i64>,
}

impl Intcode {
    fn new(memsize: usize) -> Self {
        Self {
            mem: vec![0; memsize],
            i: 0,
            base: 0,
            input: VecDeque::new(),
        }
    }

    fn reset(&mut self, codes: &[i64]) {
        self.i = 0;
        self.base = 0;
        self.mem[..codes.len()].clone_from_slice(&codes[..]);
    }

    fn push<I>(&mut self, inputs: I)
    where
        I: IntoIterator<Item = i64>,
    {
        self.input.extend(inputs);
    }

    fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    fn get_addr(&self, n: u32) -> usize {
        let d = (self.mem[self.i] / 10i64.pow(n + 1)) % 10;
        let i = self.i + n as usize;
        if d == 1 {
            i
        } else if d == 2 {
            (self.base + self.mem[i] as i32) as usize
        } else {
            self.mem[i] as usize
        }
    }

    pub fn step(&mut self) -> Option<i64> {
        loop {
            let c = self.mem[self.i];
            let inst = c % 100;
            let val1 = || self.mem[self.get_addr(1)];
            let val2 = || self.mem[self.get_addr(2)];
            let out1 = || self.get_addr(1);
            let out3 = || self.get_addr(3);
            if inst == 1 {
                let out = out3();
                self.mem[out] = val1() + val2();
                self.i += 4;
            } else if inst == 2 {
                let out = out3();
                self.mem[out] = val1() * val2();
                self.i += 4;
            } else if inst == 3 {
                let out = out1();
                let input = self.input.pop_front().unwrap_or(-1);
                self.mem[out] = input;
                self.i += 2;
                if input == -1 {
                    return None;
                }
            } else if inst == 4 {
                let output = val1();
                self.i += 2;
                return Some(output);
            } else if inst == 5 {
                if val1() != 0 {
                    self.i = val2() as usize;
                } else {
                    self.i += 3;
                }
            } else if inst == 6 {
                if val1() == 0 {
                    self.i = val2() as usize;
                } else {
                    self.i += 3;
                }
            } else if inst == 7 {
                let out = out3();
                if val1() < val2() {
                    self.mem[out] = 1;
                } else {
                    self.mem[out] = 0;
                }
                self.i += 4;
            } else if inst == 8 {
                let out = out3();
                if val1() == val2() {
                    self.mem[out] = 1;
                } else {
                    self.mem[out] = 0;
                }
                self.i += 4;
            } else if inst == 9 {
                self.base += val1() as i32;
                self.i += 2;
            } else {
                assert_eq!(inst, 99);
                unreachable!();
            }
        }
    }
}

pub fn run() {
    let codes = crate::common::get_input(23)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut computers: Vec<Intcode> = (0..50)
        .map(|addr| {
            let mut intcode = Intcode::new(10000);
            intcode.reset(&codes);
            intcode.push(vec![addr as i64]);
            intcode
        })
        .collect();

    let mut last_nat_sent_y = 0;
    let mut net_pkt = (0, 0);
    let mut last_no_outs = false;
    loop {
        let mut outs = vec![];
        for c in &mut computers {
            if let Some(o1) = c.step() {
                let o2 = c.step().unwrap();
                let o3 = c.step().unwrap();
                outs.push((o1, o2, o3));
            }
        }
        for &(addr, x, y) in &outs {
            if addr == 255 {
                if net_pkt.1 == 0 {
                    dbg!(y);
                }
                net_pkt = (x, y);
            } else {
                computers[addr as usize].push(vec![x, y]);
            }
        }
        if outs.is_empty() {
            if last_no_outs && computers.iter().all(|c| c.is_empty()) {
                computers[0].push(vec![net_pkt.0, net_pkt.1]);
                if last_nat_sent_y == net_pkt.1 {
                    dbg!(last_nat_sent_y);
                    break;
                }
                last_nat_sent_y = net_pkt.1;
            }
            last_no_outs = true;
        } else {
            last_no_outs = false;
        }
    }
}
