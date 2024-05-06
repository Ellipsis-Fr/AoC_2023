use std::{cell::RefCell, rc::{Rc, Weak}};

use AoC_2023::text_file_reader::TextFileReader;

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

    fn search_for_existing_node(&self, by_damaged_record_in_study: Option<&str>, by_hypothesis: Option<&str>, level: u32) -> Vec<Rc<RefCell<Node>>> {
        todo!()
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

    pub fn add_children_nodes(&mut self, new_children: Vec<Rc<RefCell<Node>>>) {
        self.number_of_arrangements_from_here += new_children.len() as i32;
        self.children.extend(new_children);
    }
}


fn main() {
    println!("Puzzle du 12/12 Partie 1");
    
    let damaged_records = get_puzzle();
    let total_sum_of_possible_spring_arrangements = count_possible_spring_arrangements(damaged_records);
    println!("Total sum of possible spring arrangements : {total_sum_of_possible_spring_arrangements}");
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
        let list_of_sizes_of_group_of_damaged_springs = sizes_of_group_of_damaged_springs.split(",").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();
        
        let mut arrangement_springs = ArrangementSprings::new(format!("{}+{}", springs_line, sizes_of_group_of_damaged_springs));
        let all_children = search_arrangements(springs_in_unknown_state, list_of_sizes_of_group_of_damaged_springs, &arrangement_springs, Rc::clone(&arrangement_springs.root));


        arrangement_springs.root.borrow_mut().add_children_nodes(all_children);
        // let a = arrangment_springs.root;
        // println!("{:?}", a);


        sum += arrangement_springs.root.borrow().number_of_arrangements_from_here;
    }
    
    sum
}

fn search_arrangements(
    springs_in_unknown_state: Vec<&str>,
    list_of_sizes_of_group_of_damaged_springs: Vec<i32>,
    arrangement_springs: &ArrangementSprings,
    actual_node: Rc<RefCell<Node>>
) -> Vec<Rc<RefCell<Node>>> {


    todo!()
}