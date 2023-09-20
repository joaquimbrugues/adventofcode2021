use std::{env,fs,process};
use std::collections::HashMap;

//trait Die {
    //fn roll(&mut self) -> u64
//}

struct DeterministicDie {
    counter: u64,
    face: u64,
}

impl DeterministicDie {
    fn new() -> Self {
        Self { counter: 0, face: 0 }
    }

    fn roll(&mut self) -> u64 {
        self.face %= 100;
        self.counter += 1;
        self.face += 1;
        self.face
    }
}

fn play(players: &mut [(u64, u64); 2], die: &mut DeterministicDie, i: usize) -> Option<u64> {
    // First, roll three times
    let mut shift = 0;
    for _ in 0..3 {
        shift += die.roll();
    }

    // Displace the position of the current player
    players[i].0 = ((players[i].0 + shift - 1) % 10) + 1;

    // Increase the player's score
    players[i].1 += players[i].0;

    // Check if the game is finished
    if players[i].1 >= 1000 {
        Some(players[1 - i].1 * die.counter)
    } else {
        None
    }
}

fn run1(input: &str) -> u64 {
    let (s1, s2) = input.split_once('\n').unwrap();
    let s1 = s1.rsplit_once(' ').unwrap().1;
    let pos1 = s1.parse().unwrap();
    let s2 = s2.trim().rsplit_once(' ').unwrap().1;
    let pos2 = s2.parse().unwrap();
    let mut players = [(pos1, 0), (pos2, 0)];
    let mut i = 0;
    let mut die = DeterministicDie::new();
    loop {
        i %= 2;
        if let Some(res) = play(&mut players, &mut die, i) {
            return res;
        }
        i += 1;
    }
}

fn run2(input: &str) -> u128 {
    let (s1, s2) = input.split_once('\n').unwrap();
    let s1 = s1.rsplit_once(' ').unwrap().1;
    let pos1: u32 = s1.parse().unwrap();
    let s2 = s2.trim().rsplit_once(' ').unwrap().1;
    let pos2: u32 = s2.parse().unwrap();
    // Status tuple: ( [(position, score); 2], current_player ) -> number_of_universes
    let mut stack = Vec::new();
    let mut winners = (0,0);
    stack.push(([(pos1, 0), (pos2, 0)], 0));
    // TODO: Trim this algorithm to trim repeated branches
    // Depth-first search
    while let Some((players, current)) = stack.pop() {
        if players[current].1 > 20 {
            if current % 2 == 0 {
                winners.0 += 1;
            } else {
                winners.1 += 1;
            }
        } else {
            for i in 1..=3 {
                for j in 1..=3 {
                    for k in 1..=3 {
                        let pos = ((players[current].0 + i + j + k - 1) % 10) + 1;
                        let score = players[current].1 + pos;
                        let mut new_players = [(0,0); 2];
                        new_players[1 - current] = players[1 - current];
                        new_players[current] = (pos, score);
                        stack.push((new_players, 1 - current));
                    }
                }
            }
        }
    }

    if winners.0 > winners.1 {
        winners.0
    } else {
        winners.1
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
    assert_eq!(res, 739785);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 752745);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 444356092776315);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
