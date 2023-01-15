use std::{env,fs,process};
use std::collections::{HashMap,HashSet,VecDeque};

fn run1(input: &str) -> u32 {
    // Read input
    let mut graph: HashMap<&str,Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let pair: Vec<&str> = line.split('-').collect();
        if let Some(neighbours) = graph.get_mut(pair[0]) {
            neighbours.push(pair[1]);
        } else {
            let mut neighs = vec![];
            neighs.push(pair[1]);
            graph.insert(pair[0],neighs);
        }
        if let Some(neighbours) = graph.get_mut(pair[1]) {
            neighbours.push(pair[0]);
        } else {
            let mut neighs = vec![];
            neighs.push(pair[0]);
            graph.insert(pair[1],neighs);
        }
    }

    // Now, start counting paths!
    let mut stack = vec![];
    stack.push(vec!["start"]);
    let mut paths = 0;
    while let Some(path) = stack.pop() {
        // Artificially remove paths that go for too long as they could go forever
        if path.len() > 3 * graph.len() {
            continue;
        }
        // Get current position
        let current = path.last().unwrap();
        if *current == "end" {
            paths += 1;
        } else {
            let neighs = graph.get(current).unwrap();
            for n in neighs {
                let mut npath = path.clone();
                // Skip this n if it is lowecase and already visited
                if n.chars().all(char::is_lowercase) {
                    if *n == "start" {
                        continue;
                    }
                    let mut found = false;
                    for v in path.iter() {
                        if v == n {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        continue;
                    }
                }
                npath.push(n);
                stack.push(npath);
            }
        }
    }
    paths
}

fn has_duplicates(path: &Vec<&str>) -> bool {
    let mut set = HashSet::new();
    for v in path {
        if set.contains(v) {
            return true;
        }
        if v.chars().all(char::is_lowercase) {
            set.insert(v);
        }
    }
    false
}

fn run2(input: &str) -> u32 {
    // Read input
    let mut graph: HashMap<&str,Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let pair: Vec<&str> = line.split('-').collect();
        if let Some(neighbours) = graph.get_mut(pair[0]) {
            neighbours.push(pair[1]);
        } else {
            let mut neighs = vec![];
            neighs.push(pair[1]);
            graph.insert(pair[0],neighs);
        }
        if let Some(neighbours) = graph.get_mut(pair[1]) {
            neighbours.push(pair[0]);
        } else {
            let mut neighs = vec![];
            neighs.push(pair[0]);
            graph.insert(pair[1],neighs);
        }
    }

    // Now, start counting paths!
    let mut stack = vec![];
    stack.push(vec!["start"]);
    let mut paths = 0;
    while let Some(path) = stack.pop() {
        // Artificially remove paths that go for too long as they could go forever
        if path.len() > 5 * graph.len() {
            continue;
        }
        // Get current position
        let current = path.last().unwrap();
        if *current == "end" {
            paths += 1;
        } else {
            let duplicate = has_duplicates(&path);
            let neighs = graph.get(current).unwrap();
            for n in neighs {
                let mut npath = path.clone();
                if *n == "start" {
                    continue;
                }
                // Skip this n if it is lowecase and already visited and we already visited some
                // small cavern twice
                if duplicate && n.chars().all(char::is_lowercase) {
                    let mut found = false;
                    for v in path.iter() {
                        if v == n {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        continue;
                    }
                }
                npath.push(n);
                stack.push(npath);
            }
        }
    }
    paths
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
fn example11() {
    let input = fs::read_to_string("test1.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,10);
}

#[test]
fn example12() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,19);
}

#[test]
fn example13() {
    let input = fs::read_to_string("test3.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,226);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,3576);
}

#[test]
fn example21() {
    let input = fs::read_to_string("test1.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,36);
}

#[test]
fn example22() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,103);
}

#[test]
fn example23() {
    let input = fs::read_to_string("test3.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,3509);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,84271);
}
