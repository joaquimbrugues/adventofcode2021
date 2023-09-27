use std::{env,fs,process};
use std::collections::{HashSet,HashMap,VecDeque};

#[derive(PartialEq, Eq, Debug, Hash)]
enum Amphipod { A, B, C, D, }

const ROOM_NODES_ARRAY: [(Amphipod, u16); 8] = [(Amphipod::A, 11), (Amphipod::A, 15), (Amphipod::B, 12), (Amphipod::B, 16), (Amphipod::C, 13), (Amphipod::C, 17), (Amphipod::D, 14), (Amphipod::D, 18)];

struct Node {
    id: u16,
    neighbours: Vec<u16>,
    room: Option<Amphipod>,
}

fn no_door_nodes() -> [u16;7] {
    [0, 1, 3, 5, 7, 9, 10]
}

fn door_nodes() -> HashMap<Amphipod, u16> {
    HashMap::from(ROOM_NODES_ARRAY)
}

struct Map {
    nodes: HashMap<u16, Node>,
    distances: HashMap<(u16,u16), u16>,
}

impl Map {
    fn default_map() -> Map {
        Self {
            nodes: HashMap::from([
                       (0, Node { id: 0, neighbours: vec![1], room: None }),
                       (1, Node { id: 1, neighbours: vec![0, 2], room: None }),
                       (2, Node { id: 2, neighbours: vec![1, 3, 11], room: None }),
                       (3, Node { id: 3, neighbours: vec![2, 4], room: None }),
                       (4, Node { id: 4, neighbours: vec![3, 5, 12], room: None }),
                       (5, Node { id: 5, neighbours: vec![4, 6], room: None }),
                       (6, Node { id: 6, neighbours: vec![5, 7, 13], room: None }),
                       (7, Node { id: 7, neighbours: vec![6, 8], room: None }),
                       (8, Node { id: 8, neighbours: vec![7, 9, 14], room: None }),
                       (9, Node { id: 9, neighbours: vec![8, 10], room: None }),
                       (10, Node { id: 10, neighbours: vec![9], room: None }),
                       (11, Node { id: 11, neighbours: vec![2, 15], room: Some(Amphipod::A) }),
                       (12, Node { id: 12, neighbours: vec![4, 16], room: Some(Amphipod::B) }),
                       (13, Node { id: 13, neighbours: vec![6, 17], room: Some(Amphipod::C) }),
                       (14, Node { id: 14, neighbours: vec![8, 18], room: Some(Amphipod::D) }),
                       (15, Node { id: 15, neighbours: vec![11], room: Some(Amphipod::A) }),
                       (16, Node { id: 16, neighbours: vec![12], room: Some(Amphipod::B) }),
                       (17, Node { id: 17, neighbours: vec![13], room: Some(Amphipod::C) }),
                       (18, Node { id: 18, neighbours: vec![14], room: Some(Amphipod::D) }),
            ]),
            distances: HashMap::new(),
        }
    }

    fn distance(&mut self, x: u16, y: u16) -> u16 {
        if x == y {
            0
        } else if let Some(&d) = self.distances.get(&(x,y)) {
            d
        } else {
            // Breadth-first search
            let mut queue = VecDeque::from([(x, 0)]);
            let mut visited = HashSet::from([x]);
            while let Some((l, dist)) = queue.pop_front() {
                if !self.distances.contains_key(&(x,l)) {
                    self.distances.insert((x,l), dist);
                    self.distances.insert((l,x), dist);
                }
                if l == y {
                    return dist;
                }
                for &n in self.nodes.get(&l).unwrap().neighbours.iter() {
                    if !visited.contains(&n) {
                        visited.insert(n);
                        queue.push_back((n, dist+1));
                    }
                }
            }
            0
        }
    }

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

    println!("{placement:?}");

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
