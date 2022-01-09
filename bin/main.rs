// use std::env;
use std::panic;

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

fn main() {
    panic::set_hook(Box::new(|_| {}));
    for year in (2015..=2021).rev() {
        for day in (1..=25).rev() {
            for part in (1..=2).rev() {
                match std::panic::catch_unwind(|| get(year, day, part)) {
                    Ok(ans) => {
                        println!("{} Day{} Part{}: {}", year, day, part, ans);
                        return;
                    }
                    Err(_) => continue,
                }
            }
        }
    }
}
