use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn adc06() -> io::Result<()> {
    let path = Path::new("adc06.txt");

    { // Part 01
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut problems: Vec<Vec<String>> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

            for i in 0..numbers.len() {
                if problems.len() <= i {
                    problems.push(Vec::new());
                }
                problems[i].push(numbers[i].clone());
            }
        }

        for problem in &problems {
            println!("{:?}", problem);
        }

        let mut sum = 0;
        for problem in &problems {
            println!("{:?}", problem);
            let operator = &problem[problem.len() - 1];

            let mut result: i64 = problem[0].parse::<i64>().unwrap();
            for i in 1..problem.len() - 1 {
                let no = problem[i].parse::<i64>().unwrap();
                result = match operator.as_str() {
                    "+" => result + no,
                    "*" => result * no,
                    _ => result,
                };
            }
            sum += result;
            println!("line Result: {}", result);
        }

        println!("sum: {}", sum);
    }

    Ok(())
}