use std::{env,fs,process};
use std::collections::HashMap;

fn run1(input: &str) -> u32 {
    // Read input
    let mut lines = input.lines();
    let mut polymer = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next();
    let mut insertions = HashMap::new();
    while let Some(line) = lines.next() {
        let parts = line.split("->").map(|s| s.trim()).collect::<Vec<_>>();
        let lhs = parts[0].chars().collect::<Vec<_>>();
        let rhs = parts[1].chars().next().unwrap();
        insertions.insert((lhs[0],lhs[1]),rhs);
    }

    // Insertions
    for _ in 0..10 {
        let mut i = 1;
        let n = polymer.len() - 1;
        let mut to_insert = vec![];
        for (l,r) in polymer[0..n].iter().zip(polymer[1..].iter()) {
            if let Some(c) = insertions.get(&(*l,*r)) {
                to_insert.push((i,c));
            }
            i += 1;
        }
        while let Some((pos,c)) = to_insert.pop() {
            polymer.insert(pos,*c);
        }
    }

    // Count frequencies
    let mut frequencies = HashMap::new();
    for c in polymer.iter() {
        if let Some(n) = frequencies.get_mut(&c) {
            *n += 1;
        } else {
            frequencies.insert(c,1);
        }
    }
    let mut min = polymer.len() as u32;
    let mut max = 0;
    for v in frequencies.values() {
        if min > *v {
            min = *v;
        }
        if max < *v {
            max = *v;
        }
    }
    max - min
}

fn run2(input: &str) -> u64 {
    // Read input
    let mut lines = input.lines();
    let mut polymer = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next();
    let mut insertions = HashMap::new();
    while let Some(line) = lines.next() {
        let parts = line.split("->").map(|s| s.trim()).collect::<Vec<_>>();
        let lhs = parts[0].chars().collect::<Vec<_>>();
        let rhs = parts[1].chars().next().unwrap();
        insertions.insert((lhs[0],lhs[1]),rhs);
    }

    // Insertions
    // TODO: Implement some kind of search for patterns - perhaps we can save partial insertions
    // and reuse them to more efficiently do this step (otherwise this will explode in
    // combinatorics...)
    for step in 0..40 {
        println!("{step}: {}", polymer.len());
        let mut i = 1;
        let n = polymer.len() - 1;
        let mut to_insert = vec![];
        for (l,r) in polymer[0..n].iter().zip(polymer[1..].iter()) {
            if let Some(c) = insertions.get(&(*l,*r)) {
                to_insert.push((i,c));
            }
            i += 1;
        }
        while let Some((pos,c)) = to_insert.pop() {
            polymer.insert(pos,*c);
        }
    }

    // Count frequencies
    let mut frequencies = HashMap::new();
    for c in polymer.iter() {
        if let Some(n) = frequencies.get_mut(&c) {
            *n += 1;
        } else {
            frequencies.insert(c,1);
        }
    }
    let mut min = polymer.len() as u64;
    let mut max = 0;
    for v in frequencies.values() {
        if min > *v {
            min = *v;
        }
        if max < *v {
            max = *v;
        }
    }
    max - min
}

fn main() {
    let mut args = env::args();
    let filepath;
    args.next();
    if let Some(s) = args.next() {
        filepath = s;
    }
    else {
        eprintln!("Give me a file name! I must feeds on files! Aaargh!");
        process::exit(1);
    }

    let input = fs::read_to_string(filepath).unwrap();

    let res = run2(&input);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,1588);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,3306);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,2188189693529);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
