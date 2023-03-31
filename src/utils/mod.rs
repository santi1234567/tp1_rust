use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_file(file_name: impl AsRef<Path>) -> Result<Vec<String>, String> {
    let file = File::open(file_name);
    if let Err(e) = file {
        return Err(format!("ERROR: {}", e));
    }
    let file = file.unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let lines = match read_file("tables/no_white.txt") {
            Ok(file_contents) => file_contents,
            Err(e) => {
                println!("{}", e);
                vec![]
            }
        };
        assert!(!lines.is_empty());
        assert!(lines.len() == 8);
    }
}
