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
    Literal(u16, u64), // version, literal value
    Operator(u16, u16, bool, Vec<Packet>), // version, type, type_len id, list of subpackets
}

impl Packet {
    fn read_bits(reader: &mut Reader, length: usize, counter: &mut Option<u32>) -> u16 {
        let mut num = 0;
        for _ in 0..length {
            num *= 2;
            if reader.next().unwrap() {
                num += 1;
            }
        }
        match counter.as_mut() {
            Some(inner) => *inner += length as u32,
            None => {},
        }
        num
    }

    fn parse(reader: &mut Reader, counter: &mut Option<u32>) -> Self {
        let version = Self::read_bits(reader, 3, counter);
        //println!("{version}");
        let type_id = Self::read_bits(reader, 3, counter);
        //println!("Version: {version}, Type: {type_id}, {counter:?}");
        if type_id == 4 {
            // Literal packet
            let mut value = 0;
            let mut keep_going = true;
            while keep_going {
                value *= 16;
                keep_going = reader.next().unwrap();
                match counter.as_mut() {
                    Some(inner) => *inner += 1,
                    None => {},
                }
                let byte = Self::read_bits(reader, 4, counter);
                value += byte as u64;
            }
            Packet::Literal(version,value)
        } else {
            // Operator
            let mut subpackets = vec![];
            // Read length type ID
            let type_len_id = reader.next().unwrap();
            // Count the length type ID bit
            match counter.as_mut() {
                Some(inner) => *inner += 1,
                None => {},
            }
            if type_len_id {
                // Length type 1 -> next 11 bits are a number that represents the number of
                // sub-packets immediately contained in this packet
                let length = Self::read_bits(reader,11, counter);
                for _ in 0..length {
                    let packet = Self::parse(reader, counter);
                    subpackets.push(packet);
                }
            } else {
                // Length type 0 -> next 15 bits are a number that represents the length in bits of
                // the sub-packets contained in this packet
                let length = Self::read_bits(reader,15, counter) as u32;
                let mut accumulated_bits = Some(0);
                while accumulated_bits.unwrap() < length {
                    let packet = Self::parse(reader, &mut accumulated_bits);
                    subpackets.push(packet);
                }
                match counter.as_mut() {
                    Some(inner) => *inner += accumulated_bits.unwrap(),
                    None => {},
                }
            }
            Packet::Operator(version, type_id, type_len_id, subpackets)
        }
    }

    fn get_sum_versions(&self) -> u32 {
        match self {
            Packet::Literal(version, _) => *version as u32,
            Packet::Operator(version, _, _, vector) => {
                let mut sum = *version as u32;
                for p in vector {
                    sum += p.get_sum_versions();
                }
                sum
            },
        }
    }

    fn resolve(&self) -> u64 {
        match self {
            Packet::Literal(_, num) => *num,
            Packet::Operator(_, type_id, _, vector) => {
                match *type_id {
                    0 => {
                        // Sum
                        assert!(vector.len() > 0);
                        let mut sum = 0;
                        for p in vector {
                            sum += p.resolve();
                        }
                        sum
                    },
                    1 => {
                        // Product
                        assert!(vector.len() > 0);
                        let mut product = 1;
                        for p in vector {
                            product *= p.resolve();
                        }
                        product
                    },
                    2 => {
                        // Minimum
                        assert!(vector.len() > 0);
                        let mut min = vector[0].resolve();
                        for p in vector[1..].iter() {
                            let res = p.resolve();
                            if min > res {
                                min = res;
                            }
                        }
                        min
                    },
                    3 => {
                        // Maximum
                        assert!(vector.len() > 0);
                        let mut max = vector[0].resolve();
                        for p in vector[1..].iter() {
                            let res = p.resolve();
                            if max < res {
                                max = res;
                            }
                        }
                        max
                    },
                    5 => {
                        // Greater than
                        assert!(vector.len() == 2);
                        if vector[0].resolve() > vector[1].resolve() {
                            1
                        } else {
                            0
                        }
                    },
                    6 => {
                        // Less than
                        assert!(vector.len() == 2);
                        if vector[0].resolve() < vector[1].resolve() {
                            1
                        } else {
                            0
                        }
                    },
                    7 => {
                        // Equals
                        assert!(vector.len() == 2);
                        if vector[0].resolve() == vector[1].resolve() {
                            1
                        } else {
                            0
                        }
                    },
                    _ => panic!("Unimplemented packet type id {type_id}"),
                }
            },
        }
    }

