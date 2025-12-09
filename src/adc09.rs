use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run() -> io::Result<()> {
    let path = Path::new("adc09.txt");

    { // Part 01
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut coords: Vec<(i64, i64)> = Vec::new();
        for line in reader.lines() {
            let line = line?;

            let numbers: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
            let coord = (numbers[0].parse::<i64>().unwrap(), numbers[1].parse::<i64>().unwrap());
            coords.push(coord);
        }

        println!("{:?}", coords);

        let mut found_0: (i64, i64) = (0, 0);
        let mut found_1: (i64, i64) = (0, 0);
        let mut max_size: f64 = 0.0;
        for pivot in &coords {
            // let mut sorted_coords = coords.clone();
            // sorted_coords.sort_by_key(|coord| distance(*pivot, *coord));
            for coord in &coords {
                if coord != pivot {
                    let area = area(*pivot, *coord);
                    if area > max_size {
                        max_size = area;
                        found_0 = *coord;
                        found_1 = *pivot;
                    }
                }
            }
        }

        println!("{}", max_size);
        println!("{:?}", found_0);
        println!("{:?}", found_1);
    }


    Ok(())
}

fn area(a: (i64, i64), b: (i64, i64)) -> f64 {
    ((a.0 - b.0).abs() + 1) as f64 * ((a.1 - b.1).abs() + 1) as f64
}

fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)
}
