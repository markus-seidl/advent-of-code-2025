use std::ffi::c_double;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run() -> io::Result<()> {
    let path = Path::new("adc08.txt");

    { // Part 01
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut jboxes: Vec<(i64, i64, i64)> = Vec::new();
        for line in reader.lines() {
            let line = line?;

            let numbers: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
            let jbox = (numbers[0].parse::<i64>().unwrap(), numbers[1].parse::<i64>().unwrap(), numbers[2].parse::<i64>().unwrap());
            jboxes.push(jbox);
        }

        println!("{:?}", jboxes);

        let mut circuits: Vec<Vec<(i64, i64, i64)>> = Vec::new();
        for jbox in jboxes {
            let mut circuit: Vec<(i64, i64, i64)> = Vec::new();
            circuit.push(jbox);
            circuits.push(circuit);
        }

        println!("================================================================\n");
        let amount = 1000; // 1000 for puzzle input
        for i in 0..amount - 1 {
            println!("{}", i);
            let (pair_0, pair_1, dist) = find_shortest_pair(&circuits);
            println!("{:?} + {:?} = {}", circuits[pair_0], circuits[pair_1], dist);

            // Remove circuits (remove larger index first to avoid index shifting issues)
            let (idx_first, idx_second) = if pair_0 > pair_1 { (pair_0, pair_1) } else { (pair_1, pair_0) };
            let mut circuit_0 = circuits.remove(idx_first);
            let circuit_1 = circuits.remove(idx_second);

            // Merge circuits
            circuit_0.extend_from_slice(&circuit_1);

            // Add merged circuit back
            circuits.push(circuit_0);
            println!("================================================================\n");

            let mut stop = true;
            for circuit in &circuits {
                if circuit.len() == 1 {
                    stop = false;
                    break;
                }
            }
            if stop {
                break;
            }
        }

        circuits.sort_by_key(|circuit| circuit.len());
        circuits.reverse();
        println!("{:?}", circuits);
        println!("{}", circuits.len());

        println!("{}", circuits[0].len() * circuits[1].len() * circuits[2].len())
    }


    Ok(())
}

fn find_shortest_pair(circuits: &Vec<Vec<(i64, i64, i64)>>) -> (usize, usize, f64) {
    let mut shortest_distance = f64::MAX;
    let mut pair_0 = 0;
    let mut pair_1 = 0;
    for i in 0..circuits.len() {
        for j in 0..circuits.len() {
            if i > j {
                let circuit_i = &circuits[i];
                let circuit_j = &circuits[j];

                let distance = min_distance_circuit(circuit_i, circuit_j);
                if distance < shortest_distance {
                    shortest_distance = distance;
                    pair_0 = i;
                    pair_1 = j;
                }
            }
        }
    }

    (pair_0, pair_1, shortest_distance)
}

fn min_distance_circuit(a: &Vec<(i64, i64, i64)>, b: &Vec<(i64, i64, i64)>) -> f64 {
    let mut min_distance = f64::MAX;
    for i in 0..a.len() {
        for j in 0..b.len() {
            let distance = distance(a[i], b[j]);
            if distance < min_distance {
                min_distance = distance;
            }
        }
    }
    min_distance
}

fn distance(a: (i64, i64, i64), b: (i64, i64, i64)) -> f64 {
    (((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1) + (a.2 - b.2) * (a.2 - b.2)) as f64).sqrt()
}
