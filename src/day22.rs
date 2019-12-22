#[derive(Debug)]
enum Cmd {
    Rev(),
    Inc(i128),
    Cut(i128),
}

fn pow(a: i128, b: i128, m: i128) -> i128 {
    let mut res = 1;
    let mut t = a;
    let mut b = b;
    while b > 0 {
        if b & 1 != 0 {
            res = (res * t) % m;
        }
        t = (t * t) % m;
        b >>= 1;
    }
    res
}

pub fn run() {
    let txt = crate::common::get_input(22).unwrap();
    let mut cmds = vec![];
    for line in txt.trim_end().split('\n') {
        if line.starts_with("deal with increment") {
            let inc = line[20..].parse().unwrap();
            cmds.push(Cmd::Inc(inc));
        } else if line.starts_with("deal into new stack") {
            cmds.push(Cmd::Rev());
        } else if line.starts_with("cut") {
            let cut = line[4..].parse().unwrap();
            cmds.push(Cmd::Cut(cut));
        } else {
            unreachable!();
        }
    }
    let cmds = cmds;

    // part one
    let n = 10007;
    let mut idx = 2019;
    for cmd in &cmds {
        match *cmd {
            Cmd::Inc(inc) => {
                idx = (idx * inc) % n;
            }
            Cmd::Rev() => {
                idx = n - 1 - idx;
            }
            Cmd::Cut(cut) => {
                idx = (idx - cut + n) % n;
            }
        }
    }
    dbg!(idx);

    // part two
    let n: i128 = 119315717514047;
    let iters: i128 = 101741582076661;
    let (mut a, mut b) = (1, 0); // 1 * x + 0
    for cmd in cmds.iter().rev() {
        match *cmd {
            Cmd::Inc(inc) => {
                let inc_inv = pow(inc, n - 2, n);
                // x = x * inc_inv
                a = (a * inc_inv) % n;
                b = (b * inc_inv) % n;
            }
            Cmd::Rev() => {
                // x = n - 1 - x
                a = (-a) % n;
                b = (n - 1 - b) % n;
            }
            Cmd::Cut(cut) => {
                // x = x + cut;
                b = (b + cut) % n;
            }
        }
    }
    let (a, b) = ((a + n) % n, (b + n) % n);

    let m = 10000000;
    let (mut a2, mut b2) = (1, 0);
    for _ in 0..m {
        a2 = (a2 * a) % n;
        b2 = (b2 * a + b) % n;
    }

    let mut idx = 2020;
    for _ in 0..iters % m {
        idx = (idx * a + b) % n;
    }
    for _ in 0..iters / m {
        idx = (idx * a2 + b2) % n;
    }
    dbg!(idx);
}
