use std::{env,fs,process};
use std::collections::HashSet;

// Idea of the algorithm:
// First, read all the entries and do TWO operations:
// 1 - Store the separators of the 3D "grid" into three ordered vectors
// 2 - Store the cuboids with on (true) or off (false) in a Vec
// Then, resolve each cube of the resulting grid and store it into a HashSet<(usize, usize, usize)>
// Finally, compute the volume of the resulting combination

fn insert_ord(vec: &mut Vec<i128>, num: i128) {
    let mut i = 0;
    while i < vec.len() && vec[i] < num {
        i += 1;
    }
    if i < vec.len() && vec[i] == num {
        return;
    }
    vec.insert(i, num);
}

fn operate_grid(on: &mut HashSet<(usize,usize,usize)>, grid: &[Vec<i128>;3], cuboid: [(i128, i128);3], operation: bool) {
    let mut i = 0;
    while grid[0][i] < cuboid[0].0 { i += 1; }
    let mut j0 = 0;
    while grid[1][j0] < cuboid[1].0 { j0 += 1; }
    let mut k0 = 0;
    while grid[2][k0] < cuboid[2].0 { k0 += 1; }
    while grid[0][i] < cuboid[0].1 {
        let mut j = j0;
        while grid[1][j] < cuboid[1].1 {
            let mut k = k0;
            while grid[2][k] < cuboid[2].1 {
                if operation {
                    // Add the small cuboid marked by (i,j,k) to the 'on' set
                    on.insert((i,j,k));
                } else {
                    // Remove the small cuboid marked by (i,j,k) to the 'on' set
                    on.remove(&(i,j,k));
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}

fn get_volume(on: &HashSet<(usize, usize, usize)>, grid: &[Vec<i128>;3]) -> i128 {
    let mut sum = 0;
    for &(i,j,k) in on {
        sum += (grid[0][i+1] - grid[0][i]).abs() * (grid[1][j+1] - grid[1][j]).abs() * (grid[2][k+1] - grid[2][k]).abs();
    }
    sum
}

fn run(input: &str, second_part: bool) -> i128 {
    let mut grid = [Vec::new(), Vec::new(), Vec::new()];
    let mut cuboids = Vec::new();
    for line in input.lines() {
        let (op, line) = line.split_once(' ').unwrap();
        let mut i = 0;
        let mut cuboid = [(0,0);3];
        for chunk in line.split(',') {
            let (_, chunk) = chunk.split_once('=').unwrap();
            let (s1, s2) = chunk.split_once("..").unwrap();
            let min = s1.parse().unwrap();
            insert_ord(&mut grid[i], min);
            let max = s2.parse::<i128>().unwrap() + 1;
            insert_ord(&mut grid[i], max);
            cuboid[i] = (min, max);

            i += 1;
        }
        let op = match op {
            "on" => true,
            "off" => false,
            _ => panic!("Unexpected operation"),
        };
        cuboids.push((cuboid, op));
    }

    let mut resolved = HashSet::new();
    let mut i = 1;
    let l = cuboids.len();
    for (cuboid, op) in cuboids {
        if second_part
            || ( cuboid[0].0 >= -50 && cuboid[0].1 <= 50
                && cuboid[1].0 >= -50 && cuboid[1].1 <= 50
                && cuboid[2].0 >= -50 && cuboid[2].1 <= 50) {
                // Resolve the cuboid
                println!("Step {i} of {l}");
                operate_grid(&mut resolved, &grid, cuboid, op);
        }
        i += 1;
    }
    //println!("Resolved: {resolved:?}\nGrid: {grid:?}");
    get_volume(&resolved, &grid)
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

    let res = run(&input, true);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test1.txt").unwrap();
    let res = run(&input, false);
    assert_eq!(res, 39);
}

// This test does not work for whatever reason -\_(-:_/-
#[test]
fn example2() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run(&input, false);
    assert_eq!(res, 590784);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input, false);
    assert_eq!(res, 581108);
}

#[test]
fn example3() {
    let input = fs::read_to_string("test3.txt").unwrap();
    let res = run(&input, true);
    assert_eq!(res, 2758514936282235);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input, true);
    assert_eq!(res,1325473814582641);
}
