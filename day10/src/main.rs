use std::{env,fs,process};
use std::collections::HashMap;

fn run1(input: &str) -> u32 {
    let scores = HashMap::from([(')',3),(']',57),('}',1197),('>',25137)]);
    let mut sum = 0;
    for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
            } else {
                if let Some(&o) = stack.last() {
                    if (o == '(' && c == ')') || (o == '[' && c == ']') || (o == '{' && c == '}') || (o == '<' && c == '>') {
                        stack.pop();
                    } else {
                        sum += scores.get(&c).unwrap();
                        break;
                    }
                } else {
                    continue;
                }
            }
        }
    }
    sum
}

fn run2(input: &str) -> u64 {
    let mut scores = vec![];
    for line in input.lines() {
        let mut stack = vec![];
        let mut correct_syntax = true;
        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
            } else {
                if let Some(&o) = stack.last() {
                    if (o == '(' && c == ')') || (o == '[' && c == ']') || (o == '{' && c == '}') || (o == '<' && c == '>') {
                        stack.pop();
                    } else {
                        correct_syntax = false;
                        break;
                    }
                } else {
                    continue;
                }
            }
        }
        if correct_syntax {
            let table = HashMap::from([('(',1),('[',2),('{',3),('<',4)]);
            let mut score: u64 = 0;
            while let Some(o) = stack.pop() {
                score *= 5;
                score += table.get(&o).unwrap();
            }
            let mut i = 0;
            while i < scores.len() && score > scores[i] {
                i += 1;
            }
            scores.insert(i,score);
        }
    }
        
    let i = scores.len() / 2;
    scores[i]
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
    assert_eq!(res,26397);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,167379);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,288957);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,2776842859);
}
