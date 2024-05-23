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
    // parents: Vec<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    damaged_record_in_study: String, // nc
    hypothesis : String, // np
    number_of_arrangements_from_here: i32
}

impl Node {
    pub fn new(damaged_record_in_study: String, hypothesis: String) -> Self {
        Node { /*parents: Vec::new(),*/ children: Vec::new(), damaged_record_in_study, hypothesis, number_of_arrangements_from_here: 0 }
    }

    pub fn set_number_of_arrangements(&mut self) {
        if self.children.is_empty() {
            self.number_of_arrangements_from_here = 1;
        } else {
            self.number_of_arrangements_from_here = self.children.iter().map(|v| v.borrow().number_of_arrangements_from_here).sum::<i32>();
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
    println!("Puzzle du 12/12 Partie 1");
    let now = Instant::now();
    
    let damaged_records = get_puzzle();
    let total_sum_of_possible_spring_arrangements = count_possible_spring_arrangements(damaged_records);
    println!("Total sum of possible spring arrangements : {total_sum_of_possible_spring_arrangements}");
    println!("took: {:?}", now.elapsed());

    // valeurs déjà testées : 5308, 7782, 7767, 7772, 7770, 7709, 7693, 7655

    // let mut a = "??? ###";
    // let mut b = VecDeque::from([1,1,3]);
    // assert!(can_match(a, &b));

    // a = " ?? ?? ?## ";
    // b = VecDeque::from([1,1,3]);
    // assert!(can_match(a, &b));

    // a = "?#?#?#?#?#?#?#?";
    // b = VecDeque::from([1,3,1,6]);
    // assert!(can_match(a, &b));

    // a = "???? # #";
    // b = VecDeque::from([4,1,1]);
    // assert!(can_match(a, &b));

    // a = "???? ###### #####";
    // b = VecDeque::from([1,6,5]);
    // assert!(can_match(a, &b));

    // a = "?###????????";
    // b = VecDeque::from([3,2,1]);
    // assert!(can_match(a, &b));

    // let a = "# ??";
    // let b = VecDeque::from([2]);
    // assert!(can_match(a, &b));

    // let a = vec!["?#", "??#??"];
    // let b = VecDeque::from([2]);
    // assert!(!can_match(&a, &b));

    // let a = vec!["?#", "#???#"];
    // let b = VecDeque::from([2,4]);
    // assert!(!can_match(&a, &b));


    // let a = find_possible_arrangements("??", 1);
    // let a = find_possible_arrangements("???????", 2);
    // let a = find_possible_arrangements("?##", 3);
    // let a = find_possible_arrangements("?###????????", 3);
    // dbg!(a);
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("12_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn count_possible_spring_arrangements(damaged_records: Vec<String>) -> i32 {
    let mut sum = 0;
    
    for damaged_record in damaged_records {
        let mut iter_damaged_record = damaged_record.split_whitespace();
        let (springs_line, sizes_of_group_of_damaged_springs) = (iter_damaged_record.next().unwrap().to_owned(), iter_damaged_record.next().unwrap().to_owned());
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
        // let a = arrangement_springs.root;
        // println!("{:?}", arrangement_springs.root);


        sum += {
            let mut root_borrowed_mut = arrangement_springs.root.borrow_mut();
            // println!("{:?}", root_borrowed_mut);
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

    {
        let actuel_node_borrowed_mut = rc_actual_node.borrow_mut();
        println!("{:?}   ------  {:?}", &actuel_node_borrowed_mut.hypothesis, list_of_sizes_of_group_of_damaged_springs);
    }

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
                        // println!("là");
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

    println!();
    println!("==============================================================");
    println!("==============================================================");
    println!("==============================================================");
    println!();
    println!();

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
    // println!("{:?}", springs_in_unknown_state);
    // println!("{:?}", damaged_indexes);
    
    let mut index = 0;
    let springs_group_len = springs_group_in_unknown_state.len();

    while index + size <= springs_group_len  {
        // println!("{}", index + size);
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
                // let next_size = next_size.unwrap();
                // let count_unknown_springs = remainder_springs_in_unknown_state.chars().filter(|c| *c == UNKNOWN_SPRINGS.chars().next().unwrap()).count();
                // let count_damaged_springs = remainder_springs_in_unknown_state.chars().filter(|c| *c == DAMAGED_SPRINGS.chars().next().unwrap()).count();
                if remainder_springs_in_unknown_state.len() < *next_size.unwrap() {
                    index += 1;
                    continue;
                }
            }

            // println!("{} {}", possible_springs_damaged, remainder_springs_in_unknown_state);
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
        let group_of_springs_damaged = list_of_springs_in_unknown_state.iter().filter(|springs| springs.contains(DAMAGED_SPRINGS)).collect::<Vec<_>>();
        let count_group_of_springs_damaged = group_of_springs_damaged.len();
        if count_group_of_springs_damaged > list_of_sizes_of_group_of_damaged_springs.len() {
            false
        } else if list_of_sizes_of_group_of_damaged_springs.is_empty() {
            true
        } else {
            // println!("{}", springs_in_unknown_state);

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

            // println!("{:?}", combined_regex);
    
            let combined_regex = Regex::new(&combined_regex).unwrap();
            let mut is_match = combined_regex.is_match(&springs_in_unknown_state);

            if count_group_of_springs_damaged == list_of_sizes_of_group_of_damaged_springs.len() {
                let merged_vec = group_of_springs_damaged.iter().zip(list_of_sizes_of_group_of_damaged_springs.iter()).collect::<Vec<_>>();
                // println!("{:?}", list_of_sizes_of_group_of_damaged_springs);
                // println!("{:?}", merged_vec);
                for (springs_damaged, size) in merged_vec {
                    if springs_damaged.trim_matches('?').len() > *size {
                        is_match = false;
                        break;
                    }
                }
            } 
            // else {
            //     let index_list_of_sizes = 0;

            //     for springs_damaged in group_of_springs_damaged {
            //         if springs_damaged.trim_matches('?').len() > *size {
            //             is_match = false;
            //             break;
            //         }
            //     }
            // }

            is_match

        }
    }
}