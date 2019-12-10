fn is_ok1(v: &[u8]) -> bool {
    let mut double = false;
    for i in 1..v.len() {
        if v[i] < v[i - 1] {
            return false;
        }
        if v[i] == v[i - 1] {
            double = true;
        }
    }
    double
}

fn is_ok2(v: &[u8]) -> bool {
    let mut double = false;
    let mut cont = 1;
    for i in 1..v.len() {
        if v[i] < v[i - 1] {
            return false;
        }
        if v[i] == v[i - 1] {
            cont += 1;
        } else {
            if cont == 2 {
                double = true;
            }
            cont = 1;
        }
    }
    double || cont == 2
}

pub fn run() {
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 350_000..800_000 {
        let s = i
            .to_string()
            .as_bytes()
            .iter()
            .map(|i| i - b'0')
            .collect::<Vec<u8>>();
        ans1 += is_ok1(&s) as i32;
        ans2 += is_ok2(&s) as i32;
    }
    dbg!(ans1);
    dbg!(ans2);
}
