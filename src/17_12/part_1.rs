use std::{borrow::BorrowMut, collections::HashMap, sync::{Arc, Mutex}, thread, time::Instant};

use AoC_2023::text_file_reader::TextFileReader;

type Cache = HashMap<(usize, usize), u32>;

const MAXIMUM_NUMBER_OF_STEPS_IN_SAME_DIRECTION: u8 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
	Left(char),
	Right(char),
	Up(char),
	Down(char),
}

impl Direction {
    fn new(char: char) -> Self {
        match char {
            '>' => Direction::Right(char),
            '<' => Direction::Left(char),
            '^' => Direction::Up(char),
            'v' => Direction::Down(char),
            _ => panic!("Char {} unexpected", char)
        }
    }

    fn get_char(&self) -> char {
        match self {
            Direction::Left(c) => *c,
            Direction::Right(c) => *c,
            Direction::Up(c) => *c,
            Direction::Down(c) => *c
        }
    }

    fn rotation(self) -> Vec<Direction> {
        match self {
            Direction::Left(_) | Direction::Right(_) => vec![Direction::new('v'), Direction::new('^')],
            Direction::Up(_) | Direction::Down(_) => vec![Direction::new('>'), Direction::new('<')],
        }
    }

    fn get_next_possible_directions(&self, number_of_steps_in_same_direction: u8) -> Vec<Direction> {
        let mut next_possible_directions = if number_of_steps_in_same_direction < MAXIMUM_NUMBER_OF_STEPS_IN_SAME_DIRECTION {
            vec![self.clone()]
        } else {
            Vec::new()
        };

        next_possible_directions.extend(self.clone().rotation());
        next_possible_directions
    }
}

fn main() {
    println!("Puzzle du 17/12 Partie 1");
    let now = Instant::now();
    let least_heat_loss = solve_puzzle(get_puzzle());
    println!("Least heat loss : {least_heat_loss}");
    println!("took: {:?}", now.elapsed());
}

