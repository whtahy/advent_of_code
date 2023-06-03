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
const N_DAYS: usize = 25;
const N_PARTS: usize = 2;

fn main() {
    let args = std::env::args().flat_map(|s| s.parse()).collect::<Vec<_>>();
    let valid = |arg: Option<&_>, min, max| {
        arg.is_some() && min <= *arg.unwrap() && *arg.unwrap() <= max
    };
    let valid_year = |i| valid(args.get(i), START_YEAR, END_YEAR);
    let valid_day = |i| valid(args.get(i), 1, N_DAYS);
    let valid_part = |i| valid(args.get(i), 1, N_PARTS);
    let both = Vec::from_iter(1..=N_PARTS);
    // year day part
    let (year, day, part) = if valid_year(0) && valid_day(1) && valid_part(2) {
        (args[0], args[1], vec![args[2]])
    }
    // year day
    else if valid_year(0) && valid_day(1) {
        (args[0], args[1], both)
    }
    // day part -> recent year
    else if valid_day(0) && valid_part(1) {
        (END_YEAR, args[0], vec![args[1]])
    }
    // year -> recent day
    else if valid_year(0) {
        let recent_day = TABLE_OF_CONTENTS[args[0] - START_YEAR]
            .iter()
            .take_while(|day| day.example != [""])
            .count();
        (args[0], recent_day, both)
    }
    // day -> recent year
    else if valid_day(0) {
        (END_YEAR, args[0], both)
    }
    // <none>
    else if args.is_empty() {
        let recent_day = TABLE_OF_CONTENTS
            .last()
            .unwrap()
            .iter()
            .take_while(|day| day.example != [""])
            .count();
        (END_YEAR, recent_day, both)
    } else {
        println!("Usage: <year> <day> <part>");
        return;
    };
    let is_example = std::env::args().any(|arg| arg == "example");
    for pt in part {
        get(year, day, pt, is_example);
    }
}

fn get(yr: usize, dy: usize, pt: usize, is_example: bool) {
    let shared::Day {
        part,
        puzzle,
        example,
    } = TABLE_OF_CONTENTS[yr - START_YEAR][dy - 1];
    if puzzle.is_empty() || is_example {
        println!(
            "{yr} day{dy} part{pt} example: {}",
            example
                .iter()
                .map(|s| part[pt - 1](s))
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else {
        println!("{yr} day{dy} part{pt}: {}", part[pt - 1](puzzle));
    }
}
