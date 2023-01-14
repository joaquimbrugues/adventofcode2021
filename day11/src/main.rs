use std::{env,fs,process};
use std::collections::{VecDeque,HashSet};

const GRID_SIZE: usize = 10;

fn neighbours(pos: &(usize,usize)) -> Vec<(usize,usize)> {
    let pos = *pos;
    let mut neighs = vec![];
    if pos.0 > 0 {
        neighs.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        neighs.push((pos.0, pos.1 - 1));
    }
    if pos.0 > 0 && pos.1 > 0 {
        neighs.push((pos.0 - 1, pos.1 - 1));
    }
    if pos.0 > 0 && pos.1 < GRID_SIZE - 1 {
        neighs.push((pos.0 - 1, pos.1 + 1));
    }
    if pos.1 > 0 && pos.0 < GRID_SIZE - 1 {
        neighs.push((pos.0 + 1, pos.1 - 1));
    }
    if pos.0 < GRID_SIZE - 1 {
        neighs.push((pos.0 + 1, pos.1));
    }
    if pos.1 < GRID_SIZE - 1 {
        neighs.push((pos.0, pos.1 + 1));
    }
    if pos.0 < GRID_SIZE - 1 && pos.1 < GRID_SIZE - 1 {
        neighs.push((pos.0 + 1, pos.1 + 1));
    }
    neighs
}

fn run1(input: &str) -> u32 {
    let mut dumbos = [[0;GRID_SIZE];GRID_SIZE];
    let mut i = 0;
    for line in input.lines() {
        let mut j = 0;
        for c in line.chars() {
            dumbos[i][j] = c.to_digit(10).unwrap();
            j += 1;
        }
        i += 1;
    }
    let steps = 100;
    let mut flashes = 0;
    for _ in 0..steps {
        // Initializations
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // Add 1 to all dumbos
        // Add dumbos that will flash to the queue
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                dumbos[i][j] += 1;
                if dumbos[i][j] > 9 {
                    queue.push_back((i,j));
                    visited.insert((i,j));
                }
            }
        }

        // Breadth-first search only through flashes
        while let Some((i,j)) = queue.pop_front() {
            dumbos[i][j] = 0;
            flashes += 1;
            for (k,l) in neighbours(&(i,j)) {
                if !visited.contains(&(k,l)) {
                    dumbos[k][l] += 1;
                    if dumbos[k][l] > 9 {
                        visited.insert((k,l));
                        queue.push_back((k,l));
                    }
                }
            }
        }
    }
    flashes
}

fn run2(input: &str) -> u32 {
    let mut dumbos = [[0;GRID_SIZE];GRID_SIZE];
    let mut i = 0;
    for line in input.lines() {
        let mut j = 0;
        for c in line.chars() {
            dumbos[i][j] = c.to_digit(10).unwrap();
            j += 1;
        }
        i += 1;
    }
    let mut step = 1;
    loop {
        // Initializations
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut flashes = 0;

        // Add 1 to all dumbos
        // Add dumbos that will flash to the queue
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                dumbos[i][j] += 1;
                if dumbos[i][j] > 9 {
                    queue.push_back((i,j));
                    visited.insert((i,j));
                }
            }
        }

        // Breadth-first search only through flashes
        while let Some((i,j)) = queue.pop_front() {
            dumbos[i][j] = 0;
            flashes += 1;
            for (k,l) in neighbours(&(i,j)) {
                if !visited.contains(&(k,l)) {
                    dumbos[k][l] += 1;
                    if dumbos[k][l] > 9 {
                        visited.insert((k,l));
                        queue.push_back((k,l));
                    }
                }
            }
        }

        // Check if we are done
        if flashes == GRID_SIZE * GRID_SIZE {
            return step;
        }
        step += 1;
    }
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
    assert_eq!(res,1656);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,1683);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,195);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,788);
}
