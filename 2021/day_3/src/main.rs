use std::env;
use std::fs;

struct Binary(String);

trait Convert {
    fn split_line(&mut self) -> Vec<&str>;
}

impl Convert for Binary {
    fn split_line(&mut self) -> Vec<&str> {
        let vec: Vec<&str> = self.0.split('\n').collect();
        vec
    }
}

trait Diagnostic {
    fn binary_diagnostic(self) -> usize;
    fn life_support_rating(self) -> usize;
}

impl Diagnostic for std::vec::Vec<&str> {
    fn binary_diagnostic(self) -> usize {
        let size = self[0].len();

        let mut count_positive = vec![0; size];
        let mut count_negative = vec![0; size];

        let mut one: Vec<i64> = Vec::new();
        let mut zero: Vec<i64> = Vec::new();
        for elem in 0..self.len() {
            let b: Vec<char> = self[elem].chars().collect();
            for index in 0..b.len() {
                if b[index] == '1' {
                    count_positive[index] += 1;
                } else {
                    count_negative[index] += 1;
                }
            }
        }

        for elem in 0..size {
            if count_positive[elem] > count_negative[elem] {
                one.push(1);
                zero.push(0);
            } else {
                one.push(0);
                zero.push(1);
            }
        }

        let dec1: usize = one
            .iter()
            .rev()
            .enumerate()
            .map(|(i, b)| 2usize.pow(i as u32) * *b as usize)
            .sum();
        let dec2: usize = zero
            .iter()
            .rev()
            .enumerate()
            .map(|(i, b)| 2usize.pow(i as u32) * *b as usize)
            .sum();

        let solve = dec1 * dec2;

        solve
    }

    fn life_support_rating(self) -> usize {
        let mut vec: Vec<Vec<i64>> = Vec::new();

        for elem in 0..self.len() {
            let c: Vec<i64> = self[elem]
                .to_string()
                .chars()
                .map(|c| c as i64 - 0x30)
                .collect();
            vec.push(c);
        }

        let len = vec[0].len();
        let bits = filter(len, &mut vec);
        let o2: usize = bits[0]
            .iter()
            .rev()
            .enumerate()
            .map(|(i, b)| 2usize.pow(i as u32) * *b as usize)
            .sum();

        let co2: usize = bits[1]
            .iter()
            .rev()
            .enumerate()
            .map(|(i, b)| 2usize.pow(i as u32) * *b as usize)
            .sum();

        o2 * co2
    }
}

fn filter(len: usize, vec: &mut Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut o2_bit = vec;
    let mut co2_bit = o2_bit.clone();
    for index in 0..len {
        let o2 = oxygen(index, &o2_bit);
        let mut i = 0;
        while i < o2_bit.len() {
            if o2_bit[i][index] == o2 {
                i += 1;
            } else {
                if co2_bit.len() == 1 {
                    break;
                }
                o2_bit.remove(i);
            }
        }
    }
    for index in 0..len {
        let co2 = carbon(index, &co2_bit);
        let mut i = 0;
        while i < co2_bit.len() {
            if co2_bit[i][index] == co2 {
                i += 1;
            } else {
                if co2_bit.len() == 1 {
                    break;
                }
                co2_bit.remove(i);
            }
        }
    }
    let vec: Vec<Vec<i64>> = vec![o2_bit[0].clone(), co2_bit[0].clone()];

    vec
}

fn oxygen(index: usize, vec: &Vec<Vec<i64>>) -> i64 {
    let mut bit = 0;
    for position in 0..vec.len() {
        if vec[position][index] == 1 {
            bit += 1;
        } else {
            bit -= 1;
        }
    }

    match bit {
        _ if bit > 0 => 1,
        0 => 1,
        _ if bit < 0 => 0,
        _ => 0,
    }
}

fn carbon(index: usize, vec: &Vec<Vec<i64>>) -> i64 {
    let mut bit = 0;
    for position in 0..vec.len() {
        if vec[position][index] == 1 {
            bit += 1;
        } else {
            bit -= 1;
        }
    }

    match bit {
        _ if bit > 0 => 0,
        0 => 0,
        _ if bit < 0 => 1,
        _ => 0,
    }
}

fn file() {
    let file: Vec<String> = env::args().collect();
    let path = &file[1];
    let content = fs::read_to_string(path).expect("error");
    let bin = Binary(content).split_line().life_support_rating(); //.life_support_rating();
                                                                  //let ls = Binary(content).split_line().binary_diagnostic();
                                                                  //println!("{}", bin);
    println!("{}", bin)
}

fn main() {
    file()
}
