use std::{env, fs::File, io::BufRead};
#[allow(dead_code, unused_variables)]
fn main() {
    let now = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file path as a command line argument");
    }
    let file_path = &args[1];

    let mut lines = Vec::new();

    let file = File::open(file_path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        lines.push(line.expect("Could not read line"));
    }

    let times = lines[0]
        .split(' ')
        .filter(|&x| !x.is_empty() && !x.contains(':'))
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let times_part_2 = lines[0].split(":").collect::<Vec<&str>>()[1]
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();

    let distances = lines[1]
        .split(' ')
        .filter(|&x| !x.is_empty() && !x.contains(':'))
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let distances_part_2 = lines[1].split(":").collect::<Vec<&str>>()[1]
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();

    // dbg!(&times, &distances);
    // dbg!(times_part_2, distances_part_2);

    let mut total = 1;

    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut currtotal = 0;
        for i in 0..*time {
            let timeleft = time - i;
            let speed = i;
            let distance_covered = speed * timeleft;
            if distance_covered > *distance {
                // dbg!(speed, distance_covered, timeleft);
                currtotal += 1;
            }
        }
        // dbg!(currtotal);
        total *= currtotal;
    }

    let mut total_part_2 = 0;

    for i in 0..times_part_2 {
        let timeleft = times_part_2 - i;
        let speed = i;
        let distance_covered = speed * timeleft;
        if distance_covered > distances_part_2 {
            // dbg!(speed, distance_covered, timeleft);
            total_part_2 += 1;
        }
    }

    println!("Total: {}", total);
    println!("Total part 2: {}", total_part_2);
    println!("Time elapsed: {:?}", now.elapsed());
}
