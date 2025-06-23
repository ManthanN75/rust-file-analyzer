use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct FileStats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}
pub fn analyze_file(path: &str) -> io::Result<FileStats> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    for line in reader.lines() {
        let line = line?;
        lines += 1;
        words += line.split_whitespace().count();
        chars += line.chars().count();
    }

    Ok(FileStats {
        lines,
        words,
        chars,
    })
}
