use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// Rules:
// Suffix: 0 -> invalid
// Repeating numbers (can be any length!) -> invalid
//

pub fn adc02() -> io::Result<()> {
    let path = Path::new("adc02.txt");

    { // Part 01
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut sum = 0;
        for line in reader.lines() {
            let line = line?;

            for ranges in line.split(',') {
                let range = ranges.split('-').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                let from = range[0];
                let to = range[1];

                // println!("from {} to {}", from, to);
                for i in from..=to {
                    if i < 10 {
                        continue;
                    }

                    let i_str = i.to_string();
                    // for i in 1..(i_str.len() / 2 + 1) {
                    // let found = find_similar_numbers(&i_str, i)?;
                    let found = find_similar_numbers(&i_str, 0)?;
                    if found >= 0 {
                        // println!(" - found {} in {}", found, i_str);
                        sum += i;
                    }

                    // }

                }
            }
        }
        println!(" sum Part 01: {}", sum);
    }

    { // Part 02
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut sum = 0;
        for line in reader.lines() {
            let line = line?;

            for ranges in line.split(',') {
                let range = ranges.split('-').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                let from = range[0];
                let to = range[1];

                println!("from {} to {}", from, to);
                for i in from..=to {
                    if i < 10 {
                        continue;
                    }

                    let i_str = i.to_string();
                    for il in 1..(i_str.len() / 2 + 1) {
                        let found = find_similar_numbers_p2(&i_str, il)?;
                        if found >= 0 {
                            println!(" - found {} in {}", found, i_str);
                            sum += i;
                            break;
                        }
                    }
                }
            }
        }
        println!(" sum Part 02: {}", sum);
    }

    Ok(())
}

fn find_similar_numbers(input: &str, length: usize) -> io::Result<i32> {
    if input.len() % 2 == 1 {
        return Ok(-1); // len 3 doesn't allow numbers to be repeated twice
    }

    let half = input.len() / 2;
    let sub_str_0 = &input[0..half];
    let sub_str_1 = &input[half..];
    if sub_str_0 == sub_str_1 {
        return Ok(sub_str_0.parse().unwrap());
    }

    Ok(-1)
}

fn find_similar_numbers_p2(input: &str, length: usize) -> io::Result<i32> {
    if length == 0 {
        return Ok(-1);
    }

    for i in 0..(input.len() - length) {
        let sub_str = &input[i..(i + length)]; // try to find this substr again in the input
        let count = input.matches(sub_str).count();
        let does_it_max_out = (count * sub_str.len()) as i64 - input.len() as i64;

        if does_it_max_out == 0 {
            return Ok(sub_str.parse().unwrap());
        }
    }

    Ok(-1)
}
