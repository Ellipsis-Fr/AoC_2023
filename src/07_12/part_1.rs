use std::collections::{HashMap, HashSet};

use AoC_2023::text_file_reader::TextFileReader;

enum KindOfCamelHand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}
const ORDER_OF_CARDS: [&str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];

fn main() {
    println!("Puzzle du 07/12 Partie 1");
    
    let puzzle = get_puzzle();
    let camel_games = get_poker_games(puzzle);
    let ordered_hands = get_ordered_hands(&camel_games.keys().collect::<Vec<_>>());
    let total_winnings = get_total_winnings(ordered_hands, &camel_games);
    println!("Total winnings : {total_winnings}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("07_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_poker_games(puzzle: Vec<String>) -> HashMap<String, u32> {
    puzzle.into_iter().map(|l| -> (String, u32) {
        let data = l.split_whitespace().collect::<Vec<_>>();
        ((data.get(0).unwrap()).to_string(), data.get(1).unwrap().parse::<u32>().unwrap())
    }).collect::<HashMap<_, _>>()
}

fn get_ordered_hands<'a>(hands: &Vec<&'a String>) -> Vec<&'a String> {
    
    let mut five_of_a_kind = Vec::new();
    let mut four_of_a_kind = Vec::new();
    let mut full_house = Vec::new();
    let mut three_of_a_kind = Vec::new();
    let mut two_pair = Vec::new();
    let mut one_pair = Vec::new();
    let mut high_card = Vec::new();

    for hand in hands {
        let differents_cards = hand.chars().collect::<HashSet<_>>();

        match get_kind_of_hand(hand, differents_cards) {
            KindOfCamelHand::FiveOfAKind => insert(&mut five_of_a_kind, hand),
            KindOfCamelHand::FourOfAKind => insert(&mut four_of_a_kind, hand),
            KindOfCamelHand::FullHouse => insert(&mut full_house, hand),
            KindOfCamelHand::ThreeOfAKind => insert(&mut three_of_a_kind, hand),
            KindOfCamelHand::TwoPair => insert(&mut two_pair, hand),
            KindOfCamelHand::OnePair => insert(&mut one_pair, hand),
            KindOfCamelHand::HighCard => insert(&mut high_card, hand),
        }
    }

    vec![high_card, one_pair, two_pair, three_of_a_kind, full_house, four_of_a_kind, five_of_a_kind].concat()
}

fn get_kind_of_hand(hand: &String, cards: HashSet<char>) -> KindOfCamelHand {
    match cards.len() {
        1 => KindOfCamelHand::FiveOfAKind,
        2 => {
            let all_card = hand.chars().collect::<Vec<_>>();

            for card in cards {
                let count = all_card.iter().filter(|c| **c == card).count();
                
                if count == 1 {
                    return KindOfCamelHand::FourOfAKind;
                }
            }

            KindOfCamelHand::FullHouse
        },
        3 => {
            let all_card = hand.chars().collect::<Vec<_>>();

            for card in cards {
                let count = all_card.iter().filter(|c| **c == card).count();
                
                if count == 3 {
                    return KindOfCamelHand::ThreeOfAKind;
                }
            }

            KindOfCamelHand::TwoPair
        },
        4 => KindOfCamelHand::OnePair,
        _ => KindOfCamelHand::HighCard
    }
}

fn insert<'a>(hands: &mut Vec<&'a String>, new_hand: &'a String) {
    let cards_from_new_hand = new_hand.chars().collect::<Vec<_>>();
    let mut position = 0;
    
    'outer: for (index, hand) in hands.iter().enumerate() {
        let cards_from_hand = hand.chars().collect::<Vec<_>>();

        for (a, b) in cards_from_hand.iter().zip(cards_from_new_hand.iter()) {
            let a_strength = ORDER_OF_CARDS.iter().position(|c| c.chars().next().unwrap() == *a).unwrap();
            let b_strength = ORDER_OF_CARDS.iter().position(|c| c.chars().next().unwrap() == *b).unwrap();

            
            if b_strength < a_strength {
                position = index;
                break 'outer;
            } else if b_strength > a_strength {
                position += 1;
                continue 'outer;
            }
        }

        position += 1;
    }

    hands.insert(position, new_hand);
}

fn get_total_winnings(ordered_hands: Vec<&String>, camel_games: &HashMap<String, u32>) -> u32 {
    let mut total_winnings = 0;
    
    for (index, hand) in ordered_hands.iter().enumerate() {
        let bid = camel_games.get(*hand).unwrap();
        total_winnings += (index as u32 + 1) * bid;
    }

    total_winnings
}