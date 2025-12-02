use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn adc01() -> io::Result<()> {
    // Path to the input file
    let path = Path::new("adc01.txt");

    // Open the file
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    {
        let mut result: i64 = 50;
        let mut zero_counter = 0;
        for line in reader.lines() {
            let line = line?;

            let first_char = line.chars().next().unwrap();
            let rest = &line[first_char.len_utf8()..];

            let mut sign = 0;
            match first_char {
                'L' => sign = -1,
                'R' => sign = 1,
                _ => println!("Unknown direction: {}", first_char),
            }

            let steps = rest.parse::<i64>().unwrap();
            result += (sign * steps) % 100;
            if result < 0 {
                result += 100;
            }
            if result >= 100 {
                result -= 100;
            }

            if result == 0 {
                zero_counter += 1;
            }

            // println!("Processing line: {} -> {} ({})", line, result, zero_counter);
        }
        println!("Finished processing adc01.txt Part 1: {}", zero_counter);
    }

    {
        let mut zero_counter = 0;
        let mut result = 50;
        // Reopen the file for the second pass
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;

            let first_char = line.chars().next().unwrap();
            let rest = &line[first_char.len_utf8()..];

            let mut sign = 0;
            match first_char {
                'L' => sign = -1,
                'R' => sign = 1,
                _ => println!("Unknown direction: {}", first_char),
            }

            let steps = rest.parse::<i64>().unwrap();

            for _ in 0..steps {
                result = (result + sign) % 100;
                if result == 0 {
                    zero_counter += 1;
                }
            }
            // result = result % 100;
            // if result < 0 {
            //     result += 100;
            // }
            // if result >= 100 {
            //     result -= 100;
            // }
            //
            // if result == 0 {
            //     zero_counter += 1;
            // }

            println!("Processing line Part 2: {} -> {} ({})", line, result, zero_counter);
        }
        println!("Finished processing adc01.txt Part 2: {}", zero_counter);
    }

    Ok(())
}