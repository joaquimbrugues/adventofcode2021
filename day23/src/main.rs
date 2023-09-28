use std::{env,fs,process};
use std::collections::{HashSet,HashMap,VecDeque};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Amphipod { A, B, C, D, }

impl Amphipod {
    fn cost(&self) -> u64 {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }
}

const NO_DOOR_NODES: [u16; 7] = [0, 1, 3, 5, 7, 9, 10];

struct Node {
    //id: u16,
    neighbours: Vec<u16>,
    room: Option<Amphipod>,
}

struct Map {
    nodes: HashMap<u16, Node>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            nodes: HashMap::from([
                       (0, Node { /*id: 0,*/ neighbours: vec![1], room: None }),
                       (1, Node { /*id: 1,*/ neighbours: vec![0, 2], room: None }),
                       (2, Node { /*id: 2,*/ neighbours: vec![1, 3, 11], room: None }),
                       (3, Node { /*id: 3,*/ neighbours: vec![2, 4], room: None }),
                       (4, Node { /*id: 4,*/ neighbours: vec![3, 5, 12], room: None }),
                       (5, Node { /*id: 5,*/ neighbours: vec![4, 6], room: None }),
                       (6, Node { /*id: 6,*/ neighbours: vec![5, 7, 13], room: None }),
                       (7, Node { /*id: 7,*/ neighbours: vec![6, 8], room: None }),
                       (8, Node { /*id: 8,*/ neighbours: vec![7, 9, 14], room: None }),
                       (9, Node { /*id: 9,*/ neighbours: vec![8, 10], room: None }),
                       (10, Node { /*id: 10,*/ neighbours: vec![9], room: None }),
                       (11, Node { /*id: 11,*/ neighbours: vec![2, 15], room: Some(Amphipod::A) }),
                       (12, Node { /*id: 12,*/ neighbours: vec![4, 16], room: Some(Amphipod::B) }),
                       (13, Node { /*id: 13,*/ neighbours: vec![6, 17], room: Some(Amphipod::C) }),
                       (14, Node { /*id: 14,*/ neighbours: vec![8, 18], room: Some(Amphipod::D) }),
                       (15, Node { /*id: 15,*/ neighbours: vec![11], room: Some(Amphipod::A) }),
                       (16, Node { /*id: 16,*/ neighbours: vec![12], room: Some(Amphipod::B) }),
                       (17, Node { /*id: 17,*/ neighbours: vec![13], room: Some(Amphipod::C) }),
                       (18, Node { /*id: 18,*/ neighbours: vec![14], room: Some(Amphipod::D) }),
            ]),
        }
    }
}

impl Map {
    // Return all possible destinations for the Amphipod from the node according to the rules.
    // In particular, move only through unnoccupied cells (according to Status) and only perform
    // one of the following trajectories:
    //  1- If we are in a room, move into a node in the hallway not directly in front of a door OR
    //  to the destination room
    //  2- If we are in the hallway, move into a node in the room of the type of the Amphipod
    fn destinations(&self, status: &Status, node: &(u16, Amphipod)) -> Vec<(u16, u64)> {
        let mut res = vec![];
        let mut queue = VecDeque::from([(node.0, 0)]);
        let mut visited = HashSet::from([node.0]);
        let vertex = self.nodes.get(&node.0).unwrap();
        match &vertex.room {
            Some(t) => {
                let no_door = HashSet::from(NO_DOOR_NODES);
                // We are in a room
                if *t != node.1 || vertex.neighbours.len() > 1 {
                    // We should move, use breadth-first search
                    while let Some((id, energy)) = queue.pop_front() {
                        //println!("Going from {}. Considering {id}.", node.0);
                        let vertex = self.nodes.get(&id).unwrap();
                        if let Some(t) = &vertex.room {
                            if *t == node.1 && node.0 != id {
                                // Possible destination (target room)
                                res.push((id, energy));
                            }
                        }
                        if no_door.contains(&id) {
                            // Possible destination (corridor)
                            res.push((id, energy));
                        }
                        for n in vertex.neighbours.iter() {
                            if !visited.contains(n) && !status.positions.contains_key(n) {
                                visited.insert(*n);
                                queue.push_back((*n, energy + node.1.cost()));
                            }
                        }
                    }
                }
            },
            None => {
                // We are in the hallway, we move. Use breadth-first search
                while let Some((id, energy)) = queue.pop_front() {
                    let vertex = self.nodes.get(&id).unwrap();
                    if let Some(t) = &vertex.room {
                        if *t == node.1 {
                            // Valid destination
                            res.push((id, energy));
                        }
                    }
                    for n in vertex.neighbours.iter() {
                        if !visited.contains(n) && !status.positions.contains_key(n) {
                            visited.insert(*n);
                            queue.push_back((id, energy + node.1.cost()));
                        }
                    }
                }
            },
        }
        res
    }

    fn finished(&self, status: &Status) -> bool {
        for (id, t) in status.positions.iter() {
            if let Some(tt) = &self.nodes.get(id).unwrap().room {
                if t != tt {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct Status {
    positions: HashMap<u16, Amphipod>,
    energy: u64,
}

impl From<HashMap<u16, Amphipod>> for Status {
    fn from(hash: HashMap<u16, Amphipod>) -> Self {
        Self { positions: hash, energy: 0, }
    }
}

impl Status {
    fn update(&self, from: u16, to: u16, cost: u64) -> Self{
        let mut new_pos = self.positions.clone();
        let t = self.positions.get(&from).unwrap();
        new_pos.remove(&from);
        new_pos.insert(to, *t);
        Self { positions: new_pos, energy: self.energy + cost }
    }
}

fn insert_queue(queue: &mut VecDeque<Status>, status: Status) {
    let mut i = 0;
    while i < queue.len() && queue[i].energy < status.energy {
        i += 1;
    }
    queue.insert(i,status);
}

fn run1(input: &str) -> u64 {
    // Read input
    let lines = input.lines().skip(2);
    let mut i = 1;
    let mut placement = HashMap::new();
    for line in lines {
        for c in line.chars() {
            match c {
                'A' => {
                    placement.insert(10 +i, Amphipod::A);
                    i += 1;
                },
                'B' => {
                    placement.insert(10 +i, Amphipod::B);
                    i += 1;
                },
                'C' => {
                    placement.insert(10 +i, Amphipod::C);
                    i += 1;
                },
                'D' => {
                    placement.insert(10 +i, Amphipod::D);
                    i += 1;
                },
                _ => {},
            }
            if i > 18 { break; }
        }
    }
    let status = Status::from(placement);
    // Init algorithm
    let mut queue = VecDeque::from([status]);
    let map = Map::default();

    // Weighted search
    while let Some(status) = queue.pop_front() {
        println!("{status:?}");
        if map.finished(&status) {
            println!("Finished!");
            return status.energy;
        }
        for (id, t) in status.positions.iter() {
            let dest = map.destinations(&status, &(*id, *t));
            for (new_pos, cost) in dest {
                let new_stat = status.update(*id, new_pos, cost);
                insert_queue(&mut queue, new_stat);
            }
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
