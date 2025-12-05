use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn adc05() -> io::Result<()> {
    let path = Path::new("adc05.txt");

    { // Part 01
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut fresh_db: Vec<(i64, i64)> = Vec::new();
        let mut ingredient_db: Vec<i64> = Vec::new();

        let mut db_part = true;
        for line in reader.lines() {
            let line = line?;

            if line == "" {
                db_part = false;
                continue;
            }

            if db_part {
                let (start, end) = line.split_once("-").unwrap();
                let start = start.parse::<i64>().unwrap();
                let end = end.parse::<i64>().unwrap();

                println!("FreshDB: {}-{}", start, end);
                store_in_db(&mut fresh_db, (start, end));
            } else {
                println!("Ingredient: {}", line);
                ingredient_db.push(line.parse::<i64>().unwrap());
            }
        }

        let mut fresh = 0;
        for ingredient in ingredient_db {
            if is_in_db(&mut fresh_db, ingredient) {
                // println!("Ingredient {} is in fresh_db", ingredient);
                fresh += 1;
            } else {
                // println!("Ingredient {} is not in fresh_db", ingredient);
            }
        }

        println!("Fresh {} ingredients", fresh);
    }

    { // Part 02
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut fresh_db: Vec<(i64, i64)> = Vec::new();

        for line in reader.lines() {
            let line = line?;

            if line == "" {
                break;
            }

            let (start, end) = line.split_once("-").unwrap();
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();

            println!("FreshDB: {}-{}", start, end);
            store_in_db(&mut fresh_db, (start, end));
        }

        // Fix overlapping ranges
        let mut fresh_db_fixed: Vec<(i64, i64)> = Vec::new();
        // V1
        for i in 0..fresh_db.len() {
            let candidate = fresh_db[i];

            let left_idx = is_in_db_i(&mut fresh_db_fixed, candidate.0);
            let right_idx = is_in_db_i(&mut fresh_db_fixed, candidate.1);

            if left_idx == right_idx && left_idx >= 0 {
                // contained
            } else if left_idx == right_idx && left_idx >= 0 {
                // to add
                fresh_db_fixed.push((candidate.0, candidate.1));
            } else if left_idx >= 0 && right_idx < 0 { // left is contained, right isn't! TODO there could be a range in between!
                // right extension
                fresh_db_fixed[left_idx as usize] = (candidate.0, fresh_db_fixed[left_idx as usize].1);
            } else if left_idx < 0 && right_idx >= 0 {
                // left extension
                fresh_db_fixed[right_idx as usize] = (fresh_db_fixed[right_idx as usize].0, candidate.1);
            }
        }

        // V0
        // for i in 0..fresh_db.len() {
        //     let candidate = fresh_db[i];
        //
        //     let mut add = false;
        //     for j in 0..fresh_db_fixed.len() {
        //         let range_j = fresh_db_fixed[j];
        //
        //         // Cases:
        //         // range_j.0 <= candidate.0 && range_j.1 >= candidate.1 (larger)
        //         //  --> extend
        //         // range_j.0 >= candidate.0 && range_j.1 <= candidate.1 (contained)
        //         //  --> ignore
        //         // range_j.0 <= candidate.0 && range_j.1 <= candidate.1 && range_j.1 >= candidate.0 (left extension)
        //         //   --> adjust
        //         // range_j.0 >= candidate.0 && range_j.1 >= candidate.1 && range_j.0 <= candidate.1 (right extension)
        //         //   --> adjust
        //
        //         // range_j.0 <= candidate.0 && range_j.1 <= candidate.0 (outside.1)
        //         // range_j.0 >= candidate.1 && range_j.1 >= candidate.1 (outside.2)
        //         //   --> add
        //
        //         if range_j.0 <= candidate.0 && range_j.1 >= candidate.1 {
        //             // larger
        //             fresh_db_fixed[j] = (range_j.0, range_j.1);
        //             break;
        //         } else if range_j.0 >= candidate.0 && range_j.1 <= candidate.1 {
        //             add = false;
        //             break;
        //         } else if range_j.0 <= candidate.0 && range_j.1 <= candidate.1 && range_j.1 >= candidate.0 {
        //             // right extension
        //             add = false;
        //             fresh_db_fixed[j] = (range_j.0, candidate.1);
        //             break;
        //         } else if range_j.0 >= candidate.0 && range_j.1 >= candidate.1 && range_j.0 <= candidate.1 {
        //             // left extension
        //             add = false;
        //             fresh_db_fixed[j] = (candidate.0, range_j.1);
        //             break;
        //         } else if (range_j.0 <= candidate.0 && range_j.1 <= candidate.0) || (range_j.0 >= candidate.1 && range_j.1 >= candidate.1) {
        //             // add
        //             add = true; // check the other ranges as well
        //         }
        //     }
        //
        //     if add || fresh_db_fixed.len() == 0 {
        //         fresh_db_fixed.push(candidate);
        //     }
        //
        // }

        println!("FreshDB fixed: {:?}", fresh_db_fixed);
    }
    Ok(())
}


fn store_in_db(db: &mut Vec<(i64, i64)>, range: (i64, i64)) {
    db.push(range)
}

fn is_in_db(db: &mut Vec<(i64, i64)>, needle: i64) -> bool {
    for range in db {
        if range.0 <= needle && needle <= range.1 {
            return true;
        }
    }

    false
}

fn is_in_db_i(db: &mut Vec<(i64, i64)>, needle: i64) -> i64 {
    for i in 0..db.len() {
        let range = db[i];
        if range.0 <= needle && needle <= range.1 {
            return i as i64;
        }
    }

    -1
}

