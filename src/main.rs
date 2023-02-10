use std::collections::HashMap;

/*
    Given a list of integers, return the median and mode.
    Use vectors and hashmaps
*/

fn main() {

    let mut vec = vec![1, 3, 45, 6, 77, 77, 4, 33, 65];

    // sorting the vector
    vec.sort();

    let vec_mid = vec.len() / 2;
    // print the mod
    if vec.len() % 2 != 0 {
        println!("{}", vec[vec_mid] );
    }
    else {
        println!("{}", (vec[vec_mid] + vec[vec_mid + 1]) / 2);
    }

    // return mode of th list
    let mut map = HashMap::new();

    for i in vec {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut key = 0;

    for (k, v) in map {
        if max < v {
            max = v;
            key = k;
        }
    }

    println!("mode is {}", key);
}