use std::env;

type Day = [fn() -> String; 2];
type Year = [Day; 25];

const TABLE_OF_CONTENTS: [Year; 7] = [
    aoc_2015::TABLE_OF_CONTENTS,
    aoc_2016::TABLE_OF_CONTENTS,
    aoc_2017::TABLE_OF_CONTENTS,
    aoc_2018::TABLE_OF_CONTENTS,
    aoc_2019::TABLE_OF_CONTENTS,
    aoc_2020::TABLE_OF_CONTENTS,
    aoc_2021::TABLE_OF_CONTENTS,
];

fn get(year: usize, day: usize, part: usize) -> String {
    TABLE_OF_CONTENTS[year - 2015][day - 1][part - 1]()
}

fn arg(i: usize) -> usize {
    env::args().nth(i).unwrap().parse().unwrap()
}

fn main() {
    let [year, day, part] = [arg(1), arg(2), arg(3)];
    println!("{}", get(year, day, part));
}
