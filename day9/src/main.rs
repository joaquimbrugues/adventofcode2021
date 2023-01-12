use std::{env,fs,process};

fn run1(input: &str) -> u32 {
    let mut sum = 0;
    let mut heights = vec![];
    // Read input
    for line in input.lines() {
        let mut row: Vec<u32> = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        heights.push(row);
    }

    // Find low points
    for i in 0..heights.len() {
        for j in 0..heights[i].len() {
            let mut low_point = true;
            // Check up
            low_point &= i == 0 || heights[i][j] < heights[i-1][j];
            // Check left
            low_point &= j == 0 || heights[i][j] < heights[i][j-1];
            // Check down
            low_point &= i == heights.len() - 1 || heights[i][j] < heights[i+1][j];
            // Check right
            low_point &= j == heights[i].len() - 1 || heights[i][j] < heights[i][j+1];
            if low_point {
                sum += heights[i][j] + 1;
            }
        }
    }
    sum
}

// Decreasing order
fn insert_ordered(vector: &mut Vec<u32>, n: u32) {
    let mut i = 0;
    while i < 3 {
        if i >= vector.len() || vector[i] < n {
            vector.insert(i,n);
            break;
        }
        i += 1;
    }
    
    if vector.len() > 3 {
        vector.pop();
    }
}

fn run2(input: &str) -> u32 {
    let mut map = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            let n = c.to_digit(10).unwrap();
            row.push(n < 9);
        }
        map.push(row);
    }


    let mut i = 0;
    let mut sizes = vec![];
    while i < map.len() {
        let mut j = 0;
        while j < map[i].len() {
            if map[i][j] {
                // We found an unexplored basin. Let's explore it!
                map[i][j] = false;
                let mut size = 0;
                let mut stack = vec![(i,j)];
                while let Some((k,l)) = stack.pop() {
                    size += 1;
                    if k > 0 && map[k-1][l] {
                        map[k-1][l] = false;
                        stack.push((k-1,l));
                    }
                    if l > 0 && map[k][l-1] {
                        map[k][l-1] = false;
                        stack.push((k,l-1));
                    }
                    if k < map.len() - 1 && map[k+1][l] {
                        map[k+1][l] = false;
                        stack.push((k+1,l));
                    }
                    if l < map[k].len() - 1 && map[k][l+1] {
                        map[k][l+1] = false;
                        stack.push((k,l+1));
                    }
                }
                insert_ordered(&mut sizes, size);
            }
            j += 1;
        }
        i += 1;
    }
    sizes[0] * sizes[1] * sizes[2]
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
    assert_eq!(res,15);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,591);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,1134);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,1113424);
}
