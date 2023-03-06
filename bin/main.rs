use std::env;

const TABLE_OF_CONTENTS: &[shared::Year] = &[
    aoc_2015::TABLE_OF_CONTENTS,
    aoc_2016::TABLE_OF_CONTENTS,
    aoc_2017::TABLE_OF_CONTENTS,
    aoc_2018::TABLE_OF_CONTENTS,
    aoc_2019::TABLE_OF_CONTENTS,
    aoc_2020::TABLE_OF_CONTENTS,
    aoc_2021::TABLE_OF_CONTENTS,
    aoc_2022::TABLE_OF_CONTENTS,
];
const START_YEAR: usize = 2015;
const END_YEAR: usize = START_YEAR + TABLE_OF_CONTENTS.len() - 1;
const N_DAYS: usize = TABLE_OF_CONTENTS[0].len();
const N_PARTS: usize = 2;

fn main() {
    let args = env::args().flat_map(|s| s.parse()).collect::<Vec<_>>();
    let valid = |arg: Option<&_>, min, max| {
        arg.is_some() && min <= *arg.unwrap() && *arg.unwrap() <= max
    };
    let valid_year = |i| valid(args.get(i), START_YEAR, END_YEAR);
    let valid_day = |i| valid(args.get(i), 1, N_DAYS);
    let valid_part = |i| valid(args.get(i), 1, N_PARTS);
    let both = (1..=N_PARTS).collect();
    let (year, day, part) = if valid_year(0) && valid_day(1) && valid_part(2) {
        (args[0], args[1], vec![args[2]])
    } else if valid_year(0) && valid_day(1) {
        (args[0], args[1], both)
    } else if valid_day(0) && valid_part(1) {
        (END_YEAR, args[0], vec![args[1]])
    } else if valid_year(0) {
        let recent = TABLE_OF_CONTENTS[args[0] - START_YEAR]
            .iter()
            .take_while(|day| day.puzzle != "")
            .count();
        (args[0], recent, both)
    } else if valid_day(0) {
        (END_YEAR, args[0], both)
    } else if args.is_empty() {
        let recent = TABLE_OF_CONTENTS
            .last()
            .unwrap()
            .iter()
            .take_while(|day| day.puzzle != "")
            .count();
        (END_YEAR, recent, both)
    } else {
        println!("Usage: <year> <day> <part>");
        return;
    };
    for pt in part {
        get(year, day, pt);
    }
}

fn get(yr: usize, dy: usize, pt: usize) {
    let shared::Day {
        part,
        puzzle,
        example,
    } = TABLE_OF_CONTENTS[yr - START_YEAR][dy - 1];
    let ex = example
        .iter()
        .map(|s| part[pt - 1](s))
        .collect::<Vec<_>>()
        .join(" ");
    println!("{yr} day{dy} part{pt}: {}", part[pt - 1](puzzle));
    let indent = 7 + (dy >= 10) as usize;
    println!("{}example{pt}: {}", ".".repeat(indent), ex);
}
