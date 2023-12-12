use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;

const ZERO_TO_NINE: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
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

    let mut values: Vec<i64> = Vec::new();

    for line in lines {
        let mut first: i64 = -1 as i64;
        let mut first_index: usize = line.len();
        let mut second: i64 = -1;
        let mut second_index: usize = 0;
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                let value = c.to_digit(10).unwrap() as i64;
                if first == -1 as i64 {
                    first = value;
                    first_index = i;
                }
                second = value;
                second_index = i;
            }
        }

        ZERO_TO_NINE.iter().for_each(|word| {
            let first_index_alphabetical = line.find(word);
            let last_index_alphabetical = line.rfind(word);

            if let Some(first_index_alphabetical) = first_index_alphabetical {
                dbg!(first_index_alphabetical, word, first_index);
                if first_index_alphabetical < first_index {
                    first = (ZERO_TO_NINE.iter().position(|x| x == word).unwrap() + 1) as i64;
                    first_index = first_index_alphabetical;
                }
            }
            if let Some(last_index_alphabetical) = last_index_alphabetical {
                if last_index_alphabetical > second_index {
                    second = (ZERO_TO_NINE.iter().position(|x| x == word).unwrap() + 1) as i64;
                    second_index = last_index_alphabetical;
                }
            }
        });
        dbg!(&line, first, second);
        values.push(first * 10 + second);
    }
    dbg!(&values);
    let sum: i64 = values.iter().sum();
    dbg!(sum);
}