fn solve_puzzle(puzzle: Vec<String>) -> u32 {
    // let map = puzzle.into_iter().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let map = puzzle.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let cache = HashMap::new();
    
    cross_city(
        map,
        (0, 1),
        2,
        0,
        Direction::new('>'),
        Arc::new(Mutex::new(cache))
    )
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("17_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn cross_city(
    map: Vec<Vec<char>>,
    position: (usize, usize),
    number_of_steps_in_same_direction: u8,
    lost_heat: u32,
    actual_direction: Direction,
    arc_cache: Arc<Mutex<Cache>>
) -> u32 {
    if !continue_crossing(position, lost_heat, Arc::clone(&arc_cache)) {
        return u32::MAX;
    }

    let lost_heat = lost_heat + (map[position.0][position.1]).to_digit(10).unwrap();

    if is_destination(&map, position) {
        println!();
        println!("lost heat : {lost_heat}");
        print_map(&map);
        println!();
        println!("===================================================");
        println!();
        return lost_heat;
    } 
    
    let mut next_possible_directions = actual_direction.get_next_possible_directions(number_of_steps_in_same_direction);
    let next_possible_positions = get_next_possible_positions(&map, position, &mut next_possible_directions);
    let next_possible_positions_and_directions = next_possible_positions.into_iter().zip(next_possible_directions.into_iter()).collect::<Vec<_>>();

    let mut least_heat_loss = u32::MAX;
    let mut handles = vec![];

    for (next_position, next_direction) in next_possible_positions_and_directions {
        // let map = map.clone();
        let arc_cache_cloned = Arc::clone(&arc_cache);
        let mut map_edited = map.clone();
        map_edited[position.0][position.1] = next_direction.get_char();
        let number_of_steps_in_same_direction = if next_direction != actual_direction { 1 } else { number_of_steps_in_same_direction + 1 };

        // let a = cross_city(
        //     map_edited,
        //     next_position,
        //     number_of_steps_in_same_direction,
        //     lost_heat,
        //     next_direction,
        //     Arc::clone(&arc_cache)
        // );

        // if a < least_heat_loss {
        //     least_heat_loss = a;
        // }


        let handle = thread::spawn(move || {
            cross_city(
                map_edited,
                next_position,
                number_of_steps_in_same_direction,
                lost_heat,
                next_direction,
                arc_cache_cloned
            )
        });
        handles.push(handle);
    }


    for handle in handles {
        let a = handle.join().unwrap();
        if a < least_heat_loss {
            least_heat_loss = a;
        }
    }

    // if least_heat_loss < u32::MAX {
    //     dbg!(&least_heat_loss);
    //     dbg!(position);
    //     println!("=================================");
    //     println!("=================================");
    //     println!("=================================");
    //     println!("=================================");
    // }

    least_heat_loss
}

fn continue_crossing(
    position: (usize, usize),
    lost_heat: u32,
    arc_cache: Arc<Mutex<Cache>>
) -> bool {
    let mut lock_cache = arc_cache.lock().unwrap();
    let borrow_mut_cache = lock_cache.borrow_mut();

    match borrow_mut_cache.get_mut(&position) {
        Some(actual_lost_heat) => {
            if *actual_lost_heat < lost_heat {
                false
            } else {
                *actual_lost_heat = lost_heat;
                true
            }
        },
        None => {
            borrow_mut_cache.insert(position, lost_heat);
            true
        }
    }
}

fn is_destination(map: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    let height = map.len();
    let lenght = map[0].len();

    position.0 == height - 1 && position.1 == lenght - 1 
}

fn get_next_possible_positions(map: &Vec<Vec<char>>, position: (usize, usize), next_possible_directions: &mut Vec<Direction>) -> Vec<(usize, usize)> {
    let mut next_possible_positions = Vec::new();
    let mut index_direction_impossible = Vec::new();
    
    for (index, direction) in next_possible_directions.iter().enumerate() {
        match direction {
            Direction::Left(_) => {
                match get_position(map, position, (0, -1)) {
                    Some(next_position) => next_possible_positions.push(next_position),
                    None => index_direction_impossible.push(index)
                }
            },
            Direction::Right(_) => {
                match get_position(map, position, (0, 1)) {
                    Some(next_position) => next_possible_positions.push(next_position),
                    None => index_direction_impossible.push(index)
                }
            },
            Direction::Up(_) => {
                match get_position(map, position, (-1, 0)) {
                    Some(next_position) => next_possible_positions.push(next_position),
                    None => index_direction_impossible.push(index)
                }
            },
            Direction::Down(_) => {
                match get_position(map, position, (1, 0)) {
                    Some(next_position) => next_possible_positions.push(next_position),
                    None => index_direction_impossible.push(index)
                }
            },
        }
    }

    if !index_direction_impossible.is_empty() {
        index_direction_impossible.reverse();
        index_direction_impossible.iter().for_each(|index| { next_possible_directions.remove(*index); });
    }

    next_possible_positions
}

fn get_position(map: &Vec<Vec<char>>, position: (usize, usize), shift: (isize, isize)) -> Option<(usize, usize)> {
    if let Some(next_y) = get_next_usize_safely(position.0, shift.0) {
        if next_y < map.len() {
            if let Some(next_x) = get_next_usize_safely(position.1, shift.1) {
                if next_x < map[0].len() {
                    return Some((next_y, next_x));
                }
            }
        }
    }

    None
}

fn get_next_usize_safely(m: usize, shift: isize) -> Option<usize> {
    if shift < 0 {
        let shift_abs = shift.abs();
        m.checked_sub(shift_abs as usize)
    } else {
        m.checked_add(shift as usize)
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    map.iter().for_each(|line| println!("{:?}", line));
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_1() {
        let input = 
        "
            243
            323
            325
        ";
        
        let puzzle = get_puzzle(input);
    
    
        assert_eq!(13, solve_puzzle(puzzle));
    }

    #[test]
    fn test_aoc() {
        let input = 
        "
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        ";
        
        let puzzle = get_puzzle(input);
    
    
        assert_eq!(102, solve_puzzle(puzzle));
    }


    fn get_puzzle(input: &str) -> Vec<String> {
        input.trim().lines().map(|line| line.trim().to_owned()).collect()
    }
}