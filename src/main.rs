mod sort;
use sort::{insertion, selection};

use clap::{Parser};

#[derive(Debug, Parser)]
// #[clap(author, version, about)]
pub struct CliArgs {
    /// Name of algorithm
    name: String,
}

fn main() {
    let args = CliArgs::parse();
    let mut input = vec![4,6,1,7,3,4,2];

    match args.name.as_str() {
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