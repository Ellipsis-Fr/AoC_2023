use std::{collections::{HashMap, VecDeque}, time::Instant};

use AoC_2023::text_file_reader::TextFileReader;

const SPACE: char = '.';
const CUBE_SHAPED_ROCK: char = '#';
const ROUNDED_ROCK: char = 'O';

const CYCLES: u32 = 1_000_000_000;

#[derive(Debug)]
enum Direction {
	Clockwise,
	CounterClockwise,
}

#[derive(Debug)]
enum Orientation {
	North,
	West,
    South,
	East,
}

fn main() {
    println!("Puzzle du 14/12 Partie 2");
    let now = Instant::now();
    
    let platform  = get_puzzle();
    let (platform, cycles_remainder) = cycles(platform, CYCLES);
    let (mut platform, _) = cycles(platform, cycles_remainder);

    platform = rotate_90d(platform, Direction::Clockwise);
    let total_load = compute_total_load(platform.clone());
    println!("Total load : {total_load}");

    println!("took: {:?}", now.elapsed());
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("14_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn cycles(mut platform: Vec<String>, cycles: u32) -> (Vec<String>, u32) {
    let mut cycles_remainder =0;
    
    let mut cache_1 = HashMap::new();

    for cycle in 1..=cycles {
        platform = rotate_90d(platform, Direction::Clockwise);
        platform = move_platform(platform, Orientation::North);

        platform = rotate_90d(platform, Direction::CounterClockwise);
        platform = move_platform(platform, Orientation::West);

        platform = rotate_90d(platform, Direction::Clockwise);
        platform = move_platform(platform, Orientation::South);

        platform = rotate_90d(platform, Direction::CounterClockwise);
        platform = move_platform(platform, Orientation::East);

        if cache_1.contains_key(&platform) {
            let cycle_init = cache_1.get(&platform).unwrap();
            let delta = cycle - *cycle_init;
            cycles_remainder = (cycles - cycle_init) % delta;
            break;
        } else {
            cache_1.insert(platform.clone(), cycle);
        }
    }

    (platform, cycles_remainder)
}

fn rotate_90d(platform: Vec<String>, direction: Direction) -> Vec<String> {
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
                Some(new_order) => {
                    match direction {
                        Direction::Clockwise => new_order.push_front(element),
                        Direction::CounterClockwise => new_order.push_back(element)
                    }
                }
            }
        }
    }

    match direction {
        Direction::CounterClockwise => platform_rotated.reverse(),
        _ => ()
    }

    platform_rotated.into_iter().map(|c| c.into_iter().collect::<String>()).collect()
}

fn compute_total_load(platform: Vec<String>) -> u32 {
    let mut total_load = 0;

    for elements_per_row in platform {
        let mut current_position = 1;

        for element in elements_per_row.chars() {
            if element == ROUNDED_ROCK {
                total_load += current_position;
            }
            
            current_position += 1;
        }
    }


    total_load
}

fn move_platform(platform: Vec<String>, orientation: Orientation) -> Vec<String> {
    let mut platform_moved = Vec::new();

    let to_reverse = matches!(orientation, Orientation::West | Orientation::South);

    for elements_per_row in platform {
        let iter_elements = if to_reverse { elements_per_row.chars().rev().into_iter().collect::<Vec<_>>() } else { elements_per_row.chars().collect() };

        let mut current_position = 0;
        let mut current_count_rounded_rock = 0;
        let mut rock_moved = String::new();

        for element in iter_elements {
            if element == CUBE_SHAPED_ROCK {
                if current_count_rounded_rock > 0 {
                    rock_moved.push_str(edit_rock_position(current_count_rounded_rock, current_position).as_str());
                    current_count_rounded_rock = 0;
                }
                rock_moved.push_str(CUBE_SHAPED_ROCK.to_string().as_str());
                current_position = 0;
            } else if element == ROUNDED_ROCK {
                current_count_rounded_rock += 1;
                current_position += 1;
            } else {
                if current_count_rounded_rock == 0 {
                    rock_moved.push_str(SPACE.to_string().as_str());
                    current_position = 0;
                } else {
                    current_position += 1;
                }
            }
        }

        if current_count_rounded_rock > 0 {
            rock_moved.push_str(edit_rock_position(current_count_rounded_rock, current_position).as_str());
        }

        if to_reverse {
            rock_moved = rock_moved.chars().rev().into_iter().collect::<String>();
        }

        platform_moved.push(rock_moved);
    }

    platform_moved
}

fn edit_rock_position(count_of_rock: u32, current_position: u32) -> String {
    let space_arrangement = SPACE.to_string().repeat(((current_position) - count_of_rock) as usize);
    let rock_arrangement = ROUNDED_ROCK.to_string().repeat(count_of_rock as usize);
    space_arrangement + &rock_arrangement
}