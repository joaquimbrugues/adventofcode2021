use std::{env,fs,process};

// We will save the fish in an array with 9 positions, where the position is the timer of the fish
// and its contents is the number of fish at that age
fn run(input: &str, days: usize) -> u64 {
    let mut fish = [0;9];
    for s in input.trim().split(',') {
        let i: usize = s.parse().unwrap();
        fish[i] += 1;
    }
    for _ in 0..days {
        let mut nfish = [0;9];
        nfish[8] = fish[0];
        nfish[6] = fish[0];
        for i in 1..9 {
            nfish[i-1] += fish[i];
        }
        fish = nfish;
    }
    //println!("{fish:?}");
    fish.iter().sum()
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

    let res = run(&input,256);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res1 = run(&input,18);
    assert_eq!(res1,26);
    let res2 = run(&input,80);
    assert_eq!(res2,5934);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input,80);
    assert_eq!(res,396210);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run(&input,256);
    assert_eq!(res,26984457539);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input,256);
    assert_eq!(res,1770823541496);
}
