extern crate text_tables;

use std::io;
use std::str;

fn main() {
    // can be vec or slices
    let data = [["A", "2x2"], ["pretty", "table"]];
    // we can either render to an array...
    let mut out = Vec::new();
    text_tables::render(&mut out, data).unwrap();
    println!("{}", str::from_utf8(&out).unwrap());
    // ...or we can use `Write` streams directly
    text_tables::render(&mut io::stdout(), data).unwrap();
}
