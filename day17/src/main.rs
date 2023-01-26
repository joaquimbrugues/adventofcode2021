use std::{env,fs,process};

fn min_vx(minx: i32) -> i32 {
    let mut vx = 0;
    loop {
        vx += 1;
        let mut x = 0;
        for i in 0..vx {
            x += vx - i;
            if x >= minx {
                return vx;
            }
        }
    }
}

fn shoot(vx: i32, vy: i32, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> Option<i32> {
    let (mut x, mut y) = (0, 0);
    let (mut vx, mut vy) = (vx, vy);
    let mut max_height = 0;
    loop {
        x += vx;
        y += vy;
        if vx > 0 {
            vx -= 1;
        }
        vy -= 1;

        if max_height < y {
            max_height = y;
        }

        if x > xmax || y < ymin {
            return None;
        }

        // By bypassing the last "if", we know that x <= xmax && y >= ymin
        if x >= xmin && y <= ymax {
            return Some(max_height);
        }
    }
}

fn run1(input: &str) -> i32 {
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
    let minvx = min_vx(xmin);
    let mut height = ymin;
    for vx in minvx..=xmax {
        for vy in ymin..(-ymin) {
            if let Some(h) = shoot(vx,vy,xmin,xmax,ymin,ymax) {
                if height < h {
                    height = h;
                }
            }
        }
    }
    height
}

fn run2(input: &str) -> u32 {
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
    let minvx = min_vx(xmin);
    let mut count_initial = 0;
    for vx in minvx..=xmax {
        for vy in ymin..=(2 * (- ymin)) {
            if shoot(vx,vy,xmin,xmax,ymin,ymax).is_some() {
                count_initial += 1;
            }
        }
    }
    count_initial
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
    assert_eq!(res,45);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,8646);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,112);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,5945);
}
