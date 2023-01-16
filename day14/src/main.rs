use std::{env,fs,process};
use std::collections::HashMap;

fn add(a: &mut Vec<u64>, b: &Vec<u64>) {
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        a[i] += b[i];
    }
}

fn run(input: &str, steps: u16) -> u64 {
    // Read input
    let mut lines = input.lines();
    let mut num_chars = 0;
    //let polymer = lines.next().unwrap().chars().collect::<Vec<_>>();
    let mut polymer = vec![];
    let mut indices = HashMap::new();
    for c in lines.next().unwrap().chars() {
        if !indices.contains_key(&c) {
            indices.insert(c,num_chars);
            num_chars += 1;
        }
        polymer.push(c);
    }
    lines.next();
    let mut insertions = HashMap::new();
    while let Some(line) = lines.next() {
        let parts = line.split("->").map(|s| s.trim()).collect::<Vec<_>>();
        let lhs = parts[0].chars().collect::<Vec<_>>();
        let rhs = parts[1].chars().next().unwrap();
        insertions.insert((lhs[0],lhs[1]),rhs);
        for c in [lhs[0],lhs[1],rhs].iter() {
            if !indices.contains_key(c) {
                indices.insert(*c,num_chars);
                num_chars += 1;
            }
        }
    }

    // NEW IDEA - DFS
    let mut frequencies = vec![0;num_chars];
    for c in polymer.iter() {
        frequencies[*indices.get(c).unwrap()] += 1;
    }
    // Optimization to trim branches: try to store already known results
    let mut known = HashMap::new();
    for (l,r) in polymer[0..(polymer.len()-1)].iter().zip(polymer[1..].iter()) {
        add(&mut frequencies, &dfs(*l,*r,&insertions, steps, &indices, &mut known));
    }

    frequencies.iter().max().unwrap() - frequencies.iter().min().unwrap()
}

fn dfs(left: char, right: char, insertions: &HashMap<(char,char),char>, rem_steps: u16, indices: &HashMap<char, usize>, known: &mut HashMap<(u16, char, char), Vec<u64>>) -> Vec<u64>{
    if let Some(v) = known.get(&(rem_steps,left,right)) {
        v.clone()
    } else {
        let mut frequencies = vec![0;indices.len()];
        if let Some(c) = insertions.get(&(left,right)) {
            if rem_steps > 1 {
                add(&mut frequencies, &dfs(left,*c,insertions,rem_steps - 1, indices, known));
                add(&mut frequencies, &dfs(*c,right,insertions,rem_steps - 1, indices, known));
            }
            frequencies[*indices.get(c).unwrap()] += 1;
        }
        known.insert((rem_steps, left, right), frequencies.clone());
        frequencies
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

    let res = run(&input,40);
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

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input,40);
    assert_eq!(res,3760312702877);
}
