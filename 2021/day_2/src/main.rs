use std::env;
use std::fs;

struct Depth(String);

trait Convert {
    fn split_line(&mut self) -> Vec<&str>;
}

trait CountPosition {
    fn position(self) -> Vec<i32>;
}

impl Convert for Depth {
    fn split_line(&mut self) -> Vec<&str> {
        let x: Vec<&str> = self.0.split('\n').collect();
        x
    }
}

impl CountPosition for std::vec::Vec<&str> {
    fn position(self) -> Vec<i32> {
        let mut position: Vec<i32> = Vec::new();
        let mut forward = 0;
        let mut depth = 0;
        let mut aim = 0;

        for elem in self {
            let direction: Vec<&str> = elem.split(' ').collect();
            let number = direction[1].parse::<i32>().unwrap(); //transform str into i32
            match direction[0] {
                "forward" => {
                    forward += number;
                    depth += aim * number; 
                }
                "down" => {
                    aim += number;
                }
                "up" => {
                    aim -= number;
                }
                &_ => panic!("error"),
            }
        }

        position.push(forward);
        position.push(aim);
        position.push(depth);

        position
    }
}

fn main() {
    let file: Vec<String> = env::args().collect();
    let path = &file[1];

    let content = fs::read_to_string(path).expect("Something went wrong while opening the file");

    let x = Depth(content).split_line().position();

    println!(
        "Forward: {}\nDepth: {}\nForward * Depth == {}",
        x[0],
        x[1],
        x[0] * x[1]
    );

    println!(
        "PART 2\n\nForward: {}\nDepth: {}\nForward * Depth == {}",
        x[0],
        x[2],
        x[0] * x[2]
    );
}
