use std::{collections::HashMap, sync::{Arc, Mutex}, thread};
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use AoC_2023::text_file_reader::TextFileReader;

const VERTICAL_PIPE: char = '|';
const HORIZONTAL_PIPE: char = '-';
const CONNECTING_90_DEGREE_NORTH_AND_EAST_PIPE: char = 'L';
const CONNECTING_90_DEGREE_NORTH_AND_WEST_PIPE: char = 'J';
const CONNECTING_90_DEGREE_SOUTH_AND_WEST_PIPE: char = '7';
const CONNECTING_90_DEGREE_SOUTH_AND_EAST_PIPE: char = 'F';
const GROUND: char = '.';
const STARTING_POINT: char = 'S';

#[derive(Debug, Clone)]
pub struct Path(HashMap<char, (i32, i32)>);

lazy_static! {
    static ref RIGHT: Path = Path(HashMap::from([
        (HORIZONTAL_PIPE, (0, 1)),
        (CONNECTING_90_DEGREE_SOUTH_AND_WEST_PIPE, (1, 0)),
        (CONNECTING_90_DEGREE_NORTH_AND_WEST_PIPE, (-1, 0)),
    ]));

    static ref LEFT: Path = Path(HashMap::from([
        (HORIZONTAL_PIPE, (0, -1)),
        (CONNECTING_90_DEGREE_NORTH_AND_EAST_PIPE, (-1, 0)),
        (CONNECTING_90_DEGREE_SOUTH_AND_EAST_PIPE, (1, 0)),
    ]));

    static ref UP: Path = Path(HashMap::from([
        (CONNECTING_90_DEGREE_SOUTH_AND_WEST_PIPE, (0, -1)),
        (VERTICAL_PIPE, (-1, 0)),
        (CONNECTING_90_DEGREE_SOUTH_AND_EAST_PIPE, (0, 1)),
    ]));

    static ref DOWN: Path = Path(HashMap::from([
        (CONNECTING_90_DEGREE_NORTH_AND_WEST_PIPE, (0, -1)),
        (VERTICAL_PIPE, (1, 0)),
        (CONNECTING_90_DEGREE_NORTH_AND_EAST_PIPE, (0, 1)),
    ]));
}

#[derive(Debug, EnumIter)]
enum Direction {
	RIGHT,
	DOWN,
	LEFT,
	UP,
}

impl Direction {
	pub fn get_vec2(&self) -> (i32, i32) {
		match self {
			Direction::RIGHT => (0, 1),
			Direction::DOWN => (1, 0),
			Direction::LEFT => (0, -1),
			Direction::UP => (-1, 0)
		}
	}

    pub fn get_path(&self) -> &Path {
        match self {
			Direction::RIGHT => &RIGHT,
			Direction::DOWN => &DOWN,
			Direction::LEFT => &LEFT,
			Direction::UP => &UP
		}
    }

    pub fn get_next_direction(&self, pipe: &char) -> Option<Direction> {
        match self {
			Direction::RIGHT => {
                match *pipe {
                    HORIZONTAL_PIPE => Some(Direction::RIGHT),
                    CONNECTING_90_DEGREE_SOUTH_AND_WEST_PIPE => Some(Direction::DOWN),
                    CONNECTING_90_DEGREE_NORTH_AND_WEST_PIPE => Some(Direction::UP),
                    _ => None
                }
            },
			Direction::DOWN => {
                match *pipe {
                    VERTICAL_PIPE => Some(Direction::DOWN),
                    CONNECTING_90_DEGREE_NORTH_AND_WEST_PIPE => Some(Direction::LEFT),
                    CONNECTING_90_DEGREE_NORTH_AND_EAST_PIPE => Some(Direction::RIGHT),
                    _ => None
                }
            },
			Direction::LEFT => {
                match *pipe {
                    HORIZONTAL_PIPE => Some(Direction::LEFT),
                    CONNECTING_90_DEGREE_NORTH_AND_EAST_PIPE => Some(Direction::UP),
                    CONNECTING_90_DEGREE_SOUTH_AND_EAST_PIPE => Some(Direction::DOWN),
                    _ => None
                }
            },
			Direction::UP => {
                match *pipe {
                    CONNECTING_90_DEGREE_SOUTH_AND_WEST_PIPE => Some(Direction::LEFT),
                    VERTICAL_PIPE => Some(Direction::UP),
                    CONNECTING_90_DEGREE_SOUTH_AND_EAST_PIPE => Some(Direction::RIGHT),
                    _ => None
                }
            },
		}
    }
}


