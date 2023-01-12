use std::{env,fs,process};
use std::collections::{HashSet,HashMap};

const DISPLAY: [([bool; 7], u32); 10] = [
([true,true,true,true,true,true,false],0),
([false,true,true,false,false,false,false],1),
([true,true,false,true,true,false,true],2),
([true,true,true,true,false,false,true],3),
([false,true,true,false,false,true,true],4),
([true,false,true,true,false,true,true],5),
([true,false,true,true,true,true,true],6),
([true,true,true,false,false,false,false],7),
([true,true,true,true,true,true,true],8),
([true,true,true,true,false,true,true],9),
];

fn run1(input: &str) -> u32 {
    let mut sum = 0;
    let set = HashSet::from([2,3,4,7]);
    for line in input.lines() {
        let part = line.split(" | ").collect::<Vec<_>>()[1];
        for word in part.trim().split(' ') {
            if set.contains(&word.len()) {
                sum += 1;
            }
        }
    }
    sum
}

fn difference(short: &str, long: &str) -> Vec<char> {
    assert!(short.len() <= long.len());
    let mut set = vec![];
    for c in long.chars() {
        if !short.contains(c) {
            set.push(c);
        }
    }
    set
}

fn intersection(strings: &[&str]) -> Vec<char> {
    if strings.len() == 0 {
        vec![]
    } else {
        let mut vector: Vec<char> = strings[0].chars().collect();
        for i in 1..strings.len() {
            vector.retain(|&c| strings[i].contains(c));
        }
        vector
    }
}

fn run2(input: &str) -> u32 {
    let mut sum = 0;
    let display = HashMap::from(DISPLAY);
    for line in input.lines() {
        if line.starts_with('#') {
            continue;
        }
        // First part: store the input smartly: first take the unique codes ordered by length (1,
        // 7, 4, 8), then all the 5-length codes (2, 3, 5), and lastly the 6-length codes (0, 6, 9)
        let mut unique: Vec<&str> = vec![];
        let mut fives = vec![];
        let mut sixes = vec![];
        let mut parts = line.split('|').map(|s| s.trim());
        for word in parts.next().unwrap().split(' ') {
            match word.len() {
                5 => fives.push(word),
                6 => sixes.push(word),
                n => {
                    let mut i = 0;
                    let mut pushed = false;
                    while !pushed && i < unique.len() {
                        if unique[i].len() > n {
                            unique.insert(i, word);
                            pushed = true;
                        }
                        i += 1;
                    }
                    if !pushed {
                        unique.push(word);
                    }
                },
            }
        }
        // Now, start decoding
        // The result will be an array of chars, which indicates which character activates the
        // corresponding section, where:
        /*
         *  *0*
         * *   *
         * 5   1
         * *   *
         *  *6* 
         * *   *
         * 4   2
         * *   *
         *  *3* 
         */
        let mut code = ['.'; 7];
        // First step: compare 1 and 7 to obtain position 0
        let top = difference(unique[0],unique[1]);
        assert!(top.len() == 1);
        code[0] = top[0];
        // "group 12" is just unique[0]
        // Compare 1 and 4 to get possible characters for 5 and 6
        let group56 = difference(unique[0], unique[2]);
        // Compare 4 and 8 to get possible characters for 3 and 4
        let mut group34 = difference(unique[2], unique[3]);
        // Remove character in position 0
        group34.retain(|&c| c != top[0]);
        // Use fives to get group 0, 6, 3:
        let group063 = intersection(&fives);
        // From now on this should be straightforward
        // Disambiguate group 5-6 with the data that we collected
        for &c in group063.iter() {
            if group56[0] == c {
                code[6] = c;
                code[5] = group56[1];
                break;
            } else if group56[1] == c {
                code[6] = c;
                code[5] = group56[0];
                break;
            }
        }
        // Disambiguate group 3-4
        for &c in group063.iter() {
            if group34[0] == c {
                code[3] = c;
                code[4] = group34[1];
                break;
            } else if group34[1] == c {
                code[3] = c;
                code[4] = group34[0];
                break;
            }
        }
        // Disambiguate group 1-2
        for &num in fives.iter() {
            // Discard "3"
            if !num.contains(unique[0]) {
                if num.contains(code[4]) {
                    // Number "2"
                    let c = intersection(&[unique[0],num]);
                    assert!(c.len() == 1);
                    code[1] = c[0];
                }
                if num.contains(code[5]) {
                    // Number "5"
                    let c = intersection(&[unique[0],num]);
                    assert!(c.len() == 1);
                    code[2] = c[0];
                }
            }
        }

        // We have the complete code!
        // Now it is time to decode the numbers in the second part on the input line
        let mut num = 0;
        for word in parts.next().unwrap().split(' ') {
            num *= 10;
            let mut light = [false; 7];
            for i in 0..7 {
                light[i] = word.contains(code[i]);
            }
            num += display.get(&light).unwrap();
        }
        sum += num;
    }
    sum
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
    assert_eq!(res,26);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,355);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,61229);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,983030);
}
