use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn adc03() -> io::Result<()> {
    let path = Path::new("adc03.txt");

    { // Part 01
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut sum = 0;
        for line in reader.lines() {
            let line = line?;

            // convert every character of line into a number vector
            let mut numbers = Vec::new();
            for c in line.chars() {
                numbers.push(c.to_digit(10).unwrap() as i64);
            }

            print!("{:?}", numbers);

            // try out combinations
            let mut max = 0;
            for i in 0..numbers.len() - 1 {
                for j in i + 1..numbers.len() {
                    let no = numbers[i] * 10 + numbers[j];
                    if no > max {
                        max = no;
                    }
                }
            }
            sum += max;
            println!(" - max: {}", max);
        }
        println!("sum: {}", sum);
    }

    { // Part 02
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut sum = 0;
        for line in reader.lines() {
            let line = line?;

            // convert every character of line into a number vector
            let mut numbers = Vec::new();
            for c in line.chars() {
                numbers.push(c.to_digit(10).unwrap() as i64);
            }

            print!("{:?}", numbers);

            let mut last_i: i64 = -1;
            let mut max = 0;
            for i in 0..12 {
                let (fm, max_i) = find_max(numbers.clone(), i, (last_i + 1) as usize);
                last_i = max_i;

                max += fm * 10i64.pow(11-i);
            }


            // generate all combinations of 12 length

            // let max = combination(numbers.clone(), 0, 0, 0, 0)?;
            sum += max;
            println!(" - max: {}", max);
        }
        println!("sum: {}", sum);
    }

    Ok(())
}

fn find_max(numbers: Vec<i64>, level: u32, start: usize) -> (i64, i64) {
    let mut max = 0;
    let mut max_i = 0;
    for i in start..numbers.len() - (11 - level as usize) {
        if numbers[i] > max {
            max = numbers[i];
            max_i = i;
        }
    }

    println!(" - {} Found max {} at {}", level, max, max_i);

    (max, max_i as i64)
}

fn combination(numbers: Vec<i64>, level: u32, start: usize, current_no: i64, last_max: i64) -> io::Result<i64> {
    let base: i64 = 10;

    for i in start..(numbers.len() - (11 - level as usize)) {
        let no = numbers[i] * base.pow(11 - level) + current_no;

        if level == 11 {
            if no > last_max {
                return Ok(no);
            }
        } else {
            let res = combination(numbers.clone(), level + 1, i + 1, no, last_max)?;
            if res >= 0 {
                return Ok(res);
            }
        }
    }

    Ok(-1)
}