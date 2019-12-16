pub fn run() {
    let lst: Vec<u8> = crate::common::get_input(16)
        .unwrap()
        .trim_end()
        .as_bytes()
        .iter()
        .map(|b| b - b'0')
        .collect();

    let mut pre = lst.clone();
    for _ in 0..100 {
        let mut cur = vec![0u8; pre.len()];
        for oi in 0..pre.len() {
            let mut sum: i32 = 0;
            let mut i = oi;
            let repeat = oi + 1;
            while i < cur.len() {
                for &p in pre.iter().skip(i).take(repeat) {
                    sum += p as i32;
                }
                i += repeat * 2;
                if i >= cur.len() {
                    break;
                }
                for &p in pre.iter().skip(i).take(repeat) {
                    sum -= p as i32;
                }
                i += repeat * 2;
            }
            cur[oi] = (sum.abs() % 10) as u8;
        }
        pre = cur;
    }
    let bytes = pre[..8].iter().map(|b| b + b'0').collect::<Vec<_>>();
    let ans1 = String::from_utf8_lossy(&bytes);
    dbg!(ans1);

    // part two
    let mut pre = vec![];
    for _ in 0..10_000 {
        pre.extend_from_slice(&lst);
    }
    for _ in 0..100 {
        let mut cur = vec![0u8; pre.len()];
        for oi in (5_900_000..pre.len()).rev() {
            if oi + 1 == pre.len() {
                cur[oi] = pre[oi];
            } else {
                cur[oi] = (cur[oi + 1] + pre[oi]) % 10;
            }
        }
        pre = cur;
    }
    let offset = String::from_utf8_lossy(&lst[..7].iter().map(|b| b + b'0').collect::<Vec<_>>())
        .parse::<usize>()
        .unwrap();
    let bytes = pre[offset..offset + 8]
        .iter()
        .map(|b| b + b'0')
        .collect::<Vec<_>>();
    let ans2 = String::from_utf8_lossy(&bytes);
    dbg!(ans2);
}
