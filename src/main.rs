mod stats;

use std::fs;
use rayon::prelude::*;
use stats::analyze_file;

fn main() {
    let folder = "./data";

    let file_paths: Vec<_> = fs::read_dir(folder)
        .expect("Failed to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("txt"))
        .collect();

    // Analyze in parallel
    file_paths.par_iter().for_each(|path| {
        let path_str = path.to_str().unwrap_or_default();
        match analyze_file(path_str) {
            Ok(stats) => {
                println!(
                    "📄 {} → Lines: {}, Words: {}, Chars: {}",
                    path_str, stats.lines, stats.words, stats.chars
                );
            }
            Err(e) => {
                println!("❌ Error reading {}: {}", path_str, e);
            }
        }
    });
}

