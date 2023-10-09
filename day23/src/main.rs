use std::{env,fs,process};
use std::cmp::Ordering;
use std::collections::{HashSet,HashMap,VecDeque,BinaryHeap};
use std::fmt;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum AmphipodSpecies { A, B, C, D, }

impl AmphipodSpecies {
    fn cost(&self) -> u64 {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }

    fn rooms(&self) -> [u16; 4] {
        match self {
            Self::A => [11, 15, 19, 23],
            Self::B => [12, 16, 20, 24],
            Self::C => [13, 17, 21, 25],
            Self::D => [14, 18, 22, 26],
        }
    }
}

impl From<char> for AmphipodSpecies {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            _ => panic!("Unexpected character"),
        }
    }
}

const NO_DOOR_NODES: [u16; 7] = [0, 1, 3, 5, 7, 9, 10];

struct Node {
    //id: u16,
    neighbours: Vec<u16>,
    room: Option<AmphipodSpecies>,
}

struct Map {
    nodes: HashMap<u16, Node>,
    distances: HashMap<(u16, u16), u64>,
}

impl Map {
    fn new1() -> Self {
        Self {
            nodes: HashMap::from([
                       (0, Node { neighbours: vec![1], room: None }),
                       (1, Node { neighbours: vec![0, 2], room: None }),
                       (2, Node { neighbours: vec![1, 3, 11], room: None }),
                       (3, Node { neighbours: vec![2, 4], room: None }),
                       (4, Node { neighbours: vec![3, 5, 12], room: None }),
                       (5, Node { neighbours: vec![4, 6], room: None }),
                       (6, Node { neighbours: vec![5, 7, 13], room: None }),
                       (7, Node { neighbours: vec![6, 8], room: None }),
                       (8, Node { neighbours: vec![7, 9, 14], room: None }),
                       (9, Node { neighbours: vec![8, 10], room: None }),
                       (10, Node { neighbours: vec![9], room: None }),
                       (11, Node { neighbours: vec![2, 15], room: Some(AmphipodSpecies::A) }),
                       (12, Node { neighbours: vec![4, 16], room: Some(AmphipodSpecies::B) }),
                       (13, Node { neighbours: vec![6, 17], room: Some(AmphipodSpecies::C) }),
                       (14, Node { neighbours: vec![8, 18], room: Some(AmphipodSpecies::D) }),
                       (15, Node { neighbours: vec![11], room: Some(AmphipodSpecies::A) }),
                       (16, Node { neighbours: vec![12], room: Some(AmphipodSpecies::B) }),
                       (17, Node { neighbours: vec![13], room: Some(AmphipodSpecies::C) }),
                       (18, Node { neighbours: vec![14], room: Some(AmphipodSpecies::D) }),
            ]),
            distances: HashMap::new(),
        }
    }

    fn new2() -> Self {
        Self {
            nodes: HashMap::from([
                       (0, Node { neighbours: vec![1], room: None }),
                       (1, Node { neighbours: vec![0, 2], room: None }),
                       (2, Node { neighbours: vec![1, 3, 11], room: None }),
                       (3, Node { neighbours: vec![2, 4], room: None }),
                       (4, Node { neighbours: vec![3, 5, 12], room: None }),
                       (5, Node { neighbours: vec![4, 6], room: None }),
                       (6, Node { neighbours: vec![5, 7, 13], room: None }),
                       (7, Node { neighbours: vec![6, 8], room: None }),
                       (8, Node { neighbours: vec![7, 9, 14], room: None }),
                       (9, Node { neighbours: vec![8, 10], room: None }),
                       (10, Node { neighbours: vec![9], room: None }),
                       (11, Node { neighbours: vec![2, 15], room: Some(AmphipodSpecies::A) }),
                       (12, Node { neighbours: vec![4, 16], room: Some(AmphipodSpecies::B) }),
                       (13, Node { neighbours: vec![6, 17], room: Some(AmphipodSpecies::C) }),
                       (14, Node { neighbours: vec![8, 18], room: Some(AmphipodSpecies::D) }),
                       (15, Node { neighbours: vec![11, 19], room: Some(AmphipodSpecies::A) }),
                       (16, Node { neighbours: vec![12, 20], room: Some(AmphipodSpecies::B) }),
                       (17, Node { neighbours: vec![13, 21], room: Some(AmphipodSpecies::C) }),
                       (18, Node { neighbours: vec![14, 22], room: Some(AmphipodSpecies::D) }),
                       (19, Node { neighbours: vec![15, 23], room: Some(AmphipodSpecies::A) }),
                       (20, Node { neighbours: vec![16, 24], room: Some(AmphipodSpecies::B) }),
                       (21, Node { neighbours: vec![17, 25], room: Some(AmphipodSpecies::C) }),
                       (22, Node { neighbours: vec![18, 26], room: Some(AmphipodSpecies::D) }),
                       (23, Node { neighbours: vec![19], room: Some(AmphipodSpecies::A) }),
                       (24, Node { neighbours: vec![20], room: Some(AmphipodSpecies::B) }),
                       (25, Node { neighbours: vec![21], room: Some(AmphipodSpecies::C) }),
                       (26, Node { neighbours: vec![22], room: Some(AmphipodSpecies::D) }),
            ]),
            distances: HashMap::new(),
        }
    }

