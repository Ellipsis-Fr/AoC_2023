use std::{borrow::{Borrow, BorrowMut}, collections::{HashMap, HashSet}, sync::{Arc, Mutex}, thread, time::Instant};

use num::pow;
use num_integer::sqrt;
use AoC_2023::text_file_reader::TextFileReader;

type CacheBefore = HashMap<(usize, usize, Direction, u8), u32>;
type CacheNext = HashMap<(usize, usize), HashMap<Direction, HashMap<u8, HashMap<Direction, HashMap<u8, u32>>>>>;

const MAXIMUM_NUMBER_OF_STEPS_IN_SAME_DIRECTION: u8 = 3;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        let mut next_possible_directions = match self {
            Direction::Right(_) | Direction::Down(_) => {
                if number_of_steps_in_same_direction < MAXIMUM_NUMBER_OF_STEPS_IN_SAME_DIRECTION {
                    vec![self.clone()]
                } else {
                    Vec::new()
                }
            },
            _ => Vec::new()
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

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("17_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn solve_puzzle(puzzle: Vec<String>) -> u32 {
    // let map = puzzle.into_iter().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let map = puzzle.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let destination = get_destination(&map);
    
    let cache_before = CacheBefore::new();
    let cache_next = CacheNext::new();
    let block_visited = HashSet::new();
    
    // let least_heat_loss_known = get_least_heat_loss_quickly(
    //     &map,
    //     destination,
    //     (0, 1),
    //     2,
    //     Direction::new('>'),
    // );
    // let arc_least_heat_loss_known = Arc::new(Mutex::new(least_heat_loss_known));
    let arc_least_heat_loss_known = Arc::new(Mutex::new(800));
    
    cross_city(
        map,
        destination,
        (0, 1),
        2,
        0,
        Direction::new('>'),
        Arc::new(Mutex::new(cache_before)),
        Arc::new(Mutex::new(cache_next)),
        block_visited,
        Arc::clone(&arc_least_heat_loss_known)
    )
}

fn get_destination(map: &Vec<Vec<char>>) -> (usize, usize) {
    let height = map.len();
    let lenght = map[0].len();

    (height - 1, lenght - 1) 
}

fn get_least_heat_loss_quickly(
    map: &Vec<Vec<char>>,
    destination: (usize, usize),
    position: (usize, usize),
    number_of_steps_in_same_direction: u8,
    actual_direction: Direction,
) -> u32 {

    let lost_heat = (map[position.0][position.1]).to_digit(10).unwrap();

    if position == destination {
        return lost_heat;
    } 

    let mut next_possible_directions = actual_direction.get_next_possible_directions(number_of_steps_in_same_direction);
    let next_possible_positions = get_next_possible_positions(&map, position, &mut next_possible_directions);
    let next_possible_positions_and_directions = next_possible_positions.into_iter().zip(next_possible_directions.into_iter()).collect::<Vec<_>>();
    let (next_position, next_direction) = get_next_position_and_direction(&actual_direction, next_possible_positions_and_directions);
    let number_of_steps_in_same_direction = if next_direction != actual_direction { 1 } else { number_of_steps_in_same_direction + 1 };


    lost_heat + get_least_heat_loss_quickly(
        map,
        destination,
        next_position,
        number_of_steps_in_same_direction,
        next_direction
    )
}

fn cross_city(
    mut map: Vec<Vec<char>>,
    destination: (usize, usize),
    position: (usize, usize),
    number_of_steps_in_same_direction: u8,
    lost_heat: u32,
    actual_direction: Direction,
    arc_cache_before: Arc<Mutex<CacheBefore>>,
    arc_cache_next: Arc<Mutex<CacheNext>>,
    mut block_visited: HashSet<(usize, usize)>,
    arc_least_heat_loss_known: Arc<Mutex<u32>>
) -> u32 {
    if !block_visited.insert(position.clone()) {
        return u32::MAX;
    }

    if !continue_crossing(position, number_of_steps_in_same_direction, lost_heat, &actual_direction, Arc::clone(&arc_cache_before)) {
        return u32::MAX;
    }

    // dbg!(&position);
    // let case = (map[position.0][position.1]).to_digit(10).unwrap();
    let lost_heat = lost_heat + (map[position.0][position.1]).to_digit(10).unwrap();
    if lost_too_much_heat(lost_heat, destination, position, Arc::clone(&arc_least_heat_loss_known)) {
        // println!("lost heat too much {lost_heat} at {:?}", position);
        return u32::MAX;
    }

    // if position == (1, 5) && lost_heat == 20 {
    //     println!("Lost heatà (1,5) : {lost_heat}");
    // }

    if position == destination {
        // if position == (12, 11) {
        //     println!("Lost heatà (12,11) : {lost_heat}");
        // }
        println!();
        println!("lost heat : {lost_heat}");
        print_map(&map);
        println!();
        println!("===================================================");
        println!();
        println!("least lost heat = {lost_heat}");
        let mut lock_least_heat_loss_known = arc_least_heat_loss_known.lock().unwrap();
        let borrow_mut_least_heat_loss_known = lock_least_heat_loss_known.borrow_mut();
        if **borrow_mut_least_heat_loss_known > lost_heat {
            **borrow_mut_least_heat_loss_known = lost_heat;
        }
        return lost_heat;
    } 

    map[position.0][position.1] = actual_direction.get_char();
    
    let mut next_possible_directions = actual_direction.get_next_possible_directions(number_of_steps_in_same_direction);
    let next_possible_positions = get_next_possible_positions(&map, position, &mut next_possible_directions);
    let next_possible_positions_and_directions = next_possible_positions.into_iter().zip(next_possible_directions.into_iter()).collect::<Vec<_>>();

    let mut lost_heat_here = u32::MAX;
    // let mut handles = vec![];

    for (next_position, next_direction) in next_possible_positions_and_directions {
        let number_of_steps_in_same_direction_next = if next_direction != actual_direction { 1 } else { number_of_steps_in_same_direction + 1 };

        let mut lock_cache = arc_cache_next.lock().unwrap();
        let borrow_mut_cache = lock_cache.borrow_mut();

        // if position == (12, 11) {
        //     println!("Lost heatà (12,11) : {lost_heat}");
        // }

        if let Some(lost_heat_by_actual_direction) = borrow_mut_cache.get_mut(&position) {
            if let Some(lost_heat_by_number_of_steps_for_actual_direction) = lost_heat_by_actual_direction.get_mut(&actual_direction) {
                if let Some(lost_heat_by_next_direction) = lost_heat_by_number_of_steps_for_actual_direction.get_mut(&number_of_steps_in_same_direction) {
                    if let Some(lost_heat_by_number_of_steps_for_next_direction) = lost_heat_by_next_direction.get_mut(&next_direction) {
                        if let Some(lost_heat_next_known) = lost_heat_by_number_of_steps_for_next_direction.get_mut(&number_of_steps_in_same_direction_next) {
                            let final_lost_heat = *lost_heat_next_known + lost_heat;
                            if final_lost_heat < lost_heat_here {
                                lost_heat_here = final_lost_heat;
                            }
        
                            let mut lock_least_heat_loss_known = arc_least_heat_loss_known.lock().unwrap();
                            let borrow_mut_least_heat_loss_known = lock_least_heat_loss_known.borrow_mut();
                            if final_lost_heat < **borrow_mut_least_heat_loss_known {
                                **borrow_mut_least_heat_loss_known = final_lost_heat;
                            }
        
                            continue;
                        }
                    }
                }
            }
        }
        
        drop(lock_cache);


        let map = map.clone();
        let arc_cache_before_cloned = Arc::clone(&arc_cache_before);
        let arc_cache_next_cloned = Arc::clone(&arc_cache_next);
        let block_visited_cloned = block_visited.clone();
        let arc_least_heat_loss_known_cloned = Arc::clone(&arc_least_heat_loss_known);

        let final_lost_heat = cross_city(
            map,
            destination,
            next_position,
            number_of_steps_in_same_direction_next,
            lost_heat,
            next_direction.clone(),
            arc_cache_before_cloned,
            arc_cache_next_cloned,
            block_visited_cloned,
            arc_least_heat_loss_known_cloned
        );

        if final_lost_heat != u32::MAX {
            if final_lost_heat < lost_heat_here {
                lost_heat_here = final_lost_heat;
            }
            let mut lock_cache = arc_cache_next.lock().unwrap();
            let borrow_mut_cache = lock_cache.borrow_mut();
            
            // println!("{final_lost_heat} - {lost_heat} - {case}");
            let new_lost_heat_next = final_lost_heat - lost_heat ;

            match borrow_mut_cache.get_mut(&position) {
                Some(lost_heat_by_actual_direction) => {
                    match lost_heat_by_actual_direction.get_mut(&actual_direction) {
                        Some(lost_heat_by_number_of_steps_for_actual_direction) => {
                            match lost_heat_by_number_of_steps_for_actual_direction.get_mut(&number_of_steps_in_same_direction) {
                                Some(lost_heat_by_next_direction) => {
                                    match lost_heat_by_next_direction.get_mut(&next_direction) {
                                        Some(lost_heat_by_number_of_steps) => {
                                            match lost_heat_by_number_of_steps.get_mut(&number_of_steps_in_same_direction_next) {
                                                Some(lost_heat_next_known) => {
                                                    if new_lost_heat_next < *lost_heat_next_known {
                                                        *lost_heat_next_known = new_lost_heat_next;
                                                    }
                                                },
                                                None => {
                                                    lost_heat_by_number_of_steps.insert(number_of_steps_in_same_direction_next, new_lost_heat_next);
                                                },
                                            }
                                        },
                                        None => {
                                            // if position == (1, 5) {
                                            //     println!("Lost heatà (1,5) : {lost_heat}");
                                            // }
                                            let lost_heat_by_number_of_steps = HashMap::from([(number_of_steps_in_same_direction_next, new_lost_heat_next)]);
                                            lost_heat_by_next_direction.insert(next_direction, lost_heat_by_number_of_steps);
                                        },
                                    }
                                },
                                None => {
                                    let lost_heat_by_number_of_steps = HashMap::from([(number_of_steps_in_same_direction_next, new_lost_heat_next)]);
                                    let lost_heat_by_next_direction = HashMap::from([(next_direction, lost_heat_by_number_of_steps)]);
                                    lost_heat_by_number_of_steps_for_actual_direction.insert(number_of_steps_in_same_direction, lost_heat_by_next_direction);
                                },
                            }
                        },
                        None => {
                            let lost_heat_by_number_of_steps = HashMap::from([(number_of_steps_in_same_direction_next, new_lost_heat_next)]);
                            let lost_heat_by_next_direction = HashMap::from([(next_direction, lost_heat_by_number_of_steps)]);
                            let lost_heat_by_number_of_steps_for_actual_direction = HashMap::from([(number_of_steps_in_same_direction, lost_heat_by_next_direction)]);
                            lost_heat_by_actual_direction.insert(actual_direction.clone(), lost_heat_by_number_of_steps_for_actual_direction);
                        }
                    }
                },
                None => {
                    // if position == (1, 5) {
                    //     println!("Lost heatà (1,5) : {lost_heat}");
                    // }

                    let lost_heat_by_number_of_steps = HashMap::from([(number_of_steps_in_same_direction_next, new_lost_heat_next)]);
                    let lost_heat_by_next_direction = HashMap::from([(next_direction, lost_heat_by_number_of_steps)]);
                    let lost_heat_by_number_of_steps_for_actual_direction = HashMap::from([(number_of_steps_in_same_direction, lost_heat_by_next_direction)]);
                    let lost_heat_by_actual_direction = HashMap::from([(actual_direction.clone(), lost_heat_by_number_of_steps_for_actual_direction)]);
                    borrow_mut_cache.insert(position, lost_heat_by_actual_direction);
                }
            }
        }


        // let handle = thread::spawn(move || {
        //     cross_city(
        //         map,
        //         destination,
        //         next_position,
        //         number_of_steps_in_same_direction,
        //         lost_heat,
        //         next_direction,
        //         arc_cache_before_cloned,
        //         block_visited_cloned,
        //         arc_least_heat_loss_known_cloned
        //     );
        // });
        // handles.push(handle);
    }
    
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    lost_heat_here

}

fn continue_crossing(
    position: (usize, usize),
    number_of_steps_in_same_direction: u8,
    lost_heat: u32,
    actual_direction: &Direction,
    arc_cache_before: Arc<Mutex<CacheBefore>>
) -> bool {
    let mut lock_cache = arc_cache_before.lock().unwrap();
    let borrow_mut_cache = lock_cache.borrow_mut();
    let key = (position.0, position.1, actual_direction.clone(), number_of_steps_in_same_direction);

    match borrow_mut_cache.get_mut(&key) {
        Some(actual_lost_heat) => {
            if *actual_lost_heat < lost_heat {
                false
            } else {
                *actual_lost_heat = lost_heat;
                true
            }
        },
        None => {
            borrow_mut_cache.insert(key, lost_heat);
            true
        }
    }
}

fn lost_too_much_heat(lost_heat: u32, destination: (usize, usize), position: (usize, usize), arc_least_heat_loss_known: Arc<Mutex<u32>>) -> bool {
    let lock_least_heat_loss_known = arc_least_heat_loss_known.lock().unwrap();
    let borrow_least_heat_loss_known = lock_least_heat_loss_known.borrow();

    if lost_heat >= **borrow_least_heat_loss_known || lost_heat + get_manhattan_distance(destination, position) as u32 >= **borrow_least_heat_loss_known {
        // println!("{borrow_least_heat_loss_known}");
        true
    } else { false }
}

fn get_manhattan_distance(destination: (usize, usize), position: (usize, usize)) -> usize {
    sqrt(pow(destination.0 - position.0, 2)) + sqrt(pow(destination.1 - position.1, 2))
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

fn get_next_position_and_direction(actual_direction: &Direction, next_possible_positions_and_directions:  Vec<((usize, usize), Direction)>) -> ((usize, usize), Direction) {
    let mut next_position_and_direction = None;

    for (position, direction) in next_possible_positions_and_directions {
        if *actual_direction == direction {
            next_position_and_direction = Some((position, direction));
            break;
        }

        match direction {
            // actual_direction = Direction::Down(_)
            Direction::Right(_) => next_position_and_direction = Some((position, direction)),
            Direction::Left(_) => {
                if next_position_and_direction.is_none() {
                    next_position_and_direction = Some((position, direction));
                }
            },
            // actual_direction = Direction::Right(_)
            Direction::Down(_) => next_position_and_direction = Some((position, direction)),
            Direction::Up(_) => {
                if next_position_and_direction.is_none() {
                    next_position_and_direction = Some((position, direction));
                }
            },
        }
    }

    next_position_and_direction.unwrap()
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