use std::{env,fs,process};

#[derive(Debug,Clone,Copy,PartialEq)]
enum Branch {
    Left,
    Right,
}

impl From<&Branch> for u32 {
    fn from(branch: &Branch) -> u32 {
        match branch {
            Branch::Left => 3,
            Branch::Right => 2,
        }
    }
}

type SnailNum = Vec<(Vec<Branch>, u32)>;

fn read_line(line: &str) -> Result<SnailNum, &'static str> {
    let mut result = Vec::new();
    let mut current_node = Vec::new();

    for c in line.chars() {
        match c {
            '[' => {
                current_node.push(Branch::Left);
            },
            ',' => {
                let l = current_node.len();
                if l < 1 {
                    return Err("Malformed string: ',' out of place");
                }
                current_node[l - 1] = Branch::Right;
            },
            ']' => {
                if current_node.pop().is_none() {
                    return Err("Malformed string: closing ']' out of place");
                }
            },
            _ => {
                if !c.is_whitespace() {
                    if let Some(n) = c.to_digit(10) {
                        let node = current_node.clone();
                        result.push((node, n));
                    } else {
                        return Err("Unexpected character '{c}'");
                    }
                }
            },
        }
    }

    Ok(result)
}

fn magnitude(snail: &SnailNum) -> u32 {
    let mut res = 0;
    for (node, val) in snail {
        let mut temp = *val;
        for n in node {
            temp *= u32::from(n);
        }
        res += temp;
    }
    res
}

fn concatenate(first: &SnailNum, second: &SnailNum) -> SnailNum {
    let mut res = Vec::new();
    for (node, v) in first {
        let mut p = vec![Branch::Left];
        p.extend_from_slice(&node);
        res.push((p, *v));
    }
    for (node, v) in second {
        let mut p = vec![Branch::Right];
        p.extend_from_slice(&node);
        res.push((p,*v));
    }
    res
}

fn explode_once(snail: &mut SnailNum) -> bool {
    let mut index_to_explode = None;
    let mut last_depth = 0;
    let mut i = 0;
    for (node, _) in snail.iter() {
        if last_depth > 4 && node.len() == last_depth {
            // We have located a pair in a depth greater or equal to 4: explode the pair located at
            // the indices (i-1, i)
            index_to_explode = Some(i-1);
            //println!("Explode at {i}");
            break;
        }
        last_depth = node.len();
        i += 1;
    }

    if let Some(i) = index_to_explode {
        // Remove element on the left
        let (mut node, lval) = snail.remove(i);
        // If there is an index to the left, add the value accordingly
        if i > 0 {
            snail[i - 1].1 += lval;
        }
        // Remove element on the right
        let (_, rval) = snail.remove(i);
        // If there is an index to the right, add the value accordingly
        if i < snail.len() {
            snail[i].1 += rval;
        }
        // Remove last branch in the position node (the pair is replaced by a single value)
        node.pop();
        // Insert the value 0
        snail.insert(i, (node, 0));
        true
    } else {
        false
    }
}

fn split_once(snail: &mut SnailNum) -> bool {
    let mut index_to_split = None;
    let mut i = 0;
    for (_, val) in snail.iter() {
        if *val > 9 {
            // We have located a value to split at index i
            index_to_split = Some(i);
            //println!("Split at {i}");
            break;
        }
        i += 1;
    }

    if let Some(i) = index_to_split {
        let (node, val) = &snail[i];
        let lval = val / 2;
        let mut lnode = node.clone();
        lnode.push(Branch::Left);
        let rval = lval + (val % 2);
        let mut rnode = node.clone();
        rnode.push(Branch::Right);
        let temp = [(lnode, lval), (rnode, rval)];
        snail.splice(i..=i, temp);
        true
    } else {
        false
    }
}

fn reduce(snail: &mut SnailNum) {
    let mut changed = true;
    while changed {
        changed = false;
        while explode_once(snail) {
            //println!("{}", to_string(snail));
            changed = true;
        }
        changed |= split_once(snail);
    }
}

fn add(first: &SnailNum, second: &SnailNum) -> SnailNum {
    let mut res = concatenate(first, second);
    reduce(&mut res);
    res
}

fn to_string(snail: &SnailNum) -> String {
    let mut res = String::new();
    res.push('[');
    let mut i = 0;
    let mut lsnail = Vec::new();
    while i < snail.len() && snail[i].0[0] == Branch::Left {
        let mut node = Vec::new();
        for j in 1..snail[i].0.len() {
            node.push(snail[i].0[j]);
        }
        lsnail.push((node, snail[i].1));
        i += 1;
    }
    if lsnail.len() == 1 {
        res.push_str(&lsnail[0].1.to_string());
    } else {
        res.push_str(&to_string(&lsnail));
    }
    res.push(',');
    let mut rsnail = Vec::new();
    while i < snail.len() && snail[i].0[0] == Branch::Right {
        let mut node = Vec::new();
        for j in 1..snail[i].0.len() {
            node.push(snail[i].0[j]);
        }
        rsnail.push((node, snail[i].1));
        i += 1;
    }
    if rsnail.len() == 1 {
        res.push_str(&rsnail[0].1.to_string());
    } else {
        res.push_str(&to_string(&rsnail));
    }
    res.push(']');

    res
}

fn run1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut snail = if let Some(line) = lines.next() {
        read_line(line).expect("Error?")
    } else {
        panic!("Empty file!")
    };
    while let Some(line) = lines.next() {
        let snail2 = read_line(line).expect("Error!");
        snail = add(&snail, &snail2);
        //println!("{}", to_string(&snail));
    }
    magnitude(&snail)
}

fn run2(input: &str) -> u32 {
    let mut snails = Vec::new();
    for line in input.lines() {
        snails.push(read_line(line).expect("Error?"));
    }
    let mut max = 0;
    for i in 0..snails.len() {
        for j in 0..snails.len() {
            let snail = add(&snails[i], &snails[j]);
            let m = magnitude(&snail);
            if m > max {
                max = m;
            }
        }
    }
    max
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
    assert_eq!(res,4140);
}

#[test]
fn magnitude_test() {
    let snail1 = read_line("[[1,2],[[3,4],5]]").expect("Parsing error");
    assert_eq!(magnitude(&snail1), 143);
    let snail2 = read_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").expect("Parsing error");
    assert_eq!(magnitude(&snail2), 1384);
    let snail3 = read_line("[[[[1,1],[2,2]],[3,3]],[4,4]]").expect("Parsing error");
    assert_eq!(magnitude(&snail3), 445);
    let snail4 = read_line("[[[[3,0],[5,3]],[4,4]],[5,5]]").expect("Parsing error");
    assert_eq!(magnitude(&snail4), 791);
    let snail5 = read_line("[[[[5,0],[7,4]],[5,5]],[6,6]]").expect("Parsing error");
    assert_eq!(magnitude(&snail5), 1137);
    let snail6 = read_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").expect("Parsing error");
    assert_eq!(magnitude(&snail6), 3488);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,3734);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,3993);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,4837);
}
