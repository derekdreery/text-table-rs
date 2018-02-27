# Text-table

This library provides very simple table printing using text characters. It has
no dependencies besides std. I'm interested in making it no_std if this is
possible, contributions welcome!

Message me if you want the license changed.

## Example

```rust
extern crate text_table;

use std::str;
use std::io;

fn main() {
    // can be vec or slices
    let data = [["A", "2x2"], ["pretty", "table"]];
    // we can either render to an array...
    let mut out = Vec::new();
    text_table::render(&mut out, data).unwrap();
    println!("{}", str::from_utf8(&out).unwrap());
    // ...or we can use `Write` streams directly
    text_table::render(&mut io::stdout(), data).unwrap();
}
```
