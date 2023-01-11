use std::{env,fs,process};

fn run1(input: &str) -> i32 {
    let mut max = 0;
    let mut crabs = vec![];
    for s in input.trim().split(',') {
        let a = s.parse::<i32>().unwrap();
        if max < a {
            max = a;
        }
        crabs.push(a);
    }
    let mut min : i32 = crabs.iter().sum();
    for x in 0..max {
        let mut sum = 0;
        for crab in crabs.iter() {
            sum += (x - crab).abs();
        }
        if min > sum {
            min = sum;
        }
    }
    min
}

fn run2(input: &str) -> u32 {
    0
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

    let res = run1(&input);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,37);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,342641);
}

//#[test]
//fn example2() {
    //let input = fs::read_to_string("test.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
