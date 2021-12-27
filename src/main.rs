use std::env;

fn arg(i: usize) -> usize {
    env::args().nth(i).unwrap().parse().unwrap()
}

fn main() {
    let [year, day, part] = [arg(1), arg(2), arg(3)];
    println!("{}", advent_of_code::get(year, day, part));
}
