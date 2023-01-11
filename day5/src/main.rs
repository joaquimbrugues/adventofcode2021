use std::{env,fs,process};
use std::collections::HashMap;

fn gcd(a: i32, b: i32) -> i32 {
    let a = a.abs();
    let b = b.abs();
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn get_vector(p1: &(i32,i32), p2: &(i32,i32)) -> (i32,i32) {
    let mut v = (p2.0 - p1.0, p2.1 - p1.1);
    let mcd = gcd(v.0,v.1);
    v = (v.0 / mcd, v.1 / mcd);
    v
}

fn run(input: &str, include_diagonal: bool) -> usize {
    let mut map = HashMap::new();
    for line in input.lines() {
        // Read input in line
        let spts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
        let sp1: Vec<&str> = spts[0].split(',').collect();
        let point1 = (sp1[0].parse::<i32>().unwrap(), sp1[1].parse::<i32>().unwrap());
        let sp2: Vec<&str> = spts[1].split(',').collect();
        let point2 = (sp2[0].parse::<i32>().unwrap(), sp2[1].parse::<i32>().unwrap());

        // Filter out pairs that are not horizontally/vertically aligned
        if !include_diagonal && point1.0 != point2.0 && point1.1 != point2.1 {
            continue;
        }
        // Get points inside of line segment
        let v = get_vector(&point1,&point2);
        let mut point = point1;
        loop {
            let end = point == point2;
            if map.contains_key(&point) {
                let a = map.get_mut(&point).unwrap();
                *a += 1;
            } else {
                map.insert(point,1);
            }
            if end {
                break;
            }
            point = (point.0 + v.0, point.1 + v.1);
        }
    }
    map.iter().filter(|(_,&n)| n > 1).collect::<Vec<_>>().len()
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

    let res = run(&input,true);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run(&input,false);
    assert_eq!(res,5);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input,false);
    assert_eq!(res,6461);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run(&input,true);
    assert_eq!(res,12);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input,true);
    assert_eq!(res,18065);
}
