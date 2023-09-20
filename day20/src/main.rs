use std::{env,fs,process};
use std::collections::HashMap;

fn read_enhancement(input: &str) -> Vec<bool> {
    let mut enh = Vec::new();
    for c in input.chars() {
        match c {
            '#' => enh.push(true),
            '.' => enh.push(false),
            '\n' => {},
            _ => {
                panic!("Unexpected character {c}!");
            }
        }
    }

    enh
}

fn read_image(input: &str) -> HashMap<(isize, isize), bool> {
    let mut i = 0;
    let mut j = 0;
    let mut image = HashMap::new();
    for c in input.chars() {
        match c {
            '#' => {
                image.insert((i,j), true);
                i += 1;
            },
            '.' => {
                image.insert((i,j), false);
                i +=1;
            },
            '\n' => {
                i = 0;
                j += 1;
            },
            _ => {
                panic!("Unexpected character {c}!");
            }
        }
    }

    image
}

fn square(point: (isize, isize)) -> [(isize, isize); 9] {
    [
        (point.0 - 1, point.1 - 1),
        (point.0, point.1 - 1),
        (point.0 + 1, point.1 - 1),
        (point.0 - 1, point.1),
        point,
        (point.0 + 1, point.1),
        (point.0 - 1, point.1 + 1),
        (point.0, point.1 + 1),
        (point.0 + 1, point.1 + 1),
    ]
}

struct Rectangle {
    minx: isize,
    maxx: isize,
    maxy: isize,
    curx: isize,
    cury: isize,
}

impl Rectangle {
    fn from_image(image: &HashMap<(isize, isize), bool>) -> Self {
        let mut minx = isize::MAX;
        let mut miny = isize::MAX;
        let mut maxx = isize::MIN;
        let mut maxy = isize::MIN;

        for &(px, py) in image.keys() {
            if px < minx {
                minx = px;
            }
            if py < miny {
                miny = py;
            }
            if px > maxx {
                maxx = px;
            }
            if py > maxy {
                maxy = py;
            }
        }
        minx -= 1;
        miny -= 1;
        maxx += 1;
        maxy += 1;

        Self{ minx, /*miny,*/ maxx, maxy, curx: minx - 1, cury: miny, }
        //Self{ minx: minx - 1, [>miny,<] maxx: maxx + 1, maxy: maxy + 1, curx: minx - 3, cury: miny - 2, }
    }
}

impl Iterator for Rectangle {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        self.curx += 1;
        if self.curx > self.maxx {
            self.curx = self.minx;
            self.cury += 1;
        }

        if self.cury > self.maxy {
            None
        } else {
            Some((self.curx, self.cury))
        }
    }
}

fn enhance(image: &HashMap<(isize, isize), bool>, enh: &Vec<bool>, default: bool) -> HashMap<(isize, isize), bool> {
    let mut new_image = HashMap::new();
    for pixel in Rectangle::from_image(image) {
        let square = square(pixel);
        let mut index = 0;
        for px in square {
            index *= 2;
            if let Some(&b) = image.get(&px) {
                if b {
                    index += 1;
                }
            } else if default {
                index += 1;
            }
        }
        assert!(index < 512);
        new_image.insert(pixel, enh[index]);
    }

    new_image
}

fn print_image(image: &HashMap<(isize, isize), bool>, default: bool) {
    let mut line = String::new();
    let mut last_j = isize::MAX;
    for (i, j) in Rectangle::from_image(image) {
        if j > last_j {
            println!("{line}");
            line = String::new();
        }
        if let Some(b) = image.get(&(i,j)) {
            match b {
                true => line.push('#'),
                false => line.push('.'),
            }
        } else {
            match default {
                true => line.push('#'),
                false => line.push('.'),
            }
        }
        last_j = j;
    }
    println!("{line}");
}

fn run1(input: &str) -> usize {
    // Split the two halves of the input
    let (s1, s2) = input.split_once("\n\n").unwrap();

    // Read the image enhancement algorithm part
    let enhancement = read_enhancement(s1);
    // Sanity check
    assert_eq!(enhancement.len(), 512);

    // Read the image
    let mut image = read_image(s2);

    // Enhance the image
    image = enhance(&image, &enhancement, false);
    //print_image(&image, enhancement[0]);
    // Enhance again
    image = enhance(&image, &enhancement, enhancement[0]);

    image.values().filter(|&&b| b).collect::<Vec<_>>().len()
}

fn run2(input: &str) -> usize {
    // Split the two halves of the input
    let (s1, s2) = input.split_once("\n\n").unwrap();

    // Read the image enhancement algorithm part
    let enhancement = read_enhancement(s1);
    // Sanity check
    assert_eq!(enhancement.len(), 512);

    // Read the image
    let mut image = read_image(s2);

    // Successive enhancements
    for i in 0..50 {
        let default = if !enhancement[0] {
            false
        } else if i == 0 {
            false
        } else if i % 2 == 0 {
            *enhancement.last().unwrap()
        } else {
            enhancement[0]
        };
        image = enhance(&image, &enhancement, default);
    }

    image.values().filter(|&&b| b).collect::<Vec<_>>().len()
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
    assert_eq!(res, 35);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 5619);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 3351);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 20122);
}
