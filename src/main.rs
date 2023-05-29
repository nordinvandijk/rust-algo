mod sort;

use sort::insertion;

fn main() {
    let mut input: Vec<u64> = vec![3,7,8,1,6,6];
    insertion(&mut input);
}