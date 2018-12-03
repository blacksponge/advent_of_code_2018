use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let res: i32 = reader.lines().filter_map(|line| line.unwrap().parse::<i32>().ok()).sum();
    println!("{}", res);
}
