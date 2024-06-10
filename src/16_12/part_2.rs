use std::{borrow::{Borrow, BorrowMut}, collections::HashSet, sync::{Arc, Mutex}, thread, time::Instant};

use itertools::Itertools;
use AoC_2023::text_file_reader::TextFileReader;

const LEFT_MIRROR: char = '\\';
const RIGHT_MIRROR: char = '/';
const HORIZONTAL_SPLITTER: char = '-';
const VERTICAL_SPLITTER: char = '|';
const EMPTY: char = '.';
const SHIFT: usize = 1;


#[derive(Debug, Clone)]
pub enum Tile {
	LeftMirror(char),
	RightMirror(char),
	HorizontalSplitter(char),
	VerticalSplitter(char),
	Empty(char),
}

impl Tile {
    fn new(char: char) -> Self {
        match char {
            LEFT_MIRROR => Tile::LeftMirror(char),
            RIGHT_MIRROR => Tile::RightMirror(char),
            HORIZONTAL_SPLITTER => Tile::HorizontalSplitter(char),
            VERTICAL_SPLITTER => Tile::VerticalSplitter(char),
            EMPTY => Tile::Empty(char),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
	Left,
	Right,
	Up,
	Down,
}

impl Direction {
    fn next_from_tile(self, tile: &Tile) -> Vec<Direction> {
        let mut next_directions = Vec::new();
        match tile {
            Tile::Empty(_) => next_directions.push(self),
            Tile::LeftMirror(_) => {
                match self {
                    Direction::Left => next_directions.push(Direction::Up),
                    Direction::Right => next_directions.push(Direction::Down),
                    Direction::Up => next_directions.push(Direction::Left),
                    Direction::Down => next_directions.push(Direction::Right),
                }
            },
            Tile::RightMirror(_) => {
                match self {
                    Direction::Left => next_directions.push(Direction::Down),
                    Direction::Right => next_directions.push(Direction::Up),
                    Direction::Up => next_directions.push(Direction::Right),
                    Direction::Down => next_directions.push(Direction::Left),
                }
            },
            Tile::HorizontalSplitter(_) => {
                match self {
                    Direction::Left | Direction::Right => next_directions.push(self),
                    Direction::Up | Direction::Down => {
                        next_directions.push(Direction::Left);
                        next_directions.push(Direction::Right);
                    }
                }
            },
            Tile::VerticalSplitter(_) => {
                match self {
                    Direction::Up | Direction::Down => next_directions.push(self),
                    Direction::Left | Direction::Right => {
                        next_directions.push(Direction::Up);
                        next_directions.push(Direction::Down);
                    }
                }
            }
        }

        next_directions
    }
}


fn main() {
    println!("Puzzle du 16/12 Partie 2");
    let now = Instant::now();
    
    let map = get_puzzle().into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect_vec();
    let (height, lenght) = get_height_and_length_map(&map);
    let corners = get_corners(height, lenght);
    
    let energized_tile_count = count_tile_energized_from_differents_starts(map, height, lenght, corners);
    println!("Energized tiles : {energized_tile_count}");
    println!("took: {:?}", now.elapsed());
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("16_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_height_and_length_map(map: &Vec<Vec<char>>) -> (usize, usize) {
    let height = map.len();
    let lenght = map.get(0).unwrap().len();
    (height, lenght)
}

fn get_corners(height: usize, lenght: usize) -> Vec<(usize, usize)> {
    let mut corners = Vec::new();
    corners.push((0,0));
    corners.push((height - 1,0));
    corners.push((0,lenght - 1));
    corners.push((height - 1, lenght - 1));

    corners
}

fn count_tile_energized_from_differents_starts(
    map: Vec<Vec<char>>,
    height: usize,
    lenght: usize,
    corners: Vec<(usize, usize)>
) -> usize {
    let mut max = 0;
    
    for i in 0..2 {
        let mut start = if i == 0 { (0, 0) } else { (0, lenght - 1) };
        let direction = if i == 0 { Direction::Right } else { Direction::Left };
        

        for y in 1..height {
            start.0 = y;

            let energized_tile_count = count_tile_energized(&map, start, &direction);
            if energized_tile_count > max {
                max = energized_tile_count;
            }
        }
    }

    for i in 0..2 {
        let mut start = if i == 0 { (0, 0) } else { (height - 1, 0) };
        let direction = if i == 0 { Direction::Down } else { Direction::Up };
        

        for x in 1..lenght {
            start.1 = x;

            let energized_tile_count = count_tile_energized(&map, start, &direction);
            if energized_tile_count > max {
                max = energized_tile_count;
            }
        }
    }
    
    max
}

fn count_tile_energized(map: &Vec<Vec<char>>, start: (usize, usize), direction: &Direction) -> usize {
    let energized_tiles_coordinates = HashSet::new();
    let energized_tiles_coordinates_with_direction = HashSet::new();

    let arc_energized_tiles = Arc::new(Mutex::new((energized_tiles_coordinates, energized_tiles_coordinates_with_direction)));
    cross_the_map(
        &map,
        Arc::clone(&arc_energized_tiles),
        (Some(start.0), Some(start.1)),
        direction.clone()
    );

    let lock_energized_tiles = arc_energized_tiles.lock().unwrap();

    lock_energized_tiles.0.len()
}

fn cross_the_map(
    map: &Vec<Vec<char>>,
    arc_energized_tiles: Arc<Mutex<(HashSet<(usize, usize)>,HashSet<(usize, usize, Direction)>)>>,
    next_position: (Option<usize>, Option<usize>),
    direction: Direction
) {
    if let Some(next_position) = check_next_position(next_position) {
        if let Some(tile) = get_next_tile(map, next_position) {
            let mut lock = arc_energized_tiles.lock().unwrap();
            {
                let lock_energized_tiles = lock.borrow_mut();
                
                {
                    if lock_energized_tiles.1.borrow().contains(&(next_position.0, next_position.1, direction.clone())) {
                        return;
                    }
                };
                
                lock_energized_tiles.0.borrow_mut().insert(next_position);
                lock_energized_tiles.1.borrow_mut().insert((next_position.0, next_position.1, direction.clone()));
            }
            drop(lock);
    
    
            let next_directions = get_next_directions(tile, direction);
            
            if next_directions.len() == 1 {
                let next_direction = next_directions.get(0).unwrap();
                let next_position = get_next_position(next_position, next_direction);
                cross_the_map(
                    map,
                    arc_energized_tiles,
                    next_position,
                    next_direction.clone()
                );
            } else {
                let mut handles = vec![];

                for next_direction in next_directions {
                    let next_position = get_next_position(next_position, &next_direction);
                    let arc_energized_tiles_cloned = Arc::clone(&arc_energized_tiles);
                    let map = map.clone();


                    let handle = thread::spawn(move || {
                        cross_the_map(&map, arc_energized_tiles_cloned, next_position, next_direction.clone());
                    });

                    handles.push(handle);
                }

                for handle in handles {
                    handle.join().unwrap();
                }
            }
        }
    }
}

fn check_next_position(next_position: (Option<usize>, Option<usize>)) -> Option<(usize, usize)> {
    match next_position.0 {
        None => None,
        Some(y) => {
            match next_position.1 {
                None => None,
                Some(x) => Some((y, x))
            }
        }
    }
}

fn get_next_tile(map: &Vec<Vec<char>>, next_position: (usize, usize)) -> Option<char> {
    match map.get(next_position.0) {
        None => None,
        Some(tiles) => {
            match tiles.get(next_position.1) {
                None => None,
                Some(tile) => Some(*tile)
            }
        }
    }
}

fn get_next_directions(tile: char, direction: Direction) -> Vec<Direction> {
    let tile = Tile::new(tile);
    direction.next_from_tile(&tile)
}

fn get_next_position(position: (usize, usize), next_direction: &Direction) -> (Option<usize>, Option<usize>) {
    match next_direction {
        Direction::Left => (Some(position.0), position.1.checked_sub(SHIFT)),
        Direction::Right => (Some(position.0), Some(position.1 + SHIFT)),
        Direction::Up => (position.0.checked_sub(SHIFT), Some(position.1)),
        Direction::Down => (Some(position.0 + SHIFT), Some(position.1)),
    }
}