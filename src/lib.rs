use std::io;

use std::io::BufRead;

use std::io::BufWriter;
use std::io::Write;

pub const MIN_LEN_INCLUSIVE_DEFAULT: usize = 0;

pub fn long_lines2writer<I, W>(
    lines: I,
    mut wtr: W,
    min_len_inclusive: usize,
) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Vec<u8>, io::Error>>,
    W: Write,
{
    for rline in lines {
        let line: Vec<u8> = rline?;
        let len: usize = line.len();
        if min_len_inclusive <= len {
            wtr.write_all(&line)?;
            wtr.write_all(b"\n")?;
        }
    }
    wtr.flush()
}

pub fn stdin2stdout(min_len_inclusive: usize) -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let lines = il.split(b'\n');

    let o = io::stdout();
    let mut ol = o.lock();

    let bw = BufWriter::new(&mut ol);
    long_lines2writer(lines, bw, min_len_inclusive)?;

    ol.flush()
}
