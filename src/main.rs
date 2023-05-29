mod sort;

use sort::{insertion, selection};

use std::env;



fn main() {
    let script = &*env::args().nth(1).expect("went wrong");
    let mut input: Vec<u64> = vec![3,7,8,1,6,6];

    match script {
        "insertion_sort" => {
            insertion(&mut input);
        },
        "selection_sort" => {
            selection(&mut input)
        }
        _ => {
            println!("This is not a valid implemented algorithm")
        }
    }

}