use std::{env,fs,process};
use std::collections::HashSet;

fn run1(input: &str) -> usize {
    let mut points = HashSet::new();
    let mut parts = input.split("\n\n");

    // Read input
    for line in parts.next().unwrap().lines() {
        let coordinates = line.split(',').collect::<Vec<_>>();
        points.insert((coordinates[0].parse::<u32>().unwrap(),coordinates[1].parse::<u32>().unwrap()));
    }

    // First fold:
    let line = parts.next().unwrap().lines().next().unwrap();
    let string = line.split(' ').last().unwrap().split('=').collect::<Vec<_>>();
    let fold = string[1].parse::<u32>().unwrap();
    let coord = match string[0] {
        "x" => 0,
        "y" => 1,
        s => panic!("Unexpected input {s}"),
    };

    let to_remove: Vec<(u32,u32)> = points.iter().filter(|&p| match coord {
        0 => p.0 > fold,
        1 => p.1 > fold,
        _ => panic!("Panic!"),
    }).map(|&p| p).collect();
    for p in to_remove {
        match coord {
            0 => {
                points.insert((2 * fold - p.0, p.1));
            },
            1 => {
                points.insert((p.0, 2 * fold - p.1));
            },
            _ => panic!("Panic!"),
        }
        points.remove(&p);
    }
    points.len()
}

fn display_points(points: &HashSet<(u32,u32)>, max_x: u32, max_y: u32) {
    for y in 0..max_y {
        let mut line = String::from("");
        for x in 0..max_x {
            if points.contains(&(x,y)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{line}");
    }
}

fn run2(input: &str) {
    let mut points = HashSet::new();
    let mut parts = input.split("\n\n");
    let mut max_x = 100000;
    let mut max_y = 100000;

    // Read input
    for line in parts.next().unwrap().lines() {
        let coordinates = line.split(',').collect::<Vec<_>>();
        let x = coordinates[0].parse::<u32>().unwrap();
        if max_x < x {
            max_x = x;
        }
        let y = coordinates[1].parse::<u32>().unwrap();
        if max_y < y {
            max_y = y;
        }
        points.insert((x,y));
    }

    // Fold:
    for line in parts.next().unwrap().lines() {
        let string = line.split(' ').last().unwrap().split('=').collect::<Vec<_>>();
        let fold = string[1].parse::<u32>().unwrap();
        let coord = match string[0] {
            "x" => {
                max_x = fold;
                0
            },
            "y" => {
                max_y = fold;
                1
            },
            s => panic!("Unexpected input {s}"),
        };

        let to_remove: Vec<(u32,u32)> = points.iter().filter(|&p| match coord {
            0 => p.0 > fold,
            1 => p.1 > fold,
            _ => panic!("Panic!"),
        }).map(|&p| p).collect();
        for p in to_remove {
            match coord {
                0 => {
                    points.insert((2 * fold - p.0, p.1));
                },
                1 => {
                    points.insert((p.0, 2 * fold - p.1));
                },
                _ => panic!("Panic!"),
            }
            points.remove(&p);
        }
    }
    display_points(&points,max_x,max_y);
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

    //let res = run1(&input);
    //println!("{res}");
    run2(&input);
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,17);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,802);
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
