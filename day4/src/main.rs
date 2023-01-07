use std::{env,fs,process};

#[derive(Debug)]
enum BCell {
    Marked,
    UMarked(u32),
}

impl BCell {
    fn is_marked(&self) -> bool {
        match self {
            BCell::Marked => true,
            _ => false,
        }
    }
}

fn mark(board: &mut Vec<Vec<BCell>>, number: u32) -> bool {
    use BCell::*;
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            match board[i][j] {
                UMarked(cell) => {
                    if cell == number {
                        board[i][j] = Marked;
                        // Check if we completed a row/column
                        let mut checked = true;
                        let mut k = 0;
                        while checked && k < board[i].len() {
                            checked &= board[i][k].is_marked();
                            k += 1;
                        }
                        if checked {
                            return true;
                        }
                        checked = true;
                        k = 0;
                        while checked && k < board.len() {
                            checked &= board[k][j].is_marked();
                            k += 1;
                        }
                        return checked;
                    }
                },
                _ => {},
            }
        }
    }
    false
}

fn add_unmarked(board: &Vec<Vec<BCell>>) -> u32 {
    let mut sum = 0;
    for row in board {
        for cell in row {
            match cell {
                BCell::UMarked(n) => sum += n,
                _ => {},
            }
        }
    }
    sum
}

fn run1(input: &str) -> u32 {
    // Parse input
    let mut guesses = vec![];
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    for sn in first.split(',') {
        guesses.push(sn.parse::<u32>().unwrap());
    }
    let mut blocks = input.split("\n\n");
    // Skip already parsed first line
    blocks.next();
    let mut boards = vec![];
    while let Some(sb) = blocks.next() {
        let mut board = vec![];
        for srow in sb.lines() {
            let mut row = vec![];
            for scell in srow.split_whitespace() {
                row.push(BCell::UMarked(scell.parse::<u32>().unwrap()));
            }
            board.push(row);
        }
        boards.push(board);
    }

    // Check numbers in order in all the boards
    for guess in guesses {
        for mut board in boards.iter_mut() {
            if mark(&mut board, guess) {
                //print_board(&board);
                // This board is the winner! Compute its score
                let sum = add_unmarked(&board);
                return sum * guess;
            }
        }
    }
    0
}

fn print_board(board: &Vec<Vec<BCell>>) {
    for row in board {
        let mut string = String::from("");
        for cell in row {
            string = format!("{string} {cell:?}");
        }
        println!("{string}");
    }
}

fn run2(input: &str) -> u32 {
    // Parse input
    let mut guesses = vec![];
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    for sn in first.split(',') {
        guesses.push(sn.parse::<u32>().unwrap());
    }
    let mut blocks = input.split("\n\n");
    // Skip already parsed first line
    blocks.next();
    let mut boards = vec![];
    while let Some(sb) = blocks.next() {
        let mut board = vec![];
        for srow in sb.lines() {
            let mut row = vec![];
            for scell in srow.split_whitespace() {
                row.push(BCell::UMarked(scell.parse::<u32>().unwrap()));
            }
            board.push(row);
        }
        boards.push((true,board));
    }

    // Check numbers in order in all the boards
    let mut remaining = boards.len();
    for guess in guesses {
        for mut pair in boards.iter_mut() {
            if pair.0 && mark(&mut pair.1, guess) {
                //print_board(&pair.1);
                // This board won! Update its information
                pair.0 = false;
                remaining -= 1;
                if remaining == 0 {
                    // This is the last board to win
                    let sum = add_unmarked(&pair.1);
                    return sum * guess;
                }
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
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,4512);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,35670);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,1924);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,22704);
}
