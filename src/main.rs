fn main() {
    let mut input: Vec<i32> = vec![3,7,8,1,6,6];
    insertion_sort(&mut input);
}

fn insertion_sort(vec: &mut Vec<i32>) {
    println!("In: {:?}", vec);
    for i in 0..vec.len() {
        for j in (0..i).rev() {
            if vec[j+1] < vec[j] {
                vec.swap(j, j+1);
                println!("{:?}", vec);
            }
            else {
                break;
            }
        }
    }
    println!("Out: {:?}", vec);
}
