use std::{env,fs,process};

fn run1(input: &str) -> u32 {
    let mut posh = 0;
    let mut posv = 0;
    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let num: u32 = words[1].parse().unwrap();
        match words[0] {
            "forward" => posh += num,
            "down" => posv += num,
            "up" => posv -= num,
            s => panic!("Unexpected input {s}"),
        }
    }
    posh * posv
}

fn run2(input: &str) -> u32 {
    let mut posh = 0;
    let mut posv = 0;
    let mut aim = 0;
    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let num: u32 = words[1].parse().unwrap();
        match words[0] {
            "down" => aim += num,
            "up" => aim -= num,
            "forward" => {
                posh += num;
                posv += num * aim;
            },
            s => panic!("Unexpected input {s}"),
        }
    }
    posh * posv
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
    assert_eq!(res,150);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,1451208);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,900);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,1620141160);
}
