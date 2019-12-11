pub fn run() {
    let txt = crate::common::get_input(8).unwrap();
    let bytes = txt.trim_end().as_bytes();
    let (h, w) = (6, 25);
    let mut layers = vec![];
    for i in (0..bytes.len()).step_by(h * w) {
        layers.push(&bytes[i..i + h * w]);
    }
    let l = layers
        .iter()
        .min_by_key(|&l| l.iter().filter(|&&c| c == b'0').count())
        .unwrap();
    let cnt1 = l.iter().filter(|&&c| c == b'1').count();
    let cnt2 = l.iter().filter(|&&c| c == b'2').count();
    dbg!(cnt1 * cnt2);
    // part two
    for i in 0..h {
        let mut line = vec![];
        for j in 0..w {
            let idx = i * w + j;
            let mut c = b'2';
            for l in &layers {
                if l[idx] != b'2' {
                    c = l[idx];
                    break;
                }
            }
            let c = if c == b'2' {
                b'?'
            } else if c == b'0' {
                b' '
            } else {
                b'#'
            };
            line.push(c);
        }
        println!("{}", String::from_utf8_lossy(&line));
    }
}
