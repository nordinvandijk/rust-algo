pub fn insertion(vec: &mut Vec<u64>) {
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

pub fn selection(vec: &mut Vec<u64>) {
    println!("In: {:?}", vec);
    for i in 0..vec.len() {
        let mut minIndex = i;
        for j in i..vec.len() {
            if vec[j] < vec[minIndex] {
                minIndex = j;
            }
        }
        vec.swap(i, minIndex)
    }
    println!("Out: {:?}", vec);
}