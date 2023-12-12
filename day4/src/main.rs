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

    // dbg!(&lines);
    let mut cards = Vec::with_capacity(lines.len());

    for line in lines {
        // dbg!(&line);
        let game = line.split(':').collect::<Vec<&str>>();
        let game_id = game[0].split(' ').collect::<Vec<&str>>();
        // dbg!(game_id);
        let numbers = game[1].trim().split(" | ").collect::<Vec<&str>>();
        let left_nums = numbers[0]
            .trim()
            .replace("  ", " ")
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        // dbg!(left_nums);
        // dbg!(numbers[1]);
        let right_nums = numbers[1]
            .trim()
            .replace("  ", " ")
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        // dbg!(right_nums);
        cards.push(Card {
            id: game_id.last().unwrap().parse::<usize>().unwrap(),
            points: 0,
            left_nums,
            right_nums,
        });
    }

    // dbg!(&cards);
    let mut total_points = 0;
    let mut total_cards = cards.len();

    let mut i = 0;

    while i < cards.len() {
        let card = cards.get(i).unwrap().to_owned();
        let overlap_count = card
            .left_nums
            .iter()
            .filter(|&num| card.right_nums.contains(num))
            .count();
        let value = 2u32.pow(overlap_count as u32 - 1);
        // dbg!(value);
        // println!("Overlap count: {}", overlap_count);
        // card.set_points(value);
        total_points += value;

        for j in 0..overlap_count {
            // dbg!(j, i, card.id);
            total_cards += 1;
            cards.push(cards.get(card.id + j).unwrap().to_owned());
        }
        i += 1;
    }

    println!("Total points: {}", total_points);
    println!("Total cards: {}", total_cards);
    println!("{:?}", now.elapsed());
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Card {
    pub id: usize,
    pub points: u32,
    pub left_nums: Vec<u32>,
    pub right_nums: Vec<u32>,
}

impl Card {
    pub fn set_points(&mut self, points: u32) {
        self.points = points;
    }
}
