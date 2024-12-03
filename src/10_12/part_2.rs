use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, ops::Range, rc::Rc, sync::{Arc, Mutex}, thread};
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

#[derive(Debug, PartialEq, EnumIter, Clone)]
pub enum Direction {
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

    pub fn get_origin_direction(&self) -> Direction {
        match self {
			Direction::RIGHT => Direction::LEFT,
			Direction::DOWN => Direction::UP,
			Direction::LEFT => Direction::RIGHT,
			Direction::UP => Direction::DOWN,
		}
    }

    pub fn get_next_direction_based_on_rotation(&self, rotation_sense: &RotationSense) -> Direction {
        match self {
			Direction::RIGHT => {
                match rotation_sense {
                    RotationSense::CLOCKWISE => Direction::DOWN,
                    RotationSense::COUNTERCLOCKWISE => Direction::UP
                }
            },
			Direction::DOWN => {
                match rotation_sense {
                    RotationSense::CLOCKWISE => Direction::LEFT,
                    RotationSense::COUNTERCLOCKWISE => Direction::RIGHT
                }
            },
			Direction::LEFT => {
                match rotation_sense {
                    RotationSense::CLOCKWISE => Direction::UP,
                    RotationSense::COUNTERCLOCKWISE => Direction::DOWN
                }
            },
			Direction::UP => {
                match rotation_sense {
                    RotationSense::CLOCKWISE => Direction::RIGHT,
                    RotationSense::COUNTERCLOCKWISE => Direction::LEFT
                }
            },
		}
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RotationSense {
    CLOCKWISE, COUNTERCLOCKWISE
}

#[derive(Debug, Clone, PartialEq)]
pub enum KindPipe {
    LINE,
    BEND(RotationSense)
}

// const CONNECTING_90_DEGREE_NORTH_AND_EAST_PIPE: char = 'L';
// const CONNECTING_90_DEGREE_NORTH_AND_WEST_PIPE: char = 'J';
// const CONNECTING_90_DEGREE_SOUTH_AND_WEST_PIPE: char = '7';
// const CONNECTING_90_DEGREE_SOUTH_AND_EAST_PIPE: char = 'F';

impl KindPipe {
    pub fn new(pipe_definition: char, entrance: &Direction) -> KindPipe {
        match pipe_definition {
            VERTICAL_PIPE | HORIZONTAL_PIPE => KindPipe::LINE,
            angle => {
                
                let rotation_sense = match entrance {
                    Direction::LEFT => {
                        if angle == CONNECTING_90_DEGREE_NORTH_AND_WEST_PIPE {
                            RotationSense::COUNTERCLOCKWISE
                        } else if angle == CONNECTING_90_DEGREE_SOUTH_AND_WEST_PIPE {
                            RotationSense::CLOCKWISE
                        } else {
                            panic!("inlet direction inconsistent with pipe");
                        }
                    },
                    Direction::UP => {
                        if angle == CONNECTING_90_DEGREE_NORTH_AND_EAST_PIPE {
                            RotationSense::COUNTERCLOCKWISE
                        } else if angle == CONNECTING_90_DEGREE_NORTH_AND_WEST_PIPE {
                            RotationSense::CLOCKWISE
                        } else {
                            panic!("inlet direction inconsistent with pipe");
                        }
                    },
                    Direction::RIGHT => {
                        if angle == CONNECTING_90_DEGREE_SOUTH_AND_EAST_PIPE {
                            RotationSense::COUNTERCLOCKWISE
                        } else if angle == CONNECTING_90_DEGREE_NORTH_AND_EAST_PIPE {
                            RotationSense::CLOCKWISE
                        } else {
                            panic!("inlet direction inconsistent with pipe");
                        }
                    },
                    Direction::DOWN => {
                        if angle == CONNECTING_90_DEGREE_SOUTH_AND_WEST_PIPE {
                            RotationSense::COUNTERCLOCKWISE
                        } else if angle == CONNECTING_90_DEGREE_SOUTH_AND_EAST_PIPE {
                            RotationSense::CLOCKWISE
                        } else {
                            panic!("inlet direction inconsistent with pipe");
                        }
                    }
                };

                KindPipe::BEND(rotation_sense)
            }
        }
    }

    pub fn new_without_pipe(entrance: &Direction, exit: &Direction) -> KindPipe {
        match entrance {
            Direction::LEFT => {
                match exit {
                    Direction::RIGHT => KindPipe::LINE,
                    Direction::UP => KindPipe::BEND(RotationSense::COUNTERCLOCKWISE),
                    Direction::DOWN => KindPipe::BEND(RotationSense::CLOCKWISE),
                    _ => panic!("inlet direction inconsistent with pipe")
                }
            },
            Direction::RIGHT => {
                match exit {
                    Direction::LEFT => KindPipe::LINE,
                    Direction::UP => KindPipe::BEND(RotationSense::CLOCKWISE),
                    Direction::DOWN => KindPipe::BEND(RotationSense::COUNTERCLOCKWISE),
                    _ => panic!("inlet direction inconsistent with pipe")
                }
            },
            Direction::UP => {
                match exit {
                    Direction::DOWN => KindPipe::LINE,
                    Direction::LEFT => KindPipe::BEND(RotationSense::COUNTERCLOCKWISE),
                    Direction::RIGHT => KindPipe::BEND(RotationSense::CLOCKWISE),
                    _ => panic!("inlet direction inconsistent with pipe")
                }
            },
            Direction::DOWN => {
                match exit {
                    Direction::UP => KindPipe::LINE,
                    Direction::LEFT => KindPipe::BEND(RotationSense::CLOCKWISE),
                    Direction::RIGHT => KindPipe::BEND(RotationSense::COUNTERCLOCKWISE),
                    _ => panic!("inlet direction inconsistent with pipe")
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pipe {
    pub kind: KindPipe,
    pub entrance: Direction,
    pub exit: Direction,
    pub inside_sides: Vec<Direction>
}

impl Pipe {
    pub fn new(pipe_definition: char, entrance: Direction, exit: Direction) -> Pipe {
        let kind = KindPipe::new(pipe_definition, &entrance);

        Pipe { kind, entrance, exit, inside_sides: Vec::new() }
    }

    pub fn new_without_pipe(entrance: Direction, exit: Direction) -> Pipe {
        let kind = KindPipe::new_without_pipe(&entrance, &exit);

        Pipe { kind, entrance, exit, inside_sides: Vec::new() }
    }

    // fn init_inside_sides(rotation_sense: &RotationSense, entrance: &Direction, exit: &Direction) -> Vec<Direction> {
    //     let mut inside_sides = Vec::new();

    //     inside_sides.push(exit.get_next_direction_based_on_rotation(rotation_sense));
    //     let rotation_sense = match rotation_sense {
    //         RotationSense::CLOCKWISE => &RotationSense::COUNTERCLOCKWISE,
    //         RotationSense::COUNTERCLOCKWISE => &RotationSense::CLOCKWISE
    //     };
    //     inside_sides.push(entrance.get_next_direction_based_on_rotation(rotation_sense));

    //     inside_sides
    // }

    pub fn set_inside_sides(&mut self, rotation_sense: &RotationSense) {
        self.inside_sides.push(self.exit.get_next_direction_based_on_rotation(rotation_sense));
        
        if self.kind != KindPipe::LINE {
            let rotation_sense = match rotation_sense {
                RotationSense::CLOCKWISE => &RotationSense::COUNTERCLOCKWISE,
                RotationSense::COUNTERCLOCKWISE => &RotationSense::CLOCKWISE
            };
            self.inside_sides.push(self.entrance.get_next_direction_based_on_rotation(rotation_sense));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrackingMap {
    ROWS(HashMap<u32, Range<u32>>),
    COLUMNS(HashMap<u32, Range<u32>>)
}

fn main() {
    println!("Puzzle du 10/12 Partie 2");
    
    let puzzle = get_puzzle();
    let map = puzzle.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let start_position = puzzle.iter().enumerate().find(|(_, s)| s.contains(STARTING_POINT)).and_then(|(index, s)| Some((index as i32, s.chars().into_iter().position(|c| c == STARTING_POINT).unwrap() as i32))).unwrap();
    
    let circle = get_circle(&map, start_position);

    let count_enclosed_tiles = get_count_enclosed_tiles(map, circle);
    println!("Enclosed tiles : {:?}", count_enclosed_tiles);


    
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("10_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_circle(map: &Vec<Vec<char>>, start_position: (i32, i32)) -> HashMap::<(i32, i32), Pipe> {
    let (mut semi_circle_1, mut semi_circle_2) = get_semi_circles(&map, start_position);
    reduces_duplicates(&mut semi_circle_1, &mut semi_circle_2);

    let (rotation_sense_semi_circle_1, rotation_sense_semi_circle_2) = get_rotation_sense_by_semi_circle(semi_circle_1.values().collect(), semi_circle_2.values().collect());
    define_inner_side(semi_circle_1.values_mut().collect(), rotation_sense_semi_circle_1, semi_circle_2.values_mut().collect(), rotation_sense_semi_circle_2);
    semi_circle_1.extend(semi_circle_2);
    semi_circle_1
}

fn get_semi_circles(map: &Vec<Vec<char>>, start_position: (i32, i32)) -> (HashMap::<(i32, i32), Pipe>, HashMap::<(i32, i32), Pipe>) {
    let mutex_lowest_step_count_by_position = Arc::new(Mutex::new(HashMap::new())); // K, V : (y, x), step 
    let possible_starts = get_possible_starts(&map, start_position);
    let first_pipe = guess_first_pipe(&possible_starts); // je récupère la direction opposée au path 1, donc à ajouter à path 2
    // println!("FIRST PIPE");
    // println!("{:?}", first_pipe);
    // println!("=========================");
    // println!("=========================");
    
    let mut semi_circle_1 = HashMap::new();
    let mut semi_circle_2 = HashMap::new();
    semi_circle_2.insert(start_position, first_pipe);

    let (arc_semi_circle_1, arc_semi_circle_2) = (Arc::new(Mutex::new(semi_circle_1)), Arc::new(Mutex::new(semi_circle_2))); // K, V : (y, x), Pipe

    let mut handlers = Vec::new();
    for (index, possible_start) in possible_starts.into_iter().enumerate() {
        let arc_cloned = Arc::clone(&mutex_lowest_step_count_by_position);
        let arc_semi_circle = if index == 0 {
            Arc::clone(&arc_semi_circle_1)
        } else {
            Arc::clone(&arc_semi_circle_2)
        };

        let map = map.clone();
        let handle = thread::spawn(move || {
            fill_semi_circles(arc_cloned, arc_semi_circle, map, possible_start);
        });
        handlers.push(handle);
    
    }
    
    for handle in handlers {
        handle.join().unwrap();
    }

    semi_circle_1 = arc_semi_circle_1.lock().unwrap().clone();
    semi_circle_2 = arc_semi_circle_2.lock().unwrap().clone();
    (semi_circle_1, semi_circle_2)
}

fn get_possible_starts(map: &Vec<Vec<char>>, start_position: (i32, i32)) -> Vec<((i32, i32), Direction)> {
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

fn guess_first_pipe(possible_starts: &Vec<((i32, i32), Direction)>) -> Pipe {
    let (_, direction_1) = possible_starts.get(0).unwrap();
    let (_, direction_2) = possible_starts.get(1).unwrap();

    let entrance = direction_1.clone();
    let exit = direction_2.clone();

    Pipe::new_without_pipe(entrance, exit)

}

fn fill_semi_circles(
    arc_lowest_step_count_by_position: Arc<Mutex<HashMap<(i32, i32), u32>>>,
    arc_semi_circle: Arc<Mutex<HashMap<(i32, i32), Pipe>>>,
    map: Vec<Vec<char>>,
    mut position_and_direction: ((i32, i32), Direction)
) {
    let mut current_step_count = 0;

    loop {
        let (position, direction) = position_and_direction;
        let next_step = get_possible_next_step(&map, position, &direction);

        match &next_step {
            None => break,
            Some((next_position, next_direction)) => {
                current_step_count += 1;
                let mut lowest_step_count_by_position = arc_lowest_step_count_by_position.lock().unwrap();
                if let Some(step) = lowest_step_count_by_position.get(next_position) {
                    if *step < current_step_count {
                        drop(lowest_step_count_by_position);
                        break;
                    }
                }
                lowest_step_count_by_position.insert(position, current_step_count);
                drop(lowest_step_count_by_position);

                let mut semi_circle = arc_semi_circle.lock().unwrap();
                let pipe = Pipe::new(*map.get(position.0 as usize).unwrap().get(position.1 as usize).unwrap(), direction.get_origin_direction(), next_direction.clone());
                semi_circle.insert(position, pipe);
                drop(semi_circle);
            }
        }
        position_and_direction = next_step.unwrap();
    }    
}

fn get_possible_next_step(map: &Vec<Vec<char>>, position: (i32, i32), direction: &Direction) -> Option<((i32, i32), Direction)> {
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

fn reduces_duplicates(map_1: &mut HashMap<(i32, i32), Pipe>, map_2: &mut HashMap<(i32, i32), Pipe>) {
    if map_1.len() > map_2.len() {
        let duplicate_points = map_1
            .iter()
            .filter(|(&k, _)| map_2.contains_key(&k))
            .map(|(k, _)| k)
            .collect::<Vec<_>>();

        map_2.retain(|&k, _| !duplicate_points.contains(&&k));
    } else {
        let duplicate_points = map_2
            .iter()
            .filter(|(&k, _)| map_1.contains_key(&k))
            .map(|(k, _)| k)
            .collect::<Vec<_>>();
        
        map_1.retain(|&k, _| !duplicate_points.contains(&&k));
    };
}

fn get_rotation_sense_by_semi_circle(pipes_semi_circle_1: Vec<&Pipe>, pipes_semi_circle_2: Vec<&Pipe>) -> (RotationSense, RotationSense) {
    let separate_pipes_by_rotation_sense = | pipes: Vec<&Pipe> | -> (Vec<Pipe>, Vec<Pipe>) {
        pipes
            .into_iter()
            .map(|pipe| pipe.clone())
            .filter(|pipe| KindPipe::LINE != pipe.kind)
            .partition(|pipe| matches!(pipe.kind, KindPipe::BEND(RotationSense::CLOCKWISE)))
    };
    let (clockwise_semi_circle_1, counter_clockwise_semi_circle_1): (Vec<_>, Vec<_>) = separate_pipes_by_rotation_sense(pipes_semi_circle_1);
    let (clockwise_semi_circle_2, counter_clockwise_semi_circle_2): (Vec<_>, Vec<_>) = separate_pipes_by_rotation_sense(pipes_semi_circle_2);

    if clockwise_semi_circle_1.len() + counter_clockwise_semi_circle_2.len() > counter_clockwise_semi_circle_1.len() + clockwise_semi_circle_2.len() {
        (RotationSense::CLOCKWISE, RotationSense::COUNTERCLOCKWISE)
    } else {
        (RotationSense::COUNTERCLOCKWISE, RotationSense::CLOCKWISE)
    }
}

fn define_inner_side(
    pipes_semi_circle_1: Vec<&mut Pipe>,
    rotation_sense_semi_circle_1: RotationSense,
    pipes_semi_circle_2: Vec<&mut Pipe>,
    rotation_sense_semi_circle_2: RotationSense
) {
    let define_inner_side = | pipes: Vec<&mut Pipe>, rotation_sense: &RotationSense | {
        pipes
        .into_iter()
        .for_each(|pipe| pipe.set_inside_sides(rotation_sense));
    };

    define_inner_side(pipes_semi_circle_1, &rotation_sense_semi_circle_1);
    define_inner_side(pipes_semi_circle_2, &rotation_sense_semi_circle_2);
}

fn get_count_enclosed_tiles(map: Vec<Vec<char>>, circle: HashMap<(i32, i32), Pipe>) -> u32 {
    let mut count_enclosed_tiles = 0;
    let (mut external_tracking_map, mut internal_tracking_map) = ((TrackingMap::ROWS(HashMap::new()), TrackingMap::COLUMNS(HashMap::new())), (TrackingMap::ROWS(HashMap::new()), TrackingMap::COLUMNS(HashMap::new())));
    
    for (y, line) in map.iter().enumerate() {
        for (x, _tiles) in line.iter().enumerate() {
            match circle.get(&(y as i32, x as i32)) {
                Some(_) => continue,
                None => {
                    let is_in_internal_row_serie = is_in_a_series((y as u32, x as u32), &internal_tracking_map.0);
                    let is_in_internal_column_serie = is_in_a_series((x as u32, y as u32), &internal_tracking_map.1);
                    let mut add = false;

                    if is_in_internal_row_serie && is_in_internal_column_serie {
                        add = true;
                    } else if is_in_internal_row_serie || is_in_internal_column_serie {
                        add = true;

                        if is_in_internal_row_serie {
                            add_to_internal_column_series(&circle, &mut internal_tracking_map.1, (y, x));
                        } else {
                            add_to_internal_row_series(&circle, &mut internal_tracking_map.0, (y, x));
                        }
                    } else {
                        let is_in_external_row_serie = is_in_a_series((y as u32, x as u32), &external_tracking_map.0);
                        let is_in_external_column_serie = is_in_a_series((x as u32, y as u32), &external_tracking_map.1);

                        if !is_in_external_row_serie && !is_in_external_column_serie {
                            if add_to_internal_row_series(&circle, &mut internal_tracking_map.0, (y, x)) && add_to_internal_column_series(&circle, &mut internal_tracking_map.1, (y, x)) {
                                add = true;
                            } else {
                                add_to_external_row_series(&circle, &mut external_tracking_map.0, (y, x), line.len());
                                add_to_external_column_series(&circle, &mut external_tracking_map.1, (y, x), map.len());
                            }
                        } else if !is_in_external_row_serie || !is_in_external_column_serie {
                            if is_in_external_row_serie {
                                add_to_external_column_series(&circle, &mut external_tracking_map.1, (y, x), map.len());
                            } else {
                                add_to_external_row_series(&circle, &mut external_tracking_map.0, (y, x), line.len());
                            }
                        }
                    }

                    if add {
                        count_enclosed_tiles += 1;
                    }
                }
            }
        }

        delete_past_rows(&mut external_tracking_map.0, y as u32);
        delete_past_rows(&mut internal_tracking_map.0, y as u32);

    }

    
    count_enclosed_tiles
}

fn is_in_a_series((base, position): (u32, u32), tracking_map: &TrackingMap) -> bool {
    match tracking_map {
        TrackingMap::ROWS(series) | TrackingMap::COLUMNS(series) => {
            match series.get(&base) {
                None => false,
                Some(range) => range.contains(&position)
            }
        }
    }
}

fn add_to_internal_row_series(circle: &HashMap<(i32, i32), Pipe>, rows_tracking_map: &mut TrackingMap, position: (usize, usize)) -> bool {
    let y = position.0;
    let mut x = position.1;

    loop {
        match x.checked_sub(1) {
            None => {
                return false;
            },
            Some(v) => {
                x = v;

                match circle.get(&(y as i32, x as i32)) {
                    None => (),
                    Some(pipe) => {
                        if pipe.inside_sides.contains(&Direction::RIGHT) {
                            x = position.1;
                            
                            loop {
                                x += 1;
        
                                match circle.get(&(y as i32, x as i32)) {
                                    None => (),
                                    Some(_) => {
                                        match rows_tracking_map {
                                            TrackingMap::ROWS(series) => {
                                                series.insert(y as u32, Range { start: position.1 as u32, end: x as u32 });
                                                return true;
                                            },
                                            _ => unreachable!()
                                        }
                                    }
                                }
        
                            }
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
    }
}

fn add_to_internal_column_series(circle: &HashMap<(i32, i32), Pipe>, columns_tracking_map: &mut TrackingMap, position: (usize, usize)) -> bool {
    let mut y = position.0;
    let x = position.1;

    loop {
        match y.checked_sub(1) {
            None => {
                return false;
            },
            Some(v) => {
                y = v;

                match circle.get(&(y as i32, x as i32)) {
                    None => (),
                    Some(pipe) => {
                        if pipe.inside_sides.contains(&Direction::DOWN) {
                            y = position.0;
                            
                            loop {
                                y += 1;
        
                                match circle.get(&(y as i32, x as i32)) {
                                    None => (),
                                    Some(_) => {
                                        match columns_tracking_map {
                                            TrackingMap::COLUMNS(series) => {
                                                series.insert(x as u32, Range { start: position.0 as u32, end: y as u32 });
                                                return true;
                                            },
                                            _ => unreachable!()
                                        }
                                    }
                                }
        
                            }
                        } else {
                            return false;
                        }
                    }
                }
            }
        }        
    }
}

fn add_to_external_row_series(circle: &HashMap<(i32, i32), Pipe>, rows_tracking_map: &mut TrackingMap, position: (usize, usize), limit: usize) -> bool {
    let y = position.0;
    let mut x = position.1;

    loop {
        x += 1;

        if x > limit || circle.get(&(y as i32, x as i32)).is_some() {
            match rows_tracking_map {
                TrackingMap::ROWS(series) => {
                    series.insert(y as u32, Range { start: position.1 as u32, end: x as u32 });
                    return true;
                },
                _ => unreachable!()
            }
        }
    }
}

fn add_to_external_column_series(circle: &HashMap<(i32, i32), Pipe>, columns_tracking_map: &mut TrackingMap, position: (usize, usize), limit: usize) -> bool {
    let mut y = position.0;
    let x = position.1;

    loop {
        y += 1;

        if y > limit || circle.get(&(y as i32, x as i32)).is_some() {
            match columns_tracking_map {
                TrackingMap::COLUMNS(series) => {
                    series.insert(x as u32, Range { start: position.0 as u32, end: y as u32 });
                    return true;
                },
                _ => unreachable!()
            }
        }
    }
}

fn delete_past_rows(tracking_map: &mut TrackingMap, row: u32) {

    match tracking_map {
        TrackingMap::ROWS(series) => {
            series.remove(&row);
        },
        _ => unreachable!()
    };
}