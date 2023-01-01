use std::{env,fs,process};
use std::collections::VecDeque;

fn run1(input: &str) -> u32 {
    let mut inc = 0;
    let mut last = None;
    for line in input.lines() {
        let num = line.parse::<u32>().unwrap();
        match last {
            Some(l) => {
                if l < num {
                    inc += 1;
                }
            },
            None => {},
        }
        last = Some(num);
    }
    inc
}

fn run2(input: &str) -> u32 {
    let mut inc = 0;
    let mut last: VecDeque<u32> = VecDeque::new();
    for line in input.lines() {
        let num = line.parse::<u32>().unwrap();
        if last.len() == 3 {
            let sum1: u32 = last.iter().sum();
            last.pop_front();
            let sum2 = last.iter().sum::<u32>() + num;
            if sum1 < sum2 {
                inc += 1;
            }
        }
        last.push_back(num);
    }
    inc
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
    assert_eq!(res,7);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,1121);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,5);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,1065);
}
