use itertools::Itertools;

fn more(mass: i32) -> i32 {
    if mass <= 6 {
        0
    } else {
        let fuel = mass / 3 - 2;
        fuel + more(fuel)
    }
}

pub fn run() {
    let nums = crate::common::get_input(1)
        .unwrap()
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec();
    let fuel = nums.iter().map(|mass| mass / 3 - 2).sum::<i32>();
    dbg!(fuel);
    let fuel = nums.iter().map(|mass| more(*mass)).sum::<i32>();
    dbg!(fuel);
}
