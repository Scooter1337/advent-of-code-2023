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

    let mut hands = Vec::new();

    for line in lines {
        let cardsplit = line.split(' ').collect::<Vec<&str>>();
        hands.push(Hand {
            cards: cardsplit[0].to_string(),
            value: cardsplit[1].parse::<u64>().unwrap(),
            hand_type: hand_type_calc(&cardsplit[0]),
        });
        dbg!(hand_type_calc(&cardsplit[0]));
    }

    // dbg!(&hands);

    let mut ordered_five_of_a_kind = Vec::new();
    let mut ordered_four_of_a_kind = Vec::new();
    let mut ordered_full_house = Vec::new();
    let mut ordered_three_of_a_kind = Vec::new();
    let mut ordered_two_pair = Vec::new();
    let mut ordered_one_pair = Vec::new();
    let mut ordered_high_card = Vec::new();

    for hand in hands {
        match hand.hand_type {
            HandType::FiveOfAKind => ordered_five_of_a_kind.push(hand),
            HandType::FourOfAKind => ordered_four_of_a_kind.push(hand),
            HandType::FullHouse => ordered_full_house.push(hand),
            HandType::ThreeOfAKind => ordered_three_of_a_kind.push(hand),
            HandType::TwoPair => ordered_two_pair.push(hand),
            HandType::OnePair => ordered_one_pair.push(hand),
            HandType::HighCard => ordered_high_card.push(hand),
        }
    }

    ordered_five_of_a_kind.sort_by(|a, b| {
        if a_bigger(&a.cards, &b.cards) {
            // dbg!(&a.cards, &b.cards, "less");
            std::cmp::Ordering::Greater
        } else {
            // dbg!(&a.cards, &b.cards, "grt");

            std::cmp::Ordering::Less
        }
    });
    ordered_four_of_a_kind.sort_by(|a, b| {
        if a_bigger(&a.cards, &b.cards) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });
    ordered_full_house.sort_by(|a, b| {
        if a_bigger(&a.cards, &b.cards) {
            // dbg!(&a.cards, &b.cards, "less");
            std::cmp::Ordering::Greater
        } else {
            // dbg!(&a.cards, &b.cards, "grt");

            std::cmp::Ordering::Less
        }
    });
    ordered_three_of_a_kind.sort_by(|a, b| {
        if a_bigger(&a.cards, &b.cards) {
            // dbg!(&a.cards, &b.cards, "less");
            std::cmp::Ordering::Greater
        } else {
            // dbg!(&a.cards, &b.cards, "grt");

            std::cmp::Ordering::Less
        }
    });
    ordered_two_pair.sort_by(|a, b| {
        if a_bigger(&a.cards, &b.cards) {
            // dbg!(&a.cards, &b.cards, "less");
            std::cmp::Ordering::Greater
        } else {
            // dbg!(&a.cards, &b.cards, "grt");

            std::cmp::Ordering::Less
        }
    });
    ordered_one_pair.sort_by(|a, b| {
        if a_bigger(&a.cards, &b.cards) {
            // dbg!(&a.cards, &b.cards, "less");
            std::cmp::Ordering::Greater
        } else {
            // dbg!(&a.cards, &b.cards, "grt");

            std::cmp::Ordering::Less
        }
    });
    ordered_high_card.sort_by(|a, b| {
        if a_bigger(&a.cards, &b.cards) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });

    // dbg!(&ordered_five_of_a_kind);
    // dbg!(&ordered_four_of_a_kind);
    // dbg!(&ordered_full_house);
    // dbg!(&ordered_three_of_a_kind);
    // dbg!(&ordered_two_pair);
    // dbg!(&ordered_one_pair);
    // dbg!(&ordered_high_card);
    ordered_high_card.append(&mut ordered_one_pair);
    ordered_high_card.append(&mut ordered_two_pair);
    ordered_high_card.append(&mut ordered_three_of_a_kind);
    ordered_high_card.append(&mut ordered_full_house);
    ordered_high_card.append(&mut ordered_four_of_a_kind);
    ordered_high_card.append(&mut ordered_five_of_a_kind);

    dbg!(&ordered_high_card);
    let mut total: u64 = 0;
    for i in 0..ordered_high_card.len() {
        // dbg!(
        //     i + 1,
        //     ordered_five_of_a_kind[i].value,
        //     ordered_five_of_a_kind[i].value * (i as u64 + 1)
        // );
        total += ordered_high_card[i].value * (i as u64 + 1);
    }

    println!("Total: {}", total);
    println!("Time elapsed: {:?}", now.elapsed());
}

