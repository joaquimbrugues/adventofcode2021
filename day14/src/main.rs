use std::{env,fs,process};
use std::collections::HashMap;

fn run(input: &str, steps: u16) -> u64 {
    // Read input
    let mut lines = input.lines();
    let polymer = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next();
    let mut insertions = HashMap::new();
    while let Some(line) = lines.next() {
        let parts = line.split("->").map(|s| s.trim()).collect::<Vec<_>>();
        let lhs = parts[0].chars().collect::<Vec<_>>();
        let rhs = parts[1].chars().next().unwrap();
        insertions.insert((lhs[0],lhs[1]),rhs);
    }

    // NEW IDEA - DFS
    let mut frequencies = HashMap::new();
    for c in polymer.iter() {
        if let Some(k) = frequencies.get_mut(c) {
            *k += 1;
        } else {
            frequencies.insert(*c,1);
        }
    }
    for (l,r) in polymer[0..(polymer.len()-1)].iter().zip(polymer[1..].iter()) {
        dfs(*l,*r,&insertions, steps, &mut frequencies,);
    }

    println!("{frequencies:?}");
    frequencies.values().max().unwrap() - frequencies.values().min().unwrap()
}

fn dfs(left: char, right: char, insertions: &HashMap<(char,char),char>, rem_steps: u16, frequencies: &mut HashMap<char, u64>) {
    if let Some(c) = insertions.get(&(left,right)) {
        if rem_steps > 1 {
            dfs(left,*c,insertions,rem_steps - 1, frequencies);
            dfs(*c,right,insertions,rem_steps - 1, frequencies);
        }
        if let Some(k) = frequencies.get_mut(c) {
            *k += 1;
        } else {
            frequencies.insert(*c,1);
        }
    }
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

    let res = run(&input,2);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run(&input,10);
    assert_eq!(res,1588);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input,10);
    assert_eq!(res,3306);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run(&input,40);
    assert_eq!(res,2188189693529);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
