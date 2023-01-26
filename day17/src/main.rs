use std::{env,fs,process};

fn run1(input: &str) -> u32 {
    // Parse input
    let second_part = input.trim().split(": ").skip(1).next().unwrap();
    let mut xmin = 0;
    let mut xmax = 0;
    let mut ymin = 0;
    let mut ymax = 0;
    for s1 in second_part.split(", ") {
        let parts = s1.split('=').collect::<Vec<&str>>();
        let values: Vec<i32> = parts[1].split("..").map(|s| s.parse::<i32>().unwrap()).collect();
        match parts[0] {
            "x" => {
                xmin = values[0];
                xmax = values[1];
            },
            "y" => {
                ymin = values[0];
                ymax = values[1];
            },
            _ => panic!("Unexpected value in input"),
        }
    }
    0
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
    assert_eq!(res,45);
}

//#[test]
//fn input1() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run1(&input);
    //assert_eq!(res,42);
//}

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
