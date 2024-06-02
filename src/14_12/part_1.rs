use std::{collections::VecDeque, time::Instant};

use AoC_2023::text_file_reader::TextFileReader;

const _SPACE: char = '.';
const CUBE_SHAPED_ROCK: char = '#';
const ROUNDED_ROCK: char = 'O';

fn main() {
    println!("Puzzle du 14/12 Partie 1");
    let now = Instant::now();
    
    let platform  = get_puzzle();
    let platform = rotate_90d(platform);
    let total_load = compute_total_load(platform);
    println!("Total load : {total_load}");

    println!("took: {:?}", now.elapsed());
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("14_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn rotate_90d(platform: Vec<String>) -> Vec<String> {
    let elements = platform.into_iter().map(|p| p.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut platform_rotated = Vec::new();

    for elements_per_row in elements {
        for (index, element) in elements_per_row.into_iter().enumerate() {
            match platform_rotated.get_mut(index) {
                None => {
                    let mut new_order = VecDeque::new();
                    new_order.push_front(element);
                    platform_rotated.push(new_order);
                },
                Some(new_order) => new_order.push_front(element)
            }
        }
    }

    platform_rotated.into_iter().map(|c| c.into_iter().collect::<String>()).collect()
}

fn compute_total_load(platform: Vec<String>) -> u32 {
    let mut total_load = 0;

    for elements_per_row in platform {
        let mut current_position = 0;
        let mut current_count_rounded_rock = 0;

        for element in elements_per_row.chars() {
            if element == CUBE_SHAPED_ROCK {
                if current_count_rounded_rock > 0 {
                    total_load += compute_load(current_count_rounded_rock, current_position);
                    current_count_rounded_rock = 0;
                }
            } else if element == ROUNDED_ROCK {
                current_count_rounded_rock += 1;
            }
            current_position += 1;
        }

        if current_count_rounded_rock > 0 {
            total_load += compute_load(current_count_rounded_rock, current_position);
        }
    }


    total_load
}

fn compute_load(count_of_rock: u32, mut current_position: u32) -> u32 {
    let mut load = 0;
    (0..count_of_rock).for_each(|_| {
        load += current_position;
        current_position -= 1;
    });
    load
}