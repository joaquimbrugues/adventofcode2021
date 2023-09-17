use std::{env,fs,process};
use std::collections::HashSet;

#[derive(Clone,Copy,Debug)]
enum Axis { X, Y, Z, }

impl Axis {
    fn complement(&self) -> [Self;2] {
        match self {
            Self::X => [Self::Y, Self::Z],
            Self::Y => [Self::Z, Self::X],
            Self::Z => [Self::X, Self::Y],
        }
    }

    fn axis() -> [Self;3] {
        [Self::X, Self::Y, Self::Z]
    }

    fn cross_product(&self, other: &Self) -> (Self, i32) {
        match self {
            Self::X => {
                match other {
                    Self::X => panic!("Invalid product!"),
                    Self::Y => (Self::Z, 1),
                    Self::Z => (Self::Y, -1),
                }
            },
            Self::Y => {
                match other {
                    Self::X => (Self::Z, -1),
                    Self::Y => panic!("Invalid product!"),
                    Self::Z => (Self::X, 1),
                }
            },
            Self::Z => {
                match other {
                    Self::X => (Self::Y, 1),
                    Self::Y => (Self::X, -1),
                    Self::Z => panic!("Invalid product!"),
                }
            },
        }
    }
}

impl From<Axis> for usize {
    fn from(axis: Axis) -> Self {
        match axis {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
        }
    }
}

#[derive(Debug)]
struct Rotation {
    inner: [(Axis, i32);3],
}

impl Rotation {
    fn rotations() -> Vec<Self> {
        let mut res = Vec::new();
        for x in Axis::axis() {
            for i in 0..2 {
                for y in x.complement() {
                    for j in 0..2 {
                        let az = x.cross_product(&y);
                        let sx = 1 - 2*i;
                        let sy = 1 - 2*j;
                        res.push(Rotation { inner: [(x,sx), (y,sy), (az.0, sx*sy*az.1)] });
                    }
                }
            }
        }

        res
    }

    fn rotate(&self, point: &[i32;3]) -> [i32;3] {
        let r = self.inner;
        [
            r[0].1 * point[ Into::<usize>::into(r[0].0) ],
            r[1].1 * point[ Into::<usize>::into(r[1].0) ],
            r[2].1 * point[ Into::<usize>::into(r[2].0) ],
        ]
    }
}

fn add(left: &[i32;3], right: &[i32;3]) -> [i32;3] {
    [ left[0] + right[0], left[1] + right[1], left[2] + right[2] ]
}

fn diff(left: [i32;3], right: [i32;3]) -> [i32;3] {
    [ left[0] - right[0], left[1] - right[1], left[2] - right[2] ]
}

// Try to fit all beacons associated to scanner (after rotation and translation) to coincide with
// beacons inside of known
// If 12 or more beacons coincide, return true
fn try_fit(known: &HashSet<[i32; 3]>, scanner: &Vec<[i32; 3]>, rotation: &Rotation, translation: &[i32; 3]) -> bool {
    let mut counter = 0;
    for b in scanner {
        let c = add(&rotation.rotate(b), translation);
        if known.contains(&c) {
            counter += 1;
            if counter >= 12 {
                return true;
            }
        }
    }
    false
}

// Apply the rotation and the translation to all beacons in scanner, and add them to known
fn merge(known: &mut HashSet<[i32; 3]>, scanner: &Vec<[i32; 3]>, rotation: &Rotation, translation: &[i32; 3]) {
    for b in scanner {
        let c = add(&rotation.rotate(b), translation);
        known.insert(c);
    }
}

// List all possible rotations of the scanner coordinates
// For each of them, list all possible pairs of beacons between known and scanner (rotated).
// Compute the translation between both, and call the method to try the merge
// known should be modified (by addition) if and only if this function returns the translation
fn try_merge(known: &mut HashSet<[i32; 3]>, scanner: &Vec<[i32; 3]>) -> Option<[i32;3]> {
    let mut trans = None;
    for r in Rotation::rotations() {
        for b1 in known.iter() {
            for b2 in scanner.iter() {
                let translation = diff(*b1, r.rotate(b2));
                if try_fit(known, scanner, &r, &translation) {
                    trans = Some(translation);
                }
            }
        }
        if let Some(translation) = trans {
            merge(known, scanner, &r, &translation);
            return trans;
        }
    }
    trans
}

fn read_input(input: &str) -> Vec<Vec<[i32;3]>> {
    let mut scanners = Vec::new();
    for block in input.split("\n\n") {
        let mut beacons = Vec::new();
        if let Some((_, block)) = block.split_once('\n') {
            for line in block.lines() {
                let strings: Vec<&str> = line.split(',').collect();
                let beacon = [strings[0].parse::<i32>().unwrap(), strings[1].parse::<i32>().unwrap(), strings[2].parse::<i32>().unwrap()];
                beacons.push(beacon);
            }
        }
        scanners.push(beacons);
    }
    scanners
}

fn run1(input: &str) -> usize {
    // Read input
    let mut scanners = read_input(input);
    // Take last scanner as absolute coordinates
    let mut abs_coordinates = scanners.pop().unwrap()
    // Transform from Vec into HashSet
        .into_iter().collect();

    // Find pairs of compatible scanners
    while scanners.len() > 0 {
        for i in (0..scanners.len()).rev() {
            if try_merge(&mut abs_coordinates, &scanners[i]).is_some() {
                scanners.remove(i);
            }
        }
    }
    abs_coordinates.len()
}

fn run2(input: &str) -> i32 {
    // Read input
    let mut scanners = read_input(input);
    // Take last scanner as absolute coordinates
    let mut abs_coordinates = scanners.pop().unwrap()
    // Transform from Vec into HashSet
        .into_iter().collect();
    let mut centers = vec![[0;3]];
    let mut max_distance = 0;

    // Find pairs of compatible scanners
    while scanners.len() > 0 {
        for i in (0..scanners.len()).rev() {
            if let Some(center) = try_merge(&mut abs_coordinates, &scanners[i]) {
                scanners.remove(i);
                for c in centers.iter() {
                    let d = diff(*c, center);
                    let m = d[0].abs() + d[1].abs() + d[2].abs();
                    if m > max_distance {
                        max_distance = m;
                    }
                }
                centers.push(center);
            }
        }
    }
    max_distance
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
    assert_eq!(res, 79);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,376);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,3621);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,10772);
}
