use std::{env,fs,process};
use std::collections::{BinaryHeap,HashMap,HashSet};
use std::cmp::Ordering;

#[derive(Debug,Eq)]
struct WPoint {
    weight: u64,
    x: usize,
    y: usize,
}

impl WPoint {
    fn new(weight: u64, x: usize, y: usize) -> Self {
        Self{ weight, x, y, }
    }
}

impl Ord for WPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for WPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WPoint {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight && self.x == other.x && self.y == other.y
    }
}

fn run1(input: &str) -> u64 {
    // Read input
    let mut cave = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u64);
        }
        cave.push(row);
    }
    // A*-search
    let mut heap: BinaryHeap<WPoint> = BinaryHeap::new();
    heap.push(WPoint::new(0, 0, 0));
    let mut visited = HashSet::new();
    //println!("({},{})", cave[cave.len() - 1].len() - 1, cave.len() - 1);
    while let Some(point) = heap.pop() {
        //if heap.len() > 7 {
            //break;
        //}
        if point.y == cave.len() - 1 && point.x == cave[point.y].len() - 1 {
            return point.weight;
        }
        if !visited.contains(&(point.x,point.y)) {
            visited.insert((point.x,point.y));
            if point.x > 0 {
                let p = (point.x - 1, point.y);
                heap.push(WPoint::new(point.weight + cave[p.1][p.0], p.0, p.1));
            }
            if point.x < cave[point.y].len() - 1 {
                let p = (point.x + 1, point.y);
                heap.push(WPoint::new(point.weight + cave[p.1][p.0], p.0, p.1));
            }
            if point.y > 0 {
                let p = (point.x, point.y - 1);
                heap.push(WPoint::new(point.weight + cave[p.1][p.0], p.0, p.1));
            }
            if point.y < cave.len() - 1 {
                let p = (point.x, point.y + 1);
                heap.push(WPoint::new(point.weight + cave[p.1][p.0], p.0, p.1));
            }

        }
    }
    0
}

fn multiply(map: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut cave = vec![vec![0; map.len() * 5]; map.len() * 5];
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..map.len() {
                for l in 0..map[k].len() {
                    cave[i * map.len() + k][j * map[k].len() + l] = (map[k][l] + ((i + j) as u64) - 1) % 9 + 1;
                }
            }
        }
    }
    cave
}

fn run2(input: &str) -> u64 {
    // Read input
    let mut cave = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u64);
        }
        cave.push(row);
    }
    // Multiply the cave by 5
    cave = multiply(&cave);
    // A*-search
    let mut heap: BinaryHeap<WPoint> = BinaryHeap::new();
    heap.push(WPoint::new(0, 0, 0));
    let mut visited = HashSet::new();
    //println!("({},{})", cave[cave.len() - 1].len() - 1, cave.len() - 1);
    while let Some(point) = heap.pop() {
        //if heap.len() > 7 {
            //break;
        //}
        if point.y == cave.len() - 1 && point.x == cave[point.y].len() - 1 {
            return point.weight;
        }
        if !visited.contains(&(point.x,point.y)) {
            visited.insert((point.x,point.y));
            if point.x > 0 {
                let p = (point.x - 1, point.y);
                heap.push(WPoint::new(point.weight + cave[p.1][p.0], p.0, p.1));
            }
            if point.x < cave[point.y].len() - 1 {
                let p = (point.x + 1, point.y);
                heap.push(WPoint::new(point.weight + cave[p.1][p.0], p.0, p.1));
            }
            if point.y > 0 {
                let p = (point.x, point.y - 1);
                heap.push(WPoint::new(point.weight + cave[p.1][p.0], p.0, p.1));
            }
            if point.y < cave.len() - 1 {
                let p = (point.x, point.y + 1);
                heap.push(WPoint::new(point.weight + cave[p.1][p.0], p.0, p.1));
            }

        }
    }
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

    let res = run2(&input);
    println!("{res}");
}

#[test]
fn ordering() {
    let mut heap = BinaryHeap::from([WPoint::new(9,0,0), WPoint::new(10,1,1), WPoint::new(0, 3, 3), WPoint::new(5, 1, 3)]);
    heap.push(WPoint::new(6,2,1));
    assert_eq!(heap.pop(), Some(WPoint::new(0,3,3)));
    assert_eq!(heap.pop(), Some(WPoint::new(5,1,3)));
    assert_eq!(heap.pop(), Some(WPoint::new(6,2,1)));
    assert_eq!(heap.pop(), Some(WPoint::new(9,0,0)));
    assert_eq!(heap.pop(), Some(WPoint::new(10,1,1)));
    assert_eq!(heap.pop(), None);
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,40);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,562);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,315);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,2874);
}
