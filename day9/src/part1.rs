use std::{env, fs::File, io::BufRead};

pub fn part1() {
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

    // let length = lines[0].split(' ').collect::<Vec<&str>>().len();
    // dbg!(length);
    let mut linestructs = Vec::with_capacity(lines.len());

    for line in lines {
        let values = line
            .split(' ')
            .map(|x| x.parse::<i64>().expect("Could not parse number"))
            .collect::<Vec<i64>>();
        linestructs.push(Line {
            value: *values.last().unwrap(),
            values,
        });
        // dbg!(&line);
    }

    for line in &mut linestructs {
        while !line.values.iter().all(|&x| x == 0) {
            let mut newvalues = Vec::with_capacity(line.values.len() - 1);
            for i in 0..line.values.len() - 1 {
                newvalues.push(line.values[i + 1] - line.values[i]);
            }
            line.values = newvalues;
            line.value += *line.values.last().unwrap();
        }
    }

    // dbg!(&linestructs);

    let sum = linestructs.iter().fold(0, |acc, x| acc + x.value);
    // dbg!(sum);

    println!("Sum: {}", sum);
    println!("Time elapsed: {:?}", now.elapsed());
}

#[derive(Debug)]
pub struct Line {
    pub values: Vec<i64>,
    pub value: i64,
}
