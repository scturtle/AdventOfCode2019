use std::cmp::Ordering;

fn parse(line: &str) -> Vec<i32> {
    line.replace("<x=", "")
        .replace(" y=", "")
        .replace(" z=", "")
        .replace(">", "")
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

pub fn run() {
    let txt = crate::common::get_input(12).unwrap();
    let lines: Vec<&str> = txt.lines().collect();
    let mut poses: Vec<_> = lines.iter().map(|l| parse(l)).collect();
    let ori_poses = poses.clone();
    let mut vels = [[0; 3]; 4];
    for _step in 0..1000 {
        for i in 0..3 {
            for j in i + 1..4 {
                for k in 0..3 {
                    match poses[i][k].cmp(&poses[j][k]) {
                        Ordering::Less => {
                            vels[i][k] += 1;
                            vels[j][k] -= 1;
                        }
                        Ordering::Greater => {
                            vels[i][k] -= 1;
                            vels[j][k] += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
        for i in 0..4 {
            for k in 0..3 {
                poses[i][k] += vels[i][k];
            }
        }
    }
    let energy = poses
        .iter()
        .zip(vels.iter())
        .map(|(p, v)| {
            p.iter().map(|i| i.abs()).sum::<i32>() * v.iter().map(|i| i.abs()).sum::<i32>()
        })
        .sum::<i32>();
    dbg!(energy);

    // part two
    fn period(p0: &[i32], v0: &[i32]) -> u64 {
        let mut p = p0.to_owned();
        let mut v = v0.to_owned();
        let mut step = 0;
        loop {
            for i in 0..3 {
                for j in i + 1..4 {
                    match p[i].cmp(&p[j]) {
                        Ordering::Less => {
                            v[i] += 1;
                            v[j] -= 1;
                        }
                        Ordering::Greater => {
                            v[i] -= 1;
                            v[j] += 1;
                        }
                        _ => {}
                    }
                }
            }
            for i in 0..4 {
                p[i] += v[i];
            }
            step += 1;
            if p.as_slice() == p0 && v.as_slice() == v0 {
                return step;
            }
        }
    }
    fn lcm(a: u64, b: u64) -> u64 {
        fn gcd(a: u64, b: u64) -> u64 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        a * b / gcd(a, b)
    }
    let mut periods = [0; 3];
    for k in 0..3 {
        let p0: Vec<_> = ori_poses.iter().map(|i| i[k]).collect();
        let v0 = vec![0; 4];
        periods[k] = period(&p0, &v0);
    }
    let steps = lcm(lcm(periods[0], periods[1]), periods[2]);
    dbg!(steps);
}
