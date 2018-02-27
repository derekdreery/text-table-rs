//! A small library for pretty-printing tables in monospace text.
//!
//! Example
//!
//! ```
//! let data = [["Some", "printable"], ["data", "fields"]];
//! let mut out = Vec::new();
//! text_tables::render(&mut out, data).unwrap();
//! println!("{}", ::std::str::from_utf8(&out).unwrap());
//! ```

use std::io;
use std::cmp;

const CORNER_STR: &'static str = "+";
const HORIZ_BORDER_STR: &'static str = "-";
const VERT_BORDER_STR: &'static str = "|";
const SPACE_STR: &'static str = " ";
const NEW_LINE_STR: &'static str = "\n";

/// Render the table to a writer
///
/// Note that there are a lot of write calls, use a BufferedWriter if your writer is I/O for better
/// performance.
///
/// # Panics
///
/// Will panic if all rows are not the same length
pub fn render<W, T, R, C>(writer: &mut W, data: T) -> io::Result<()>
    where W: io::Write,
          T: AsRef<[R]>,
          R: AsRef<[C]>,
          C: AsRef<str>
{
    let widths = widths(&data);
    let data = data.as_ref();

    render_border_line(writer, &widths)?;
    for row in data.iter() {
        let row = row.as_ref();
        render_text_line(writer, &widths, row)?;
        render_border_line(writer, &widths)?;
    }

    Ok(())
}

// Internal helpers
// ================

/// Get the largest width of each column.
fn widths<T, R, C>(data: T) -> Vec<usize>
    where T: AsRef<[R]>,
          R: AsRef<[C]>,
          C: AsRef<str>
{
    let data = data.as_ref();
    let mut widths = vec![0; data.len()];
    // bail early if there is nothing to do
    if data.len() == 0 {
        return widths;
    }
    // this would panic without len check above
    let row_len = data[0].as_ref().len();
    for row in data.iter() {
        let row = row.as_ref();
        if row_len != row.len() {
            // todo better handle this situation
            panic!("rows must be the same length");
        }
        for (idx, cell) in row.iter().enumerate() {
            widths[idx] = cmp::max(widths[idx], cell.as_ref().len());
        }
    }
    widths
}

/// Render a border line
fn render_border_line<W: io::Write>(writer: &mut W, lengths: &[usize]) -> io::Result<()> {
    if lengths.len() == 0 || lengths[0] == 0 {
        return Ok(());
    }
    write!(writer, "{}", CORNER_STR)?;
    for len in lengths {
        for _ in 0..(*len + 2) {
            write!(writer, "{}", HORIZ_BORDER_STR)?;
        }
        write!(writer, "{}", CORNER_STR)?;
    }
    write!(writer, "\n")
}

/// Render a text line
fn render_text_line<W, C>(writer: &mut W, lengths: &[usize], row: &[C])
    -> io::Result<()>
    where W: io::Write,
          C: AsRef<str>
{
    if lengths.len() == 0 || lengths[0] == 0 {
        return Ok(());
    }
    write!(writer, "{}", VERT_BORDER_STR)?;
    for (cell, len) in row.iter().zip(lengths.iter()) {
        let cell = cell.as_ref();
        let extra = len - cell.len();
        write!(writer, "{}{}", SPACE_STR, cell)?;
        for _ in 0..extra+1 {
            write!(writer, "{}", SPACE_STR)?;
        }
        write!(writer, "{}", VERT_BORDER_STR)?;
    }
    write!(writer, "{}", NEW_LINE_STR)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn render() {
        let tables = vec![
            (vec![], &b""[..]),
            (vec![vec![]], &b""[..]),
            (vec![vec!["single", "line"], vec!["second", "lines"]],
&b"+--------+-------+
| single | line  |
+--------+-------+
| second | lines |
+--------+-------+
"[..])
        ];
        for (table, result) in tables {
            let mut out = Vec::new();
            super::render(&mut out, &table).unwrap();
            assert_eq!(out, &result[..], "{:#?}", table);
        }
    }
}
