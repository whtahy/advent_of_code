use std::env;
use std::panic;

type Day = [fn() -> String; 2];
type Year = [Day; 25];

const TABLE_OF_CONTENTS: [Year; 8] = [
    aoc_2015::TABLE_OF_CONTENTS,
    aoc_2016::TABLE_OF_CONTENTS,
    aoc_2017::TABLE_OF_CONTENTS,
    aoc_2018::TABLE_OF_CONTENTS,
    aoc_2019::TABLE_OF_CONTENTS,
    aoc_2020::TABLE_OF_CONTENTS,
    aoc_2021::TABLE_OF_CONTENTS,
    aoc_2022::TABLE_OF_CONTENTS,
];

fn main() {
    let args: Vec<usize> = env::args().filter_map(|s| s.parse().ok()).collect();
    if let [year, day] = args[..] {
        let ans1 = get(year, day, 1);
        let ans2 = get(year, day, 2);
        println!("{year} Day{day} Part1: {ans1}");
        println!("{year} Day{day} Part2: {ans2}");
    } else if let [year, day, part] = args[..] {
        let ans = get(year, day, part);
        println!("{year} Day{day} Part{part}: {ans}");
    } else {
        get_most_recent()
    }
}

fn get(year: usize, day: usize, part: usize) -> String {
    TABLE_OF_CONTENTS[year - 2015][day - 1][part - 1]()
}

fn get_most_recent() {
    panic::set_hook(Box::new(|_| {}));
    for year in (2015..=2021).rev() {
        for day in (1..=25).rev() {
            for part in (1..=2).rev() {
                if let Ok(ans) =
                    std::panic::catch_unwind(|| get(year, day, part))
                {
                    return println!("{year} Day{day} Part{part}: {ans}");
                }
            }
        }
    }
}
