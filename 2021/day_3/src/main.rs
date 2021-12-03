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
    fn binary_diagnostic(self);
}

impl Diagnostic for std::vec::Vec<&str> {
    fn binary_diagnostic(self) {
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
            } else {
                one.push(0);
            }

            if count_positive[elem] < count_negative[elem] {
                zero.push(1);
            } else {
                zero.push(0);
            }

        }

        println!("{:?}", one.iter().fold(0, |acc, elem| acc * 10 + elem));
        println!("{:?}", zero.iter().fold(0, |acc, elem| acc * 10 + elem));

        //3882564
        todo!()
    }
}

fn file() {
    let file: Vec<String> = env::args().collect();
    let path = &file[1];
    let content = fs::read_to_string(path).expect("error");
    
    let bin = Binary(content).split_line().binary_diagnostic();
}

fn main() {
    file()
}