fn hand_type_calc(cards: &str) -> HandType {
    let mut card_values = Vec::new();

    for card in cards.split("") {
        if card == "" {
            continue;
        }
        card_values.push(card);
    }

    if card_values.iter().find(|&&c| c == "J").is_some() {
        // count cards
        let mut card_count = [0; 13];
        while card_values.contains(&"J") {
            for card in &card_values {
                match card {
                    &"A" => card_count[0] += 1,
                    &"2" => card_count[1] += 1,
                    &"3" => card_count[2] += 1,
                    &"4" => card_count[3] += 1,
                    &"5" => card_count[4] += 1,
                    &"6" => card_count[5] += 1,
                    &"7" => card_count[6] += 1,
                    &"8" => card_count[7] += 1,
                    &"9" => card_count[8] += 1,
                    &"T" => card_count[9] += 1,
                    &"J" => card_count[10] -= 1,
                    &"Q" => card_count[11] += 1,
                    &"K" => card_count[12] += 1,
                    _ => panic!("Invalid card"),
                }
            }

            if card_count[10] == -5 {
                return HandType::FiveOfAKind;
            }

            // find biggest card
            let mut biggest_card = 0;
            let mut biggest_card_count = 0;
            for i in 0..card_count.len() {
                if card_count[i] > biggest_card_count {
                    biggest_card = i;
                    biggest_card_count = card_count[i];
                }
            }

            dbg!(
                &card_values,
                biggest_card,
                card_count[biggest_card],
                card_count[9],
                "J"
            );
            let pos = card_values.iter().position(|&x| x == "J").unwrap();
            let biggest_card_char = int_to_char(biggest_card).to_string();
            card_values[pos] = int_to_char_str(biggest_card);
            dbg!(pos, &card_values, &biggest_card_char);
        }
    }

    card_values.sort();
    // count distinctive
    let mut count = 1;

    // dbg!(&card_values);

    for i in 0..card_values.len() - 1 {
        // dbg!(card_values[i], card_values[i + 1]);
        if card_values[i] != card_values[i + 1] {
            count += 1;
        }
    }

    // dbg!(count);
    if count == 1 {
        return HandType::FiveOfAKind;
    }
    if count == 2 && (card_values[0] == card_values[3] || card_values[1] == card_values[4]) {
        return HandType::FourOfAKind;
    }
    if count == 2 && (card_values[0] == card_values[2] && card_values[3] == card_values[4])
        || (card_values[0] == card_values[1] && card_values[2] == card_values[4])
    {
        return HandType::FullHouse;
    }
    if count == 3
        && (card_values[0] == card_values[2]
            || card_values[1] == card_values[3]
            || card_values[2] == card_values[4])
    {
        return HandType::ThreeOfAKind;
    }
    if count == 3
        && (card_values[0] == card_values[1] && card_values[2] == card_values[3]
            || card_values[1] == card_values[2] && card_values[3] == card_values[4]
            || card_values[0] == card_values[1] && card_values[3] == card_values[4])
    {
        return HandType::TwoPair;
    }
    if count == 4 {
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn a_bigger(a: &str, b: &str) -> bool {
    let a = a.chars().collect::<Vec<char>>();
    let b = b.chars().collect::<Vec<char>>();

    for i in 0..a.len() {
        match a[i] {
            c if c == b[i] => continue,
            'A' => return true,
            'K' => {
                if b[i] == 'A' {
                    return false;
                }
                return true;
            }
            'Q' => {
                if b[i] == 'K' || b[i] == 'A' {
                    return false;
                }
                return true;
            }
            'J' => {
                return false;
            }
            'T' => {
                if b[i] == 'K' || b[i] == 'Q' || b[i] == 'A' {
                    return false;
                }
                return true;
            }
            _ => {
                if b[i] == 'K' || b[i] == 'Q' || b[i] == 'T' || b[i] == 'A' {
                    return false;
                }
                if b[i] == 'J' {
                    return true;
                }
                return a[i] > b[i];
            }
        }
    }

    false
}

#[derive(Debug)]
pub struct Hand {
    cards: String,
    value: u64,
    hand_type: HandType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[test]
fn test_hand_type() {
    assert_eq!(hand_type_calc("32T3K"), HandType::OnePair,);
    assert_eq!(hand_type_calc("T55J5"), HandType::ThreeOfAKind,);
    assert_eq!(hand_type_calc("KK677"), HandType::TwoPair,);
    assert_eq!(hand_type_calc("KTJJT"), HandType::TwoPair,);
    assert_eq!(hand_type_calc("QQQJA"), HandType::ThreeOfAKind,);
    assert_eq!(hand_type_calc("JJJJJ"), HandType::FiveOfAKind);
    assert_eq!(hand_type_calc("JJJJJ"), HandType::FiveOfAKind);
    assert_eq!(hand_type_calc("JJJJQ"), HandType::FourOfAKind);
    assert_eq!(hand_type_calc("JJQJJ"), HandType::FourOfAKind);
}

#[test]
fn comparisons() {
    macro_rules! compare {
        ($hand1:literal, $hand2:literal, $side:literal) => {
            let mut vec = vec![$hand1, $hand2];
            vec.sort_by(|a, b| {
                if a_bigger(&a, &b) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            assert_eq!(vec[0] == $hand1, $side);

            let mut vec = vec![$hand2, $hand1];
            vec.sort_by(|a, b| {
                if a_bigger(&a, &b) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            assert_eq!(vec[0] == $hand1, $side);
        };
    }

    // compare!("2345J", "23455", false);

    // compare!("2345J", "23454", false);

    // compare!("23329", "2332J", true);

    // compare!("JKKK2", "QQQQ2", true);

    // compare!("KJKK2", "QQQQ2", false);

    // compare!("22345", "23455", true);
    // compare!("22325", "23455", true);
    // compare!("22345", "23455", true);
    // compare!("22345", "23455", true);

    // compare!("2JJ34", "29934", false);
    // compare!("JJ324", "99324", false);

    compare!("TTTTJ", "J4444", false);
}

fn int_to_char(i: usize) -> String {
    match i {
        0 => "A".to_string(),
        1 => '2'.to_string(),
        2 => '3'.to_string(),
        3 => '4'.to_string(),
        4 => '5'.to_string(),
        5 => '6'.to_string(),
        6 => '7'.to_string(),
        7 => '8'.to_string(),
        8 => '9'.to_string(),
        9 => 'T'.to_string(),
        10 => 'J'.to_string(),
        11 => 'Q'.to_string(),
        12 => 'K'.to_string(),
        _ => panic!("Invalid card"),
    }
}

fn int_to_char_str(i: usize) -> &'static str {
    match i {
        0 => "A",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
        7 => "8",
        8 => "9",
        9 => "T",
        10 => "J",
        11 => "Q",
        12 => "K",
        _ => panic!("Invalid card"),
    }
}
