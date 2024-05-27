use std::{cell::RefCell, collections::VecDeque, rc::Rc, time::Instant};

use itertools::Itertools;
use regex::Regex;
use AoC_2023::text_file_reader::TextFileReader;

const OPERATIONAL_SPRING_SEARCH_REGEX: &str = r"(\s|\?)+";
const DAMAGED_SPRING_SEARCH_REGEX: &str = r"(#|\?){n}";

const OPERATIONAL_SPRINGS: &str = ".";
const DAMAGED_SPRINGS: &str = "#";
const UNKNOWN_SPRINGS: &str = "?";

#[derive(Debug)]
struct ArrangementSprings {
    root: Rc<RefCell<Node>>
}

impl ArrangementSprings {
    fn new(damaged_record_in_study: String) -> Self {
        ArrangementSprings { root: Rc::new(RefCell::new(Node::new(damaged_record_in_study, String::from("Racine")))) }
    }

    fn search_for_existing_node(&self, by_damaged_record_in_study: Option<&str>, by_hypothesis: Option<&str>, level: u32) -> Option<Vec<Rc<RefCell<Node>>>> {
        self.root.borrow().search_for_existing_node(by_damaged_record_in_study, by_hypothesis, level)
    }


}

#[derive(Debug)]
struct Node {
    children: Vec<Rc<RefCell<Node>>>,
    damaged_record_in_study: String, // nc
    hypothesis : String, // np
    number_of_arrangements_from_here: i128
}

impl Node {
    pub fn new(damaged_record_in_study: String, hypothesis: String) -> Self {
        Node { children: Vec::new(), damaged_record_in_study, hypothesis, number_of_arrangements_from_here: 0 }
    }

    pub fn set_number_of_arrangements(&mut self) {
        println!("{}", self.hypothesis);

        if self.children.is_empty() {
            let mut hypothesis = self.hypothesis.split(",").collect::<VecDeque<_>>();
            println!("{:?}", hypothesis);
            hypothesis.pop_front();
            if !hypothesis.is_empty() && hypothesis.contains(&"#") {
                println!("là");
                self.number_of_arrangements_from_here = 0;
            } else {
                self.number_of_arrangements_from_here = 1;
            }
        } else {
            self.number_of_arrangements_from_here = self.children.iter().map(|v| v.borrow().number_of_arrangements_from_here).sum::<i128>();
        }
    }

    pub fn add_children_nodes(&mut self, new_children: Vec<Rc<RefCell<Node>>>) {
        self.children.extend(new_children);
    }

    pub fn search_for_existing_node(&self, by_damaged_record_in_study: Option<&str>, by_hypothesis: Option<&str>, level: u32) -> Option<Vec<Rc<RefCell<Node>>>> {
        let mut existing_nodes = Vec::new();

        if level == 0 {
            existing_nodes = self.children
                .iter()
                .filter(|child| {
                    if by_damaged_record_in_study.is_some() {
                        child.borrow().damaged_record_in_study == by_damaged_record_in_study.unwrap()
                    } else {
                        child.borrow().hypothesis == by_hypothesis.unwrap()
                    }
                })
                .map(|child| Rc::clone(&child))
                .collect::<Vec<_>>();
        } else {
            for child in &self.children {
                match child.borrow().search_for_existing_node(by_damaged_record_in_study, by_hypothesis, level - 1) {
                    Some(nodes) => {
                        existing_nodes = nodes;
                        break;
                    },
                    None => ()
                }
            }
        }

        if existing_nodes.is_empty() {
            None
        } else {
            Some(existing_nodes)
        }
    }
}


