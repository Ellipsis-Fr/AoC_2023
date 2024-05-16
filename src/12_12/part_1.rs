use std::{cell::RefCell, collections::{vec_deque, VecDeque}, rc::Rc};

use itertools::Itertools;
use regex::Regex;
use AoC_2023::text_file_reader::TextFileReader;

const OPERATIONAL_SPRING_SEARCH_REGEX: &str = r"(\s|\?)+";
const DAMAGED_SPRING_SEARCH_REGEX: &str = r"(#|\?){n}";

const OPERATIONAL_SPRING: &str = ".";
const DAMAGED_SPRING: &str = "#";

#[derive(Debug, Clone)]
enum SpringsState {
    OPERATIONAL(String),
    DAMAGED(String)
}

impl SpringsState {
    fn new(sign: String) -> Self {
        match sign.as_str() {
            "." => Self::OPERATIONAL(sign),
            "#" => Self::DAMAGED(sign),
            _ => panic!("no other signs allowed")            
        }
    }
}

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
            self.number_of_arrangements_from_here += self.children.iter().map(|v| v.borrow().number_of_arrangements_from_here).sum::<i32>();
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
    
    let damaged_records = get_puzzle();
    // let total_sum_of_possible_spring_arrangements = count_possible_spring_arrangements(damaged_records);
    // println!("Total sum of possible spring arrangements : {total_sum_of_possible_spring_arrangements}");

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


    // let a = find_possible_arrangements("??", 1);
    // let a = find_possible_arrangements("???????", 2);
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
        
        let mut arrangement_springs = ArrangementSprings::new(format!("{}+{}", springs_line, sizes_of_group_of_damaged_springs));
        search_arrangements(
            springs_in_unknown_state,
            list_of_sizes_of_group_of_damaged_springs.clone(),
            &arrangement_springs,
            Rc::clone(&arrangement_springs.root),
            0
        );
        // let a = arrangment_springs.root;
        // println!("{:?}", a);


        sum += {
            let mut root_borrowed_mut = arrangement_springs.root.borrow_mut();
            root_borrowed_mut.set_number_of_arrangements();
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
) -> bool {
    let mut arrangement_found = false;

    match arrangement_springs.search_for_existing_node(Some(&springs_in_unknown_state.join(",")), None, level) {
        Some(existing_nodes) => {
            arrangement_found = true;
            let mut actuel_node_borrowed_mut = rc_actual_node.borrow_mut();
            actuel_node_borrowed_mut.add_children_nodes(existing_nodes);
            actuel_node_borrowed_mut.set_number_of_arrangements();
        },
        None => {
            if let Some(size) = list_of_sizes_of_group_of_damaged_springs.pop_front() {
                for (index, springs_group_in_unknown_state) in springs_in_unknown_state.iter().enumerate() {
                    if springs_group_in_unknown_state.len() < size {
                        continue;
                    }

                    
                    let possible_arrangements = find_possible_arrangements(springs_group_in_unknown_state, size);
                    
                    if possible_arrangements.is_empty() {
                        continue;
                    }




                    // TODO : A partir de la précédente liste nous créons x noeud et y ajoutons à chaque fois le reste à la liste 'springs_in_unknown_state'
                    // TODO : après y avoir substiuer le ou les 'springs_group_in_unknown_state' passé (cf. index)
                    // TODO : Mais avant toute nouvelle relance on contrôle la cohérence du nouveau contenu de 'springs_in_unknown_state' avec la liste à jour de 'list_of_sizes_of_group_of_damaged_springs'




    
                    //// Donc finalement je gère la possible incohérence, via le retour bool de cette méthode appelée récursivement,
                    //// quand 'false' me sera retourné je supprimerai ce noeud de la liste des enfants du noeud actuel

                    // * Mais peut être qu'en utilisant une bonne regex je pourrais obtenir une solution me permettant de gérer les incohérences... car j'ai peur que ne pas les gérer cause des soucs de perte de temps...
    
                }
            }
        }
    }

    arrangement_found

    //
    // Nouveau fonctionnement :
    // -dès que je crée un nouveau qui n'existe pas déjà je l'ajoute au noeud actuel
    //- puis je lane à nouveau search arrangement
    //- dès que je reviens au nouveau du noeud qui venait d'être créé et ajouté, je compte les arrangements du nouveau noeud
    //
    // ! Attention ce nouveau fonctionnement n'est possible que sous réserve que l'algo d'évaluation de cohérence soit infallible,
    // ! sinon il faudra revoir ce fonctionnement et notamment y ajouter la possibilité qu'aucune correspondance ne soit trouvée
    //
}

/// Méthode contrôlant ce qu'il est possible de faire avec les entrées suivantes : 'springs_group_in_unknown_state' et 'size' 
/// et retournant un vec de tuple : [(arrangement réalisé pour ce sous groupe, reste de ce sous-groupe), ...], exemple : 
/// entrées : 'springs_group_in_unknown_state' = '???????' et 'size' = 2
/// sortie : [ ('##.', '????'), ('##.', '???'), ('##.', '??'), ('##.', '?'), ('##.', ''), ('##', '')] 
/// dans cet exemple les 2 dernières valeurs ne seront pas utilisées dû au contrôle de cohérence
/// Cette méthode procédera ainsi :
/// - contrôlera la cohérence de la demande
/// - parcourera la chaine 'springs_group_in_unknown_state' à partir de l'index 0 sur une suite de x valant 'size' et y associera le reste
fn find_possible_arrangements(springs_group_in_unknown_state: &str, size: usize) -> Vec<(String,String)> {
    let mut possible_arrangements = Vec::new();

    let springs_in_unknown_state = springs_group_in_unknown_state.chars().collect::<Vec<_>>();
    let damaged_indexes = springs_in_unknown_state.iter().positions(|s| *s == DAMAGED_SPRING.chars().next().unwrap()).collect::<Vec<_>>();
    
    let mut index = 0;
    let springs_group_len = springs_group_in_unknown_state.len();

    while index + size <= springs_group_len  {
        if damaged_indexes.iter().filter(|i| **i < index || **i > index + size).collect::<Vec<_>>().is_empty() {
            let mut possible_springs_damaged = DAMAGED_SPRING.to_string().repeat(size);
            
            if index + size < springs_group_len {
                possible_springs_damaged.push_str(OPERATIONAL_SPRING);
            }

            let remainder_springs_in_unknown_state = if index + size + 1 < springs_group_len {
                let a = &springs_in_unknown_state[(index + size + 1)..].iter().map(|c| c.to_string()).collect::<Vec<_>>();
                a.join("")
            } else {
                String::new()
            };

            possible_arrangements.push((possible_springs_damaged, remainder_springs_in_unknown_state));            
        }

        index += 1;
    }



    possible_arrangements
}

fn can_match(springs_in_unknown_state: &str, list_of_sizes_of_group_of_damaged_springs: &VecDeque<usize>) -> bool {
    if list_of_sizes_of_group_of_damaged_springs.is_empty() {
        false
    } else {
        let mut combined_regex = String::new();
    
        for size in list_of_sizes_of_group_of_damaged_springs {
            let damaged_spring_search_regex = DAMAGED_SPRING_SEARCH_REGEX.replace("n", &(size.to_string()));

            if combined_regex.is_empty() {
                combined_regex = format!("{}", damaged_spring_search_regex);
            } else {
                combined_regex.push_str(&format!("{}{}", OPERATIONAL_SPRING_SEARCH_REGEX, damaged_spring_search_regex));
            }
        }

        let combined_regex = Regex::new(&combined_regex).unwrap();
        combined_regex.is_match(springs_in_unknown_state)
    }
}