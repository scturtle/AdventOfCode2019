pub fn run() {
    let codes = crate::common::get_input(21)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut intcode = crate::intcode::Intcode::new(10000);

    let prog = "NOT A J\nNOT B T\nOR T J\nNOT C T\nOR T J\nAND D J\nWALK\n";
    intcode.push(prog.bytes().map(|i| i as i64));
    intcode.reset(&codes);
    while let Some(c) = intcode.step() {
        if c > 255 {
            println!("{}", c);
        } else {
            print!("{}", c as u8 as char);
        }
    }

    // (NOT (A AND B AND C)) AND D AND (H or (E and (F or I)))
    let prog2 = r#"NOT J J
OR I J
OR F J
AND E J
OR H J
AND D J
NOT T T
AND A T
AND B T
AND C T
NOT T T
AND T J
RUN
"#;
    intcode.push(prog2.bytes().map(|i| i as i64));
    intcode.reset(&codes);
    while let Some(c) = intcode.step() {
        if c > 255 {
            println!("{}", c);
        } else {
            print!("{}", c as u8 as char);
        }
    }
}