fn main() {
    println!("Puzzle du 12/12 Partie 2");
    let now = Instant::now();
    
    let damaged_records = get_puzzle();
    let total_sum_of_possible_spring_arrangements = count_possible_spring_arrangements(damaged_records);
    println!("Total sum of possible spring arrangements : {total_sum_of_possible_spring_arrangements}");
    println!("took: {:?}", now.elapsed());
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("12_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn count_possible_spring_arrangements(damaged_records: Vec<String>) -> i128 {
    let mut sum = 0;
    const UNFOLD_RECORD: usize = 5;
    
    for damaged_record in damaged_records {
        let mut iter_damaged_record = damaged_record.split_whitespace();
        let (folded_springs_line, folded_sizes_of_group_of_damaged_springs) = (iter_damaged_record.next().unwrap().to_owned(), iter_damaged_record.next().unwrap().to_owned());
        
        let mut springs_line = String::new();
        let mut sizes_of_group_of_damaged_springs = String::new();

        let mut unfolding = 0;
        while unfolding < UNFOLD_RECORD {
            unfolding += 1;

            springs_line.push_str(&folded_springs_line);
            sizes_of_group_of_damaged_springs.push_str(&folded_sizes_of_group_of_damaged_springs);

            if unfolding < UNFOLD_RECORD {
                springs_line.push_str(&UNKNOWN_SPRINGS);
                sizes_of_group_of_damaged_springs.push_str(",");
            }
        }        
        
        let springs_in_unknown_state  = springs_line.split(".").filter(|v| !v.is_empty()).collect::<Vec<_>>();
        let list_of_sizes_of_group_of_damaged_springs = sizes_of_group_of_damaged_springs.split(",").map(|v| v.parse::<usize>().unwrap()).collect::<VecDeque<_>>();
        
        let arrangement_springs = ArrangementSprings::new(format!("{}+{}", springs_line, sizes_of_group_of_damaged_springs));
        search_arrangements(
            springs_in_unknown_state,
            list_of_sizes_of_group_of_damaged_springs.clone(),
            &arrangement_springs,
            Rc::clone(&arrangement_springs.root),
            0
        );


        sum += {
            let mut root_borrowed_mut = arrangement_springs.root.borrow_mut();
            root_borrowed_mut.set_number_of_arrangements();
            println!("{} - {}", springs_line, root_borrowed_mut.number_of_arrangements_from_here);
            root_borrowed_mut.number_of_arrangements_from_here
        };
    }
    
    sum
}

fn search_arrangements(
    springs_in_unknown_state: Vec<&str>,
    mut list_of_sizes_of_group_of_damaged_springs: VecDeque<usize>,
    arrangement_springs: &ArrangementSprings,
    rc_actual_node: Rc<RefCell<Node>>,
    level: u32
) {
    assert!(!springs_in_unknown_state.is_empty());
    assert!(!list_of_sizes_of_group_of_damaged_springs.is_empty());

    let damaged_record_in_study = &springs_in_unknown_state.join(",");

    match arrangement_springs.search_for_existing_node(Some(damaged_record_in_study), None, level) {
        Some(existing_nodes) => {
            let mut actuel_node_borrowed_mut = rc_actual_node.borrow_mut();
            actuel_node_borrowed_mut.add_children_nodes(existing_nodes);
            actuel_node_borrowed_mut.set_number_of_arrangements();
        },
        None => {
            if let Some(size) = list_of_sizes_of_group_of_damaged_springs.pop_front() {
                let mut inflexible = false;
                for (index, springs_group_in_unknown_state) in springs_in_unknown_state.iter().enumerate() {
                    if springs_group_in_unknown_state.len() < size {
                        continue;
                    }
                    
                    if springs_group_in_unknown_state.contains(DAMAGED_SPRINGS) {
                        inflexible = true;
                    } else if springs_in_unknown_state[(index + 1)..].iter().filter(|s| s.contains(DAMAGED_SPRINGS)).map(|s| s.chars().filter(|c| *c == DAMAGED_SPRINGS.chars().next().unwrap()).count()).sum::<usize>() > list_of_sizes_of_group_of_damaged_springs.iter().sum() {
                        continue;
                    }

                    let possible_arrangements = find_possible_arrangements(springs_group_in_unknown_state, size, list_of_sizes_of_group_of_damaged_springs.get(0));
                    if !possible_arrangements.is_empty() {
                        possible_arrangements.into_iter().for_each(|(possible_springs_damaged, remainder_springs_in_unknown_state)| {
                            let new_list_of_springs_in_unknown_state = if remainder_springs_in_unknown_state.is_empty() {
                                springs_in_unknown_state[(index + 1)..].to_vec()
                            } else {
                                let mut vec = vec![remainder_springs_in_unknown_state.as_str()];
                                vec.extend_from_slice(&springs_in_unknown_state[(index + 1)..]);
                                vec
                            };

                            if can_match(&new_list_of_springs_in_unknown_state, &list_of_sizes_of_group_of_damaged_springs) {
                                let mut hypothesis = possible_springs_damaged;
                                if !new_list_of_springs_in_unknown_state.is_empty() {
                                    hypothesis += ",";
                                    hypothesis += &new_list_of_springs_in_unknown_state.join(",");
                                }

                                match arrangement_springs.search_for_existing_node(None, Some(hypothesis.as_str()), level) {
                                    Some(existing_nodes) => {
                                        let mut actuel_node_borrowed_mut = rc_actual_node.borrow_mut();
                                        actuel_node_borrowed_mut.add_children_nodes(existing_nodes);
                                    },
                                    None => {
                                        let new_arrangement_node = Rc::new(RefCell::new(Node::new(damaged_record_in_study.clone(), hypothesis)));
                                        {
                                            let mut actuel_node_borrowed_mut = rc_actual_node.borrow_mut();
                                            actuel_node_borrowed_mut.add_children_nodes(vec![Rc::clone(&new_arrangement_node)]);
                                        }

                                        if list_of_sizes_of_group_of_damaged_springs.is_empty() {
                                            assert!(!new_list_of_springs_in_unknown_state.contains(&DAMAGED_SPRINGS));
                                            let mut new_arrangement_node_borrowed_mut = new_arrangement_node.borrow_mut();
                                            new_arrangement_node_borrowed_mut.set_number_of_arrangements();
                                        } else {
                                            search_arrangements(
                                                new_list_of_springs_in_unknown_state,
                                                list_of_sizes_of_group_of_damaged_springs.clone(),
                                                &arrangement_springs,
                                                Rc::clone(&new_arrangement_node),
                                                level + 1
                                            );
                                        }
                                    }
                                }
                            }
                        });

                    }
                    if inflexible {
                        break;
                    }
                }

                let mut actuel_node_borrowed_mut = rc_actual_node.borrow_mut();
                actuel_node_borrowed_mut.set_number_of_arrangements();
            }
        }
    }
}

/// Méthode contrôlant ce qu'il est possible de faire avec les entrées suivantes : 'springs_group_in_unknown_state' et 'size' 
/// et retournant un vec de tuple : [(arrangement réalisé pour ce sous groupe, reste de ce sous-groupe), ...], exemple : 
/// entrées : 'springs_group_in_unknown_state' = '???????' et 'size' = 2
/// sortie : [ ('##.', '????'), ('##.', '???'), ('##.', '??'), ('##.', '?'), ('##.', ''), ('##', '')] 
/// dans cet exemple les 2 dernières valeurs ne seront pas utilisées dû au contrôle de cohérence
/// Cette méthode procédera ainsi :
/// - contrôlera la cohérence de la demande
/// - parcourera la chaine 'springs_group_in_unknown_state' à partir de l'index 0 sur une suite de x valant 'size' et y associera le reste
fn find_possible_arrangements(springs_group_in_unknown_state: &str, size: usize, next_size: Option<&usize>) -> Vec<(String,String)> {
    let mut possible_arrangements = Vec::new();

    let springs_in_unknown_state = springs_group_in_unknown_state.chars().collect::<Vec<_>>();
    let damaged_indexes = springs_in_unknown_state.iter().positions(|s| *s == DAMAGED_SPRINGS.chars().next().unwrap()).collect::<Vec<_>>();
    
    let mut index = 0;
    let springs_group_len = springs_group_in_unknown_state.len();

    while index + size <= springs_group_len  {
        if damaged_indexes.iter().filter(|i| **i < index || **i == index + size).collect::<Vec<_>>().is_empty() {
            let mut possible_springs_damaged = DAMAGED_SPRINGS.to_string().repeat(size);
            
            if index + size < springs_group_len {
                possible_springs_damaged.push_str(OPERATIONAL_SPRINGS);
            }

            let remainder_springs_in_unknown_state = if index + size + 1 < springs_group_len {
                let a = &springs_in_unknown_state[(index + size + 1)..].iter().map(|c| c.to_string()).collect::<Vec<_>>();
                a.join("")
            } else {
                String::new()
            };

            if remainder_springs_in_unknown_state.contains(DAMAGED_SPRINGS) {
                if next_size.is_none() {
                    index += 1;
                    continue;
                }

                if remainder_springs_in_unknown_state.len() < *next_size.unwrap() {
                    index += 1;
                    continue;
                }
            }

            possible_arrangements.push((possible_springs_damaged, remainder_springs_in_unknown_state));            
        }

        index += 1;
    }



    possible_arrangements
}

fn can_match(list_of_springs_in_unknown_state: &Vec<&str>, list_of_sizes_of_group_of_damaged_springs: &VecDeque<usize>) -> bool {
    if list_of_springs_in_unknown_state.is_empty() {
        if list_of_sizes_of_group_of_damaged_springs.is_empty() {
            true
        } else {
            false
        }
    } else {
        let springs_in_unknown_state = list_of_springs_in_unknown_state.join(" ");

        let mut combined_regex = String::new();
    
        for size in list_of_sizes_of_group_of_damaged_springs {
            let damaged_spring_search_regex = DAMAGED_SPRING_SEARCH_REGEX.replace("n", &(size.to_string()));

            if combined_regex.is_empty() {
                combined_regex = format!("{}", damaged_spring_search_regex);
            } else {
                combined_regex.push_str(&format!("{}{}", OPERATIONAL_SPRING_SEARCH_REGEX, damaged_spring_search_regex));
            }
        }
        
        combined_regex.push_str("[^#]*$");


        let combined_regex = Regex::new(&combined_regex).unwrap();
        combined_regex.is_match(&springs_in_unknown_state) && !combined_regex.replace_all(&springs_in_unknown_state, ".").contains("#")
    }
}