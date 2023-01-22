use std::{env,fs,process};
use std::str::Chars;
use std::collections::HashMap;

const TABLE: [(char,u8); 16] = [('0', 0), ('1', 1),('2', 2),('3', 3),('4', 4),('5', 5),('6', 6),('7', 7),('8', 8),('9', 9),('A', 10),('B', 11),('C', 12),('D', 13),('E', 14),('F', 15)];

struct Reader<'a> {
    inner: Chars<'a>,
    current: Option<[bool;4]>,
    index: usize,
}

impl<'a> Reader<'a> {
    fn new(string: &'a str) -> Self {
        Self { inner: string.chars(), current: None, index: 0 }
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() || self.index >= 4 {
            match self.inner.next() {
                Some(c) => {
                    let map = HashMap::from(TABLE);
                    match map.get(&c) {
                        Some(n) => {
                            let mut array = [false; 4];
                            let mut num = *n;
                            for i in 0..4 {
                                array[3 - i] = num % 2 == 1;
                                num /= 2;
                            }
                            self.current = Some(array);
                            self.index = 0;
                        },
                        None => panic!("Unexpected character {c}"),
                    }
                },
                None => {
                    return None;
                }
            }
        }
        let next = self.current.expect("Current array should be non-empty!")[self.index];
        self.index += 1;
        Some(next)
    }
}

#[derive(Debug)]
enum Packet {
    Literal(u16, u32), // version, literal value
    Operator(u16, u16, Vec<Packet>), // version, type, list of subpackets
}

impl Packet {
    fn read_bits(reader: &mut Reader, length: usize, counter_stack: &mut Vec<u32>) -> u16 {
        let mut num = 0;
        for _ in 0..length {
            num *= 2;
            if reader.next().unwrap() {
                num += 1;
            }
        }
        for counter in counter_stack.iter_mut() {
            *counter += length as u32;
        }
        num
    }

    fn read(reader: &mut Reader, counter_stack: &mut Vec<u32>) -> Self {
        let version = Self::read_bits(reader, 3, counter_stack);
        println!("{version}");
        let type_id = Self::read_bits(reader, 3, counter_stack);
        if type_id == 4 {
            // Literal packet
            let mut value = 0;
            let mut keep_going = true;
            while keep_going {
                value *= 16;
                keep_going = reader.next().unwrap();
                for counter in counter_stack.iter_mut() {
                    *counter += 1;
                }
                let byte = Self::read_bits(reader, 4, counter_stack);
                value += byte as u32;
            }
            Packet::Literal(version,value)
        } else {
            // Operator
            let mut subpackets = vec![];
            // Read length type ID
            if reader.next().unwrap() {
                // Length type 1 -> next 11 bits are a number that represents the number of
                // sub-packets immediately contained in this packet
                let length = Self::read_bits(reader,11, counter_stack);
                for _ in 0..length {
                    let packet = Self::read(reader, &mut vec![]);
                    subpackets.push(packet);
                }
            } else {
                // Length type 0 -> next 15 bits are a number that represents the length in bits of
                // the sub-packets contained in this packet
                let length = Self::read_bits(reader,15, counter_stack) as u32;
                counter_stack.push(length);
                while *counter_stack.last().unwrap() < length {
                    let packet = Self::read(reader, counter_stack);
                    subpackets.push(packet);
                }
                counter_stack.pop();
            }
            Packet::Operator(version, type_id, subpackets)
        }
    }

    fn get_sum_versions(&self) -> u32 {
        match self {
            Packet::Literal(version, _) => *version as u32,
            Packet::Operator(version, _, vector) => {
                let mut sum = *version as u32;
                for p in vector {
                    sum += p.get_sum_versions();
                }
                sum
            },
        }
    }
}

fn run1(input: &str) -> u32 {
    let mut reader = Reader::new(input.trim());
    let packet = Packet::read(&mut reader, &mut vec![]);
    packet.get_sum_versions()
}

fn run2(input: &str) -> u32 {
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
    for line in input.lines() {
        let res = run1(line);
        println!("{res}");
    }
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    assert_eq!(run1(lines[0]),16);
    assert_eq!(run1(lines[1]),12);
    assert_eq!(run1(lines[2]),23);
    assert_eq!(run1(lines[3]),31);
}

//#[test]
//fn input1() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run1(&input);
    //assert_eq!(res,42);
//}

//#[test]
//fn example2() {
    //let input = fs::read_to_string("test.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
