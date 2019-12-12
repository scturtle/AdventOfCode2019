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
    println!("energy: {}", energy);
}
