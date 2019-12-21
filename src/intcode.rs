use std::collections::VecDeque;

pub struct Intcode {
    mem: Vec<i64>,
    i: usize, // pc
    base: i32,
    input: VecDeque<i64>,
}

impl Intcode {
    pub fn new(memsize: usize) -> Self {
        Self {
            mem: vec![0; memsize],
            i: 0,
            base: 0,
            input: VecDeque::new(),
        }
    }

    pub fn reset(&mut self, codes: &[i64]) {
        self.i = 0;
        self.base = 0;
        self.mem[..codes.len()].clone_from_slice(&codes[..]);
    }

    pub fn push<I>(&mut self, inputs: I)
    where
        I: IntoIterator<Item = i64>,
    {
        self.input.extend(inputs);
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
                self.mem[out] = self.input.pop_front().unwrap();
                self.i += 2;
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
                return None;
            }
        }
    }
}
