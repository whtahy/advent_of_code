const TABLE_OF_CONTENTS: &[shared::Year] = &[
    aoc_2015::TABLE_OF_CONTENTS,
    aoc_2016::TABLE_OF_CONTENTS,
    aoc_2017::TABLE_OF_CONTENTS,
    aoc_2018::TABLE_OF_CONTENTS,
    aoc_2019::TABLE_OF_CONTENTS,
    aoc_2020::TABLE_OF_CONTENTS,
    aoc_2021::TABLE_OF_CONTENTS,
    aoc_2022::TABLE_OF_CONTENTS,
    aoc_2023::TABLE_OF_CONTENTS,
    aoc_2024::TABLE_OF_CONTENTS,
];
const FIRST_YEAR: usize = 2015;
const LAST_YEAR: usize = FIRST_YEAR + TABLE_OF_CONTENTS.len() - 1;
const N_DAYS: usize = 25;
const N_PARTS: usize = 2;

fn main() {
    let args: Vec<usize> = std::env::args().flat_map(|s| s.parse()).collect();
    let recent_day = |year: &usize| {
        TABLE_OF_CONTENTS[year - FIRST_YEAR]
            .iter()
            .take_while(|day| day.example != [""])
            .count()
    };
    let both_parts = 1..=N_PARTS;
    let (year, day, part) = match (args.first(), args.get(1), args.get(2)) {
        // year day part
        (
            Some(year @ FIRST_YEAR..=LAST_YEAR),
            Some(day @ 1..=N_DAYS),
            Some(&part @ 1..=N_PARTS),
        ) => (year, day, part..=part),
        // year day -> both parts
        (Some(year @ FIRST_YEAR..=LAST_YEAR), Some(day @ 1..=N_DAYS), _) => {
            (year, day, both_parts)
        }
        // year -> recent day, both parts
        (Some(year @ FIRST_YEAR..=LAST_YEAR), _, _) => {
            (year, &recent_day(year), both_parts)
        }
        // day part -> last year
        (Some(day @ 1..=N_DAYS), Some(&part @ 1..=N_PARTS), _) => {
            (&LAST_YEAR, day, part..=part)
        }
        // day -> last year, both parts
        (Some(day @ 1..=N_DAYS), _, _) => (&LAST_YEAR, day, both_parts),
        // <blank> -> last year, recent day, both parts
        (None, None, None) => (&LAST_YEAR, &recent_day(&LAST_YEAR), both_parts),
        _ => {
            println!("Usage: <year> <day> <part>");
            return;
        }
    };
    for pt in part {
        let shared::Day {
            parts,
            puzzle,
            example,
        } = TABLE_OF_CONTENTS[year - FIRST_YEAR][day - 1];
        println!(
            "{year} day{day} example{pt}: {}",
            example
                .iter()
                .map(|s| parts[pt - 1](s))
                .collect::<Vec<_>>()
                .join(" ")
        );
        if !puzzle.is_empty() {
            println!("{year} day{day} part{pt}...: {}", parts[pt - 1](puzzle));
        }
    }
}