    fn serialize_bits(&self) -> Vec<bool> {
        let mut result = vec![];
        match self {
            Packet::Literal(version, literal) => {
                let mut wversion = *version;
                let mut vversion = vec![];
                for _ in 0..3 {
                    vversion.push(wversion % 2 == 1);
                    wversion /= 2;
                }
                while vversion.len() < 3 {
                    vversion.push(false);
                }
                for v in vversion.into_iter().rev() {
                    result.push(v);
                }
                result.extend_from_slice(&[true, false, false]);

                let mut number = *literal;
                let mut vnumber = vec![];
                while number > 0 {
                    vnumber.push(number % 2 == 1);
                    number /= 2;
                }
                while vnumber.len() == 0 || vnumber.len() % 4 != 0 {
                    vnumber.push(false);
                }
                for i in (0..vnumber.len()).rev() {
                    if i % 4 == 3 {
                        result.push(i != 3);
                    }
                    result.push(vnumber[i]);
                }
            },
            Packet::Operator(version, type_id, type_len_id, subpackets) => {
                let mut wversion = *version;
                let mut vversion = vec![];
                for _ in 0..3 {
                    vversion.push(wversion % 2 == 1);
                    wversion /= 2;
                }
                for v in vversion.into_iter().rev() {
                    result.push(v);
                }

                let mut wtype_id = *type_id;
                let mut vtype_id = vec![];
                for _ in 0..3 {
                    vtype_id.push(wtype_id % 2 == 1);
                    wtype_id /= 2;
                }
                for t in vtype_id.into_iter().rev() {
                    result.push(t);
                }

                result.push(*type_len_id);

                if *type_len_id {
                    // Type_len == 1 -> len represents number of subpackets in 11 bits
                    let mut wlen = subpackets.len();
                    let mut vlen = vec![];
                    while wlen > 0 {
                        vlen.push(wlen % 2 == 1);
                        wlen /= 2;
                    }
                    while vlen.len() < 11 {
                        vlen.push(false);
                    }
                    for l in vlen.into_iter().rev() {
                        result.push(l);
                    }

                    for sp in subpackets {
                        result.extend_from_slice(&sp.serialize_bits());
                    }
                } else {
                    // Type_len == 0 -> len represents the length of the serialization of the
                    // subpackets in 15 bits
                    let mut temp = vec![];
                    for sp in subpackets {
                        temp.extend_from_slice(&sp.serialize_bits());
                    }

                    let mut wlen = temp.len();
                    let mut vlen = vec![];
                    while wlen > 0 {
                        vlen.push(wlen % 2 == 1);
                        wlen /= 2;
                    }
                    while vlen.len() < 15 {
                        vlen.push(false);
                    }
                    for l in vlen.into_iter().rev() {
                        result.push(l);
                    }

                    result.extend_from_slice(&temp);
                }
            },
        }
        result
    }

    fn serialize(&self) -> String {
        let mut bits = self.serialize_bits();
        let bits_as_num = bits.iter().map(|&b| match b {
                true => 1,
                false => 0,
            }).collect::<Vec<u8>>();
        //println!("{bits_as_num:?}");
        let padding_len = (4 - (bits.len() % 4)) % 4;
        for _ in 0..padding_len {
            bits.push(false);
        }
        let mut result = String::from("");
        for iter in (0..bits.len()).step_by(4) {
            let mut group = 0;
            for i in 0..4 {
                group *= 2;
                if bits[iter + i] {
                    group += 1;
                }
            }
            result = format!("{result}{group:X}");
        }
        result
    }
}

fn run1(input: &str) -> u32 {
    let mut reader = Reader::new(input.trim());
    let packet = Packet::parse(&mut reader, &mut None);
    //println!("{packet:?}");
    packet.get_sum_versions()
}

fn run2(input: &str) -> u64 {
    let mut reader = Reader::new(input.trim());
    let packet = Packet::parse(&mut reader, &mut None);
    packet.resolve()
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
    //for line in input.lines() {
        //let res = run1(line);
        //println!("Sum of versions: {res}");
    //}
    for line in input.lines() {
        let res = run2(line);
        println!("Resolution of packet: {res}");
    }
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    assert_eq!(run1(lines[0]),16);
    assert_eq!(run1(lines[1]),12);
    assert_eq!(run1(lines[2]),23);
    assert_eq!(run1(lines[3]),31);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,929);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    assert_eq!(run2(lines[0]),3);
    assert_eq!(run2(lines[1]),54);
    assert_eq!(run2(lines[2]),7);
    assert_eq!(run2(lines[3]),9);
    assert_eq!(run2(lines[4]),1);
    assert_eq!(run2(lines[5]),0);
    assert_eq!(run2(lines[6]),0);
    assert_eq!(run2(lines[7]),1);
    assert_eq!(run2(lines[8]),15);
    assert_eq!(run2(lines[9]),46);
    assert_eq!(run2(lines[10]),46);
    assert_eq!(run2(lines[11]),54);
    assert_eq!(run2(lines[12]),2021);
    assert_eq!(run2(lines[13]),1);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,911945136934);
}