fn main() {
    println!("Puzzle du 10/12 Partie 1");
    
    let puzzle = get_puzzle();
    let map = puzzle.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let start_position = puzzle.iter().enumerate().find(|(_, s)| s.contains(STARTING_POINT)).and_then(|(index, s)| Some((index as i32, s.chars().into_iter().position(|c| c == STARTING_POINT).unwrap() as i32))).unwrap();
    
    let step_count_to_furthest_position = get_step_count_to_furthest_position(map, start_position);
    println!("step count to furthest position : {step_count_to_furthest_position}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("10_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_step_count_to_furthest_position(map: Vec<Vec<char>>, start_position: (i32, i32)) -> u32 {
    let mutex_lowest_step_count_by_position = Arc::new(Mutex::new(HashMap::new())); // K, V : (y, x), step 
    let possible_starts = get_possible_starts(&map, &start_position);

    let mut handlers = Vec::new();
    for possible_start in possible_starts {
        let arc_cloned = Arc::clone(&mutex_lowest_step_count_by_position);
        let map = map.clone();
        let handle = thread::spawn(move || {
            find_number_steps_from_furthest_position(arc_cloned, map, possible_start);
        });
        handlers.push(handle);
    
    }
    
    for handle in handlers {
        handle.join().unwrap();
    }
    
    let lowest_step_count_by_position = mutex_lowest_step_count_by_position.lock().unwrap();
    *lowest_step_count_by_position.values().into_iter().max().unwrap()
}

fn get_possible_starts(map: &Vec<Vec<char>>, start_position: &(i32, i32)) -> Vec<((i32, i32), Direction)> {
    let mut possible_paths = Vec::new();

    for direction in Direction::iter() {
        let vec2 = direction.get_vec2();
        let next_position = (start_position.0 + vec2.0, start_position.1 + vec2.1);
        
        let next_tile = match map.get(next_position.0 as usize) {
            None => continue,
            Some(tiles) => {
                match tiles.get(next_position.1 as usize) {
                    None => continue,
                    Some(tile) => tile
                }
            }
        };

        if *next_tile == STARTING_POINT || *next_tile == GROUND {
            continue;
        }

        possible_paths.push((next_position, direction));

    }


    possible_paths
}

fn find_number_steps_from_furthest_position(arc_lowest_step_count_by_position: Arc<Mutex<HashMap<(i32, i32), u32>>>, map: Vec<Vec<char>>, mut position: ((i32, i32), Direction)) {
    let mut current_step_count = 0;

    loop {
        let next_step = get_possible_next_step(&map, position.0, position.1);

        match &next_step {
            None => break,
            Some(v) => {
                current_step_count += 1;
                let mut lowest_step_count_by_position = arc_lowest_step_count_by_position.lock().unwrap();
                if let Some(step) = lowest_step_count_by_position.get(&v.0) {
                    if *step < current_step_count {
                        drop(lowest_step_count_by_position);
                        break;
                    }
                }
                lowest_step_count_by_position.insert(position.0, current_step_count);
                drop(lowest_step_count_by_position);
            }
        }
        position = next_step.unwrap();
    }    
}

fn get_possible_next_step(map: &Vec<Vec<char>>, position: (i32, i32), direction: Direction) -> Option<((i32, i32), Direction)> {
    let pipe = map.get(position.0 as usize).unwrap().get(position.1 as usize).unwrap();
    let path = direction.get_path();
    let next_direction = direction.get_next_direction(pipe);
    let shift = path.0.get(pipe);

    if next_direction.is_none() || shift.is_none() {
        None
    } else {
        let shift = shift.unwrap();
        let next_position = (position.0 + shift.0, position.1 + shift.1);

        if let Some(tiles) = map.get(next_position.0 as usize) {
            if tiles.get(next_position.1 as usize).is_none() {
                return None;
            }
        } else {
            return None;
        };

        Some((next_position, next_direction.unwrap()))
    }
}