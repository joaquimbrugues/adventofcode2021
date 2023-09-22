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

const ROLLS: [(u32,u128); 7] = [(3,1), (4,3), (5,6), (6,7), (7,6), (8,3), (9,1)];
const WIN_SCORE: u32 = 21;

// Return HashMap with keys: turns and values: HashMap with keys: score, and values: universes
fn compute_universes(initial_position: u32) -> HashMap<u32, HashMap<u32, u128>> {
    let mut result = HashMap::new();
    let mut state = HashMap::from([((initial_position, 0), 1)]);
    let mut turn = 1;
    let mut all_win = false;
    while !all_win {
        all_win = true;
        let mut rest = HashMap::new();
        let mut next_state = HashMap::new();

        for ((pos, score), universes) in state {
            if score < WIN_SCORE {
                for (x, u) in ROLLS {
                    let new_pos = ((pos - 1 + x) % 10) + 1;
                    let new_score = score + new_pos;
                    all_win &= new_score >= WIN_SCORE;
                    let new_universes = u*universes;
                    if let Some(u2) = next_state.get_mut(&(new_pos, new_score)) {
                        *u2 += new_universes;
                    } else {
                        next_state.insert((new_pos, new_score), new_universes);
                    }
                    if let Some(u2) = rest.get_mut(&new_score) {
                        *u2 += new_universes;
                    } else {
                        rest.insert(new_score, new_universes);
                    }
                }
            }
        }
        result.insert(turn, rest);
        state = next_state;
        turn += 1;
    }

    result
}

fn run2(input: &str) -> u128 {
    let (s1, s2) = input.split_once('\n').unwrap();
    let pos1 = s1.trim().rsplit_once(' ').unwrap().1.parse().unwrap();
    let pos2 = s2.trim().rsplit_once(' ').unwrap().1.parse().unwrap();

    let states1 = compute_universes(pos1);
    let states2 = compute_universes(pos2);

    let mut wins1 = 0;
    for (turn, st1) in states1.iter() {
        if let Some(st2) = states2.get(&(turn - 1)) {
            for (&score1, u1) in st1 {
                if score1 >= WIN_SCORE {
                    for (&score2, u2) in st2 {
                        if score2 < WIN_SCORE {
                            wins1 += u1 * u2;
                        }
                    }
                }
            }
        }
    }

    let mut wins2 = 0;
    for (turn, st2) in states2 {
        if let Some(st1) = states1.get(&turn) {
            for (score2, u2) in st2 {
                if score2 >= WIN_SCORE {
                    for (&score1, u1) in st1 {
                        if score1 < WIN_SCORE {
                            wins2 += u1 * u2;
                        }
                    }
                }
            }
        }
    }

    if wins1 > wins2 {
        wins1
    } else {
        wins2
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

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 309196008717909);
}
