use std::{env,fs,process};

enum Amphipod { A, B, C, D, }

struct Node {
    id: u16,
    neighbours: Vec<u16>,
    room: Option<Amphipod>,
}

type Map = Vec<Node>;

fn get_map() -> Map {
    vec![
        Node {
            id: 0,
            neighbours: vec![1],
            room: None,
        },
        Node {
            id: 1,
            neighbours: vec![0, 2],
            room: None,
        },
        Node {
            id: 2,
            neighbours: vec![1, 3, 11],
            room: None,
        },
        Node {
            id: 3,
            neighbours: vec![2, 4],
            room: None,
        },
        Node {
            id: 4,
            neighbours: vec![3, 5, 12],
            room: None,
        },
        Node {
            id: 5,
            neighbours: vec![4, 6],
            room: None,
        },
        Node {
            id: 6,
            neighbours: vec![5, 7, 13],
            room: None,
        },
        Node {
            id: 7,
            neighbours: vec![6, 8],
            room: None,
        },
        Node {
            id: 8,
            neighbours: vec![7, 9, 14],
            room: None,
        },
        Node {
            id: 9,
            neighbours: vec![8, 10],
            room: None,
        },
        Node {
            id: 10,
            neighbours: vec![9],
            room: None,
        },
        Node {
            id: 11,
            neighbours: vec![2, 15],
            room: Some(Amphipod::A),
        },
        Node {
            id: 12,
            neighbours: vec![4, 16],
            room: Some(Amphipod::B),
        },
        Node {
            id: 13,
            neighbours: vec![6, 17],
            room: Some(Amphipod::C),
        },
        Node {
            id: 14,
            neighbours: vec![8, 18],
            room: Some(Amphipod::D),
        },
        Node {
            id: 15,
            neighbours: vec![11],
            room: Some(Amphipod::A),
        },
        Node {
            id: 16,
            neighbours: vec![12],
            room: Some(Amphipod::B),
        },
        Node {
            id: 17,
            neighbours: vec![13],
            room: Some(Amphipod::C),
        },
        Node {
            id: 18,
            neighbours: vec![14],
            room: Some(Amphipod::D),
        },
    ]
}

fn run1(input: &str) -> u32 {
    // Read input
    let lines = input.lines().skip(2);
    let mut i = 1;
    let mut placement = vec![];
    for line in lines {
        for c in line.chars() {
            match c {
                'A' => {
                    placement.push((10 +i, Amphipod::A));
                    i += 1;
                },
                'B' => {
                    placement.push((10 +i, Amphipod::B));
                    i += 1;
                },
                'C' => {
                    placement.push((10 +i, Amphipod::C));
                    i += 1;
                },
                'D' => {
                    placement.push((10 +i, Amphipod::D));
                    i += 1;
                },
                _ => {},
            }
            if i > 18 { break; }
        }
    }

    // Init algorithm

    // Weighted search
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
    assert_eq!(res, 12521);
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
