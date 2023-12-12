use std::{env, fs::File, io::BufRead};

const BLUE_MAX: i64 = 14;
const RED_MAX: i64 = 12;
const GREEN_MAX: i64 = 13;

// #[allow(clippy::all)]
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

    let mut games: Vec<Line> = Vec::new();

    for line in lines {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;
        let split = line.split(':').collect::<Vec<&str>>(); // ["Game 1", " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]
        let game_id = split[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<i64>()
            .unwrap(); // ["Game", "1"]
        dbg!(&game_id);
        let split = split[1].trim().split("; ").collect::<Vec<&str>>(); // [" 3 blue, 4 red", " 1 red, 2 green, 6 blue", " 2 green"]
        dbg!(&split);
        for split_line in split {
            let pull = split_line.split(", ").collect::<Vec<&str>>(); // ["3", "blue,", "4", "red"]
            dbg!(&pull);
            for pull_line in pull {
                let single_pull = pull_line.split(' ').collect::<Vec<&str>>(); // ["3", "blue"]
                dbg!(&single_pull);
                let color = single_pull[1].trim().to_string();
                let number = single_pull[0].parse::<i64>().unwrap();
                if (color == "blue") && (number > max_blue) {
                    max_blue = number;
                } else if (color == "red") && (number > max_red) {
                    max_red = number;
                } else if (color == "green") && (number > max_green) {
                    max_green = number;
                }
            }
        }
        games.push(Line {
            game_id,
            red_max: max_red,
            blue_max: max_blue,
            green_max: max_green,
        });
    }

    dbg!(&games);

    let mut total = 0;
    let mut power_total: i64 = 0;

    for game in games {
        if game.red_max <= RED_MAX && game.blue_max <= BLUE_MAX && game.green_max <= GREEN_MAX {
            total += game.game_id;
        }
        power_total += game.red_max * game.blue_max * game.green_max;
    }
    dbg!(total, power_total);
}

#[derive(Debug)]
pub struct Line {
    pub game_id: i64,
    pub red_max: i64,
    pub blue_max: i64,
    pub green_max: i64,
}