    fn distance(&mut self, from: u16, to: u16) -> u64 {
        let mut op = self.distances.get_mut(&(from, to)).map(|&mut d| d);
        *op.get_or_insert_with(|| {
            let mut queue = VecDeque::from([(from, 0)]);
            let mut visited = HashSet::from([from]);
            while let Some((x,d)) = queue.pop_front() {
                if x == to {
                    return d;
                }
                let node = self.nodes.get(&x).unwrap();
                for &n in node.neighbours.iter() {
                    if !visited.contains(&n) {
                        visited.insert(n);
                        queue.push_back((n, d+1));
                    }
                }
            }
            0
        })
    }
}

#[derive(Clone,Debug,Hash,PartialEq,Eq)]
struct Amphipod {
    pos: u16,
    species: AmphipodSpecies,
}

impl Amphipod {
    fn finished(&self) -> bool {
        self.species.rooms().contains(&self.pos)
    }

    fn displace(&self, num: u16) -> Self {
        Self { pos: self.pos + num, species: self.species, }
    }
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.species {
            AmphipodSpecies::A => write!(f, "A"),
            AmphipodSpecies::B => write!(f, "B"),
            AmphipodSpecies::C => write!(f, "C"),
            AmphipodSpecies::D => write!(f, "D"),
        }
    }
}

#[derive(Debug)]
struct Status {
    amphipods: Vec<Amphipod>,
    energy: u64,
}

impl From<Vec<Amphipod>> for Status {
    fn from(vec: Vec<Amphipod>) -> Self {
        Self { amphipods: vec, energy: 0, }
    }
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        other.energy == self.energy
    }
}

impl Eq for Status {}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.energy.cmp(&self.energy))
    }
}

impl Ord for Status {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy.cmp(&self.energy)
    }
}

impl Status {
    fn finished(&self) -> bool {
        for amphipod in self.amphipods.iter() {
            if !amphipod.finished() {
                return false;
            }
        }
        true
    }

    fn extend(&mut self) {
        self.amphipods = self.amphipods.iter().map(|a| {
            a.displace(8)
        }).collect();
        self.amphipods.extend_from_slice(&[
            Amphipod { pos: 15, species: AmphipodSpecies::D },
            Amphipod { pos: 16, species: AmphipodSpecies::C },
            Amphipod { pos: 17, species: AmphipodSpecies::B },
            Amphipod { pos: 18, species: AmphipodSpecies::A },
            Amphipod { pos: 19, species: AmphipodSpecies::D },
            Amphipod { pos: 20, species: AmphipodSpecies::B },
            Amphipod { pos: 21, species: AmphipodSpecies::A },
            Amphipod { pos: 22, species: AmphipodSpecies::C },
        ]);
    }
}



fn move_from_to(from: u16, to: u16, amphipods: &mut Vec<Amphipod>) {
    amphipods.iter_mut().filter(|a| a.pos == from).for_each(|mut a| {a.pos = to;})
}

// Check if the amphipod in position pos is blocking an amphipod of the right type from getting to
// the end of the right room
fn blocking(pos: u16, amphipods: &Vec<Amphipod>, map: &Map) -> bool {
    if let Some(s) = map.nodes.get(&pos).unwrap().room {
        let rooms = s.rooms();
        for a in amphipods {
            if rooms.contains(&a.pos) && a.species != s {
                return true;
            }
        }
    }
    false
}

fn valid_destinations(from: u16, amphipods: &Vec<Amphipod>, map: &Map) -> HashSet<u16> {
    let mut dest = HashSet::new();

    let amph = amphipods.iter().find(|a| a.pos == from).unwrap();

    let node = map.nodes.get(&from).unwrap();
    if node.room.is_some() {
        // Amphipod is in a room
        // Check if amphipod is finished or blocking
        if blocking(amph.pos, amphipods, map) {
            // Not in the right room or blocking. Attempt to move to the corridor
            // Depth-first search
            let mut stack = vec![from];
            let mut visited = HashSet::new();
            while let Some(x) = stack.pop() {
                if NO_DOOR_NODES.contains(&x) {
                    dest.insert(x);
                } else if let Some(a) = map.nodes.get(&x).unwrap().room {
                    if a == amph.species {
                        if !blocking(x, amphipods, map) {
                            dest.insert(x);
                        }
                    }
                }
                if !visited.contains(&x) {
                    visited.insert(x);
                    for &n in map.nodes.get(&x).unwrap().neighbours.iter() {
                        if amphipods.iter().find(|a| a.pos == n).is_none() {
                            // Free node
                            stack.push(n);
                        }
                    }
                }
            }
        }
    } else {
        // Amphipod is in the corridor
        // Attempt to move to the deepest free spot in their room
        // Depth-first search
        let mut stack = vec![from];
        let mut visited = HashSet::new();
        while let Some(x) = stack.pop() {
            // Destination
            if let Some(a) = map.nodes.get(&x).unwrap().room {
                if a == amph.species {
                    if !blocking(x, amphipods, map) {
                        dest.insert(x);
                    }
                }
            }

            if !visited.contains(&x) {
                visited.insert(x);
                for &n in map.nodes.get(&x).unwrap().neighbours.iter() {
                    if amphipods.iter().find(|a| a.pos == n).is_none() {
                        // Free node
                        if let Some(a) = map.nodes.get(&n).unwrap().room {
                            // Do not bother entering rooms that are not own type
                            if a == amph.species {
                                stack.push(n);
                            }
                        } else {
                            stack.push(n);
                        }
                    }
                }
            }
        }
    }

    dest
}

