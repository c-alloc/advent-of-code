use std::env;
use std::fs;

struct Depth(String);

trait CheckIncrease {
    fn has_increase(vec: &mut Vec<i32>) -> i32;
    fn three_measurement(vec: &mut Vec<i32>) -> Vec<i32>;
    fn to_vec_of_i32(self) -> Vec<i32>;
}

impl Depth {
    fn new(text: String) -> Depth {
        Depth(text)
    }
}

impl CheckIncrease for Depth {
    //sums how many times the number is greater 
    fn has_increase(vec: &mut Vec<i32>) -> i32 {
        let mut time_increase: i32 = 0;

        for elem in 1..vec.len() {
            if vec[elem] > vec[elem - 1] {
                time_increase = time_increase + 1;
            }
        }
        time_increase
    }

    //return the sum of three-measurement numbers of document
    fn three_measurement(vec: &mut Vec<i32>) -> Vec<i32> {
        let mut sum_vec: Vec<i32> = Vec::new();
        let mut i = 0;

        while i <= vec.len() - 3 {
            let sum = vec[i] + vec[i + 1] + vec[i + 2];

            i += 1;
            sum_vec.push(sum);
        }

        sum_vec
    }

    // convert text string to vec of integer
    fn to_vec_of_i32(self) -> Vec<i32> {
        let num: Vec<i32> = self
            .0
            .split_whitespace()
            .map(|n| n.parse().expect("parse error"))
            .collect();

        num
    }
}

fn main() {
    let file: Vec<String> = env::args().collect();
    let path = &file[1];

    let content = fs::read_to_string(path)
        .expect("Something went wrong, file missing");

    let mut vec = Depth::new(content).to_vec_of_i32();
    println!(
        "Times where the number was increased: {}",
        Depth::has_increase(&mut vec)
    );


    let mut new_vec = Depth::three_measurement(&mut vec);
    println!(
        "Times where the sum of three-numbers was increased: {}",
        Depth::has_increase(&mut new_vec)
    );
}
