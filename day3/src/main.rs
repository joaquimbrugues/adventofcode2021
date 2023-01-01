use std::{env,fs,process};

fn run1(input: &str) -> u32 {
    let mut ones = vec![];
    let mut num_lines = 0;
    for line in input.lines() {
        let mut i = 0;
        for c in line.chars() {
            match c {
                '1' => {
                    while i > ones.len() {
                        ones.push(0);
                    }
                    if i == ones.len() {
                        ones.push(1);
                    } else {
                        ones[i] += 1;
                    }
                },
                '0' => {},
                x => panic!("Unexpected character {x}"),
            }
            i += 1;
        }
        num_lines += 1;
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for n in ones {
        gamma *= 2;
        epsilon *= 2;
        if n > (num_lines / 2) {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    gamma * epsilon
}

fn vbooltoint(vec: &Vec<bool>) -> u32 {
    let mut num = 0;
    for b in vec {
        num *= 2;
        if *b {
            num += 1;
        }
    }
    num
}

fn run2(input: &str) -> u32 {
    let mut oxigens = vec![];
    let mut co2s = vec![];
    for line in input.lines() {
        let mut num = vec![];
        for c in line.chars() {
            match c {
                '1' => num.push(true),
                '0' => num.push(false),
                x => panic!("Unexpected character {x}"),
            }
        }
        oxigens.push(num.clone());
        co2s.push(num);
    }
    let mut i = 0;
    while oxigens.len() > 1 && i < oxigens[0].len() {
        // Get most common digit
        let mut ones = 0;
        for v in &oxigens {
            if v[i] {
                ones += 1;
            }
        }
        // Mark numbers for removal
        let mut torem = vec![];
        let mut j = 0;
        for v in &oxigens {
            if v[i] ^ (2 * ones >= oxigens.len()) {
                torem.push(j);
            }
            j += 1;
        }
        // Remove
        while let Some(j) = torem.pop() {
            oxigens.remove(j);
        }
        i += 1;
    }
    i = 0;
    while co2s.len() > 1 && i < co2s[0].len() {
        // Get most common digit
        let mut ones = 0;
        for v in &co2s {
            if v[i] {
                ones += 1;
            }
        }
        // Mark numbers for removal
        let mut torem = vec![];
        let mut j = 0;
        for v in &co2s {
            if !(v[i] ^ (2 * ones >= co2s.len())) {
                torem.push(j);
            }
            j += 1;
        }
        // Remove
        while let Some(j) = torem.pop() {
            co2s.remove(j);
        }
        i += 1;
    }
    vbooltoint(&oxigens[0]) * vbooltoint(&co2s[0])
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
    assert_eq!(res,198);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,3009600);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,230);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,6940518);
}
