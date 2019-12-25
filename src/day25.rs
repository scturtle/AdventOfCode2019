use crate::intcode::Intcode;

pub fn run() {
    let codes = crate::common::get_input(25)
        .unwrap()
        .trim_end()
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // manual play
    if false {
        let mut intcode = Intcode::new(10000);
        intcode.reset(&codes);
        let mut out = String::new();
        while let Some(o) = intcode.step() {
            let o = o as u8 as char;
            out.push(o);
            if o != '\n' {
                continue;
            }
            print!("{}", out);
            if out == "Command?\n" {
                use std::io::BufRead;
                let mut line = String::new();
                std::io::stdin().lock().read_line(&mut line).unwrap();
                intcode.push(line.chars().map(|c| c as i64));
            }
            out.clear();
        }
    }

    let pre_inputs_txt = r#"north
take mouse
north
take pointer
south
south
west
take monolith
north
west
take food ration
south
take space law space brochure
north
east
south
south
take sand
south
west
take asterisk
south
take mutex
north
east
north
north
east
south
south
west
south
"#; // then east to evaluate

    let items = [
        "pointer",
        "mutex",
        "asterisk",
        "space law space brochure",
        "monolith",
        "mouse",
        "food ration",
        "sand",
    ];

    // try all possible combinations
    for sel in 0..1 << items.len() {
        let mut intcode = Intcode::new(10000);
        intcode.reset(&codes);

        // collect all items and drop some
        let mut inputs: Vec<_> = pre_inputs_txt.lines().map(|s| s.to_owned()).collect();
        for (i, item) in items.iter().enumerate() {
            if (1 << i) & sel == 0 {
                inputs.push(format!("drop {}", item));
            }
        }
        inputs.push("east\n".to_owned());

        let mut out = String::new();
        while let Some(o) = intcode.step() {
            let o = o as u8 as char;
            out.push(o);
            if o != '\n' {
                continue;
            }
            if out.starts_with("\"Oh, hello!") {
                print!("{}", out);
                std::process::exit(0);
            }
            if out == "Command?\n" {
                if !inputs.is_empty() {
                    let line = inputs.remove(0);
                    intcode.push(line.chars().map(|c| c as i64));
                    intcode.push(vec!['\n' as i64]);
                } else {
                    break;
                }
            }
            out.clear();
        }
    }
}
