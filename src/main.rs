use std::{fs::File, io::{self, BufRead, BufReader}, path::PathBuf};

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let path = PathBuf::from(path);
        if path.is_dir() {
            println!("total: {}", count_dir(&path));
        } else {
            let lines = count_lines(&path).expect("count_lines failed");
            println!("{} lines of code in {:?}", lines, path);
        }
    } else {
       println!("total: {}", count_dir(&PathBuf::from(".")));
    }
}

fn count_lines(path: &PathBuf) -> Result<usize, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut count = 0;
    for (i, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if line.trim().is_empty() || line.trim().starts_with("//") {
                continue;
            }
            count += 1;
        }
    }
    Ok(count)
}

fn count_dir (path: &PathBuf) -> usize {
    let mut count = 0;
    for entry in path.read_dir().expect("read_dir call failed") {
        let entry = entry.expect("entry failed");
        let path = entry.path();
        if path.is_dir() {
            count_dir(&path);
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            let lines = count_lines(&path).expect("count_lines failed");
            println!("{} lines of code in {:?}", lines, path);
            count += lines;
        }
    }
    count
}