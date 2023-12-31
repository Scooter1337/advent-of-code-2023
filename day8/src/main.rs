use std::{env, fs::File, io::BufRead};

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

    // dbg!(&lines);

    let order = lines[0].to_owned();

    // dbg!(&order);

    let mut map = std::collections::HashMap::new();

    let mut first_three_letters: Vec<String> = Vec::new();

    for line in lines {
        if !line.contains('=') {
            continue;
        }
        let line = line.replace([' ', '(', ')'], "");
        let splitline = line
            .split('=')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let name = splitline[0].as_str().to_owned();
        if &name[2..=2] == "A" {
            first_three_letters.push(name[0..=2].to_string());
        }
        let splitleftright = splitline[1]
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let left = splitleftright[0].to_string();
        let right = splitleftright[1].to_string();

        let route = Route { left, right };
        map.insert(name, route);
    }

    let mut moves = 0;

    loop {
        if first_three_letters.iter().all(|f| &f[2..=2] == "Z") {
            break;
        }
        first_three_letters = first_three_letters
            .iter()
            .map(|f| {
                let curr_route = map.get(f).unwrap();
                let curr_order = order.chars().nth(moves % order.len()).unwrap();
                // dbg!(&first_three_letters);
                let new_f = match curr_order {
                    'L' => curr_route.left.to_owned(),
                    'R' => curr_route.right.to_owned(),
                    _ => panic!("Invalid order"),
                };
                // dbg!(&curr_route);

                new_f
            })
            .collect();
        moves += 1;
        if moves % 1000000 == 0 {
            println!("Moves: {}", moves);
        }
    }

    println!("Moves: {}", moves);
    println!("Time elapsed: {:?}", now.elapsed());
}

#[derive(Debug)]
struct Route {
    left: String,
    right: String,
}
