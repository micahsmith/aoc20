use std::collections::HashSet;
use std::iter::FromIterator;

pub fn start(input: &str) {
    let processed = process_input(input);
    let (one, two) = find_pair(&processed);
    println!("{}", one * two);
    let (one, two, three) = find_trio(&processed);
    println!("{}", one * two * three);
}

fn process_input(input: &str) -> HashSet<i32> {
    let vec: Vec<i32> = input
        .split('\n')
        .filter(|&line| line != "")
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    return HashSet::from_iter(vec);
}

fn find_pair(input: &HashSet<i32>) -> (i32, i32) {
    for i in input.iter() {
        let complement = 2020 - i;
        if input.contains(&complement) {
            return (*i, complement);
        }
    }
    panic!("Pair not found");
}

fn find_trio(input: &HashSet<i32>) -> (i32, i32, i32) {
    for i in input.iter() {
        for j in input.iter() {
            let complement = 2020 - i - j;
            if input.contains(&complement) {
                return (*i, *j, complement);
            }
        }
    }
    panic!("Trio not found");
}
