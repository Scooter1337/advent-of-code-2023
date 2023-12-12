use std::{
    cmp::{max, min},
    env,
    fs::File,
    io::BufRead,
};

#[allow(clippy::all, unused_parens, dead_code, unused)]
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

    let linelength = lines[0].len();

    // dbg!(&lines);

    let mut numbers = Vec::new();

    let mut i = 0;

    while i < lines.len() {
        let line = &lines[i];
        let mut j = 0;
        while j < line.len() {
            // dbg!(i, j);
            let c = line.chars().nth(j).unwrap();

            if c.is_numeric() {
                let mut value = c.to_string();
                let x = j;
                let y = i;
                loop {
                    j += 1;
                    if j >= line.len() {
                        break;
                    }
                    let c = line.chars().nth(j).unwrap();
                    if c.is_numeric() {
                        value.push(c);
                    } else {
                        j -= 1;
                        break;
                    }
                }
                let size = value.len() as usize;
                let value = value.parse::<i64>().unwrap();
                let number = Number { size, value, x, y };
                numbers.push(number);
            }
            j += 1;
        }
        i += 1;
    }
    // dbg!(&numbers);

    let mut gears: Vec<Gear> = Vec::new();

    let mut total: i64 = 0;
    let mut gear_total: i64 = 0;

    const GEAR: bool = false;

    for number in numbers {
        for i in (max(number.x, 1) - 1)..min((number.x + number.size + 1), linelength) {
            // dbg!(i);
            for j in (max(number.y, 1) - 1)..min((number.y + 2), lines.len()) {
                // dbg!(j);
                match lines[j].chars().nth(i).unwrap() {
                    c if c.is_numeric() => (),
                    '.' => (),
                    c => {
                        // dbg!(i, j);
                        total += number.value;
                        if c == '*' && GEAR {
                            let curr_gear = gears.iter().find(|&gear| gear.x == i && gear.y == j);

                            if let Some(gear) = curr_gear {
                                gear_total += (gear.value * number.value);
                            } else {
                                gears.push(Gear {
                                    x: i,
                                    y: j,
                                    value: number.value,
                                })
                            }
                        }
                    }
                }
            }
        }
        // dbg!("");
    }
    println!("Total: {total}");
    // dbg!(gears);
    println!("Gear Total: {gear_total}");
    println!("time: {:#?}", now.elapsed());
}

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
    size: usize,
    value: i64,
    x: usize,
    y: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Gear {
    pub x: usize,
    pub y: usize,
    pub value: i64,
}
