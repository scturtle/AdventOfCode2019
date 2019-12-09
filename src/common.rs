use isahc::prelude::*;

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

pub fn get_input(day: i8) -> std::io::Result<String> {
    dotenv::dotenv().expect(".env");
    let session = std::env::var("SESSION").expect("env SESSION");
    let filename = format!("{}.txt", day);
    let path = Path::new(&filename);
    if path.exists() {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        return Ok(s);
    }
    let day_url = format!("https://adventofcode.com/2019/day/{}", day);
    let s = Request::get(day_url.to_owned() + "/input")
        .header("referer", &day_url)
        .header("cookie", "session=".to_owned() + &session)
        .body(())
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
        .send()?
        .text()?;
    File::create(path)?.write_all(s.as_bytes())?;
    Ok(s)
}