fn run(status: Status, mut map: Map) -> u64 {
    let mut visited = HashSet::new();

    // Solve: A*-search
    let mut heap: BinaryHeap<Status> = BinaryHeap::from([status]);

    while let Some(s) = heap.pop() {
        if !visited.contains(&s.amphipods) {
            visited.insert(s.amphipods.clone());
            debug_map2(&s.amphipods);
            println!("Energy: {}\n", s.energy);

            if s.finished() {
                return s.energy;
            }

            for amph in s.amphipods.iter() {
                for pos in valid_destinations(amph.pos, &s.amphipods, &map) {
                    let mut new_amphs = s.amphipods.clone();
                    let energy = (amph.species.cost() * map.distance(amph.pos, pos)) + s.energy;
                    move_from_to(amph.pos, pos, &mut new_amphs);
                    let new_state = Status { amphipods: new_amphs, energy, };
                    heap.push(new_state);
                }
            }
        }
    }
    0
}

fn run1(input: &str) -> u64 {
    // Read input
    let lines = input.lines().skip(2);
    let mut i = 1;
    let mut placement = Vec::new();
    for line in lines {
        for c in line.chars() {
            if c == 'A' || c == 'B' || c == 'C' || c == 'D' {
                placement.push(Amphipod {pos: 10 + i, species: AmphipodSpecies::from(c), });
                i += 1;
            }
            if i > 18 { break; }
        }
    }
    let status = Status::from(placement);
    let map = Map::new1();
    run(status, map)
}

fn run2(input: &str) -> u64 {
    // Read input
    let lines = input.lines().skip(2);
    let mut i = 1;
    let mut placement = Vec::new();
    for line in lines {
        for c in line.chars() {
            if c == 'A' || c == 'B' || c == 'C' || c == 'D' {
                placement.push(Amphipod {pos: 10 + i, species: AmphipodSpecies::from(c), });
                i += 1;
            }
            if i > 18 { break; }
        }
    }
    let mut status = Status::from(placement);
    status.extend();
    let map = Map::new2();
    run(status, map)
}

fn debug_map1(positions: &Vec<Amphipod>) {
    println!("#############");
    print!("#");
    // Corridor
    for i in 0..11 {
        if let Some(a) = positions.iter().find(|a| a.pos == i) {
            print!("{}", a);
        } else {
            print!(".");
        }
    }
    print!("#\n");
    // Room 1
    print!("##");
    for i in 11..15 {
        print!("#");
        if let Some(a) = positions.iter().find(|a| a.pos == i) {
            print!("{}", a);
        } else {
            print!(".");
        }
    }
    print!("###\n");
    // Room 2
    print!("  ");
    for i in 15..19 {
        print!("#");
        if let Some(a) = positions.iter().find(|a| a.pos == i) {
            print!("{}", a);
        } else {
            print!(".");
        }
    }
    print!("#\n");
    println!("  #########");
}

fn debug_map2(positions: &Vec<Amphipod>) {
    println!("#############");
    print!("#");
    // Corridor
    for i in 0..11 {
        if let Some(a) = positions.iter().find(|a| a.pos == i) {
            print!("{}", a);
        } else {
            print!(".");
        }
    }
    print!("#\n");
    // Room 1
    print!("##");
    for i in 11..15 {
        print!("#");
        if let Some(a) = positions.iter().find(|a| a.pos == i) {
            print!("{}", a);
        } else {
            print!(".");
        }
    }
    print!("###\n");
    // Rooms
    for y in 0..3 {
        print!("  ");
        for x in 0..4 {
            print!("#");
            let i = 15 + y*4 + x;
            if let Some(a) = positions.iter().find(|a| a.pos == i) {
                print!("{}", a);
            } else {
                print!(".");
            }
        }
        print!("#\n");
    }
    println!("  #########");
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
    assert_eq!(res, 12521);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 19160);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 44169);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
