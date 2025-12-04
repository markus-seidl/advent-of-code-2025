use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

macro_rules! is_occupied_or_false {
    ($matrix: expr, $i: expr, $j: expr) => {
        if
            ($i as i64) >= 0 && ($j as i64) >= 0 // lower
            && ($i as i64) < $matrix.len() as i64 && ($j as i64) < $matrix[$i as usize].len() as i64 // upper
        {
            $matrix[$i as usize][$j as usize] == '@'
        }
        else {
            false
        }
    };
}

pub fn adc04() -> io::Result<()> {
    let path = Path::new("adc04.txt");

    { // Part 01
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut matrix: Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let chars: Vec<char> = line.chars().collect();
            matrix.push(chars);
        }

        // print_matrix(&matrix);
        println!("===========================================");

        // deep clone matrix
        let mut matrix_clone = matrix.clone();

        // ij
        // 01 02 03
        // 11 xx 13
        // 21 22 23
        let mut sum = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let ii = i as i64;
                let jj = j as i64;
                let o_01 = is_occupied_or_false!(matrix, ii-1, jj-1);
                let o_02 = is_occupied_or_false!(matrix, ii-1, j);
                let o_03 = is_occupied_or_false!(matrix, ii-1, j+1);
                let o_11 = is_occupied_or_false!(matrix, ii, jj-1);

                let o_12 = matrix[i][j] == '@';

                let o_13 = is_occupied_or_false!(matrix, i, j+1);
                let o_21 = is_occupied_or_false!(matrix, i+1, jj-1);
                let o_22 = is_occupied_or_false!(matrix, i+1, j);
                let o_23 = is_occupied_or_false!(matrix, i+1, j+1);

                let mut occupied = 0;
                if o_01 {
                    occupied += 1;
                }
                if o_02 {
                    occupied += 1;
                }
                if o_03 {
                    occupied += 1;
                }
                if o_11 {
                    occupied += 1;
                }
                if o_13 {
                    occupied += 1;
                }
                if o_21 {
                    occupied += 1;
                }
                if o_22 {
                    occupied += 1;
                }
                if o_23 {
                    occupied += 1;
                }

                if occupied < 4 && o_12 {
                    matrix_clone[i][j] = 'x';
                    sum += 1;
                }
            }
        }

        // print_matrix(&matrix_clone);
        println!("sum part 01: {}", sum);

    }

    { // Part 02
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut matrix: Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let chars: Vec<char> = line.chars().collect();
            matrix.push(chars);
        }

        print_matrix(&matrix);
        println!("===========================================");

        // deep clone matrix
        let mut matrix_clone = matrix.clone();

        // ij
        // 01 02 03
        // 11 xx 13
        // 21 22 23
        let mut sum = 0;
        let mut last_sum = 0;

        while true {
            last_sum = sum;
            for i in 0..matrix.len() {
                for j in 0..matrix[i].len() {
                    let ii = i as i64;
                    let jj = j as i64;
                    let o_01 = is_occupied_or_false!(matrix, ii-1, jj-1);
                    let o_02 = is_occupied_or_false!(matrix, ii-1, j);
                    let o_03 = is_occupied_or_false!(matrix, ii-1, j+1);
                    let o_11 = is_occupied_or_false!(matrix, ii, jj-1);

                    let o_12 = matrix[i][j] == '@';

                    let o_13 = is_occupied_or_false!(matrix, i, j+1);
                    let o_21 = is_occupied_or_false!(matrix, i+1, jj-1);
                    let o_22 = is_occupied_or_false!(matrix, i+1, j);
                    let o_23 = is_occupied_or_false!(matrix, i+1, j+1);

                    let mut occupied = 0;
                    if o_01 {
                        occupied += 1;
                    }
                    if o_02 {
                        occupied += 1;
                    }
                    if o_03 {
                        occupied += 1;
                    }
                    if o_11 {
                        occupied += 1;
                    }
                    if o_13 {
                        occupied += 1;
                    }
                    if o_21 {
                        occupied += 1;
                    }
                    if o_22 {
                        occupied += 1;
                    }
                    if o_23 {
                        occupied += 1;
                    }

                    if occupied < 4 && o_12 {
                        matrix_clone[i][j] = '.';
                        sum += 1;
                    }
                }
            }

            if last_sum == sum {
                println!("Stopping, no more rolls to remove.");
                break;
            }

            matrix = matrix_clone.clone();
        }

        print_matrix(&matrix_clone);
        println!("sum Part 02: {}", sum);

        Ok(())
    }
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}
// let o_01 = matrix[i-1][j-1];
//                 let o_02 = matrix[i-1][j];
//                 let o_03 = matrix[i-1][j+1];
//                 let o_11 = matrix[i][j-1];
//
//                 let o_12 = matrix[i][j]; // center
//
//                 let o_13 = matrix[i][j+1];
//                 let o_21 = matrix[i+1][j-1];
//                 let o_22 = matrix[i+1][j];
//                 let o_23 = matrix[i+1][j+1];


// let o_01 = if i as i64 - 1 >= 0 && j as i64 - 1 >= 0 { matrix[i - 1][j - 1] == '@' } else { false };
//                 let o_02 = if i as i64 - 1 >= 0 { matrix[i - 1][j] == '@' } else { false };
//                 let o_03 = if i as i64 - 1 >= 0 && j as i64 + 1 < matrix[i].len() as i64 { matrix[i - 1][j + 1] == '@' } else { false };
//                 let o_11 = if j as i64 >= 0 { matrix[i][j - 1] == '@' } else { false };
//
//                 let o_12 = matrix[i][j]; // center
//
//                 let o_13 = if j as i64 + 1 < matrix[i].len() as i64 {matrix[i][j + 1]== '@' } else { false };
//                 let o_21 = matrix[i + 1][j - 1];
//                 let o_22 = matrix[i + 1][j];
//                 let o_23 = matrix[i + 1][j + 1];

