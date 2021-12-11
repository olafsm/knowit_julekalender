use std::collections::HashMap;

fn main() {
    let input:Vec<char> = include_str!("../input.txt").chars().collect();
    let mut n = 0;
    let mut j = 0;

    let mut values:HashMap<i32, usize> = HashMap::new();
    let mut largest_hood= 0;
    let mut largest_hood_index = 0;
    
    for (i, c) in input.iter().enumerate() {
        let diff = n-j;
        if !values.contains_key(&diff) {
            values.insert(diff, i);
        } else {
            let size_index = *values.get(&diff).unwrap();
            let size = i  - size_index;
            if size > largest_hood {
                largest_hood_index = size_index;
                largest_hood = size;
            }
        }

        match c {
            'N' => n+=1,
            'J' => j+=1,
            _ => {}
        }
    }
    eprintln!("largest_hood = {:?}", largest_hood);
    eprintln!("largest_hood_index = {:?}", largest_hood_index);
}
