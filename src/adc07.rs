use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn adc07() -> io::Result<()> {
    let path = Path::new("adc06.txt");

    { // Part 01
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut problems: Vec<Vec<String>> = Vec::new();
        for line in reader.lines() {
            let line = line?;

        }

    }

    Ok(())
}