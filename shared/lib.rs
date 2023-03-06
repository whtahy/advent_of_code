pub type Year = [Day; 25];
pub struct Day {
    pub part: [Part; 2],
    pub puzzle: &'static str,
    pub example: &'static [&'static str],
}
type Part = fn(&str) -> String;

#[macro_export]
macro_rules! puzzle {
    () => {
        pub const PUZZLE: &str = "";
    };
    ($day:expr) => {
        pub const PUZZLE: &str = include_str!(concat!("./day", $day, ".txt"));
    };
    ($day:expr, $part1:expr) => {
        shared::puzzle!($day);
        #[test]
        pub fn test_part1() {
            assert_eq!(part1(PUZZLE), $part1.to_string());
        }
    };
    ($day:expr, $part1:expr, $part2:expr) => {
        shared::puzzle!($day, $part1);
        #[test]
        pub fn test_part2() {
            assert_eq!(part2(PUZZLE), $part2.to_string());
        }
    };
}

#[macro_export]
macro_rules! example {
    () => {
        pub const EXAMPLE: &'static [&str] = &[""];
    };
    ($day:expr) => {
        pub const EXAMPLE: &'static [&str] =
            &[include_str!(concat!("./example", $day, ".txt"))];
    };
    ($day:expr, $part1:expr) => {
        shared::example!($day);
        #[test]
        fn test_part1_example() {
            assert_eq!(part1(EXAMPLE[0]), $part1.to_string());
        }
    };
    ($day:expr, $part1:expr, $part2:expr) => {
        shared::example!($day, $part1);
        #[test]
        fn test_part2_example() {
            assert_eq!(part2(EXAMPLE[0]), $part2.to_string());
        }
    };
    ($day:expr, $($example:ident => ($part1:expr, ),)+) => {
        pub const EXAMPLE: &'static [&str] = &[$(
            include_str!(concat!(
                "./example", $day, stringify!($example), ".txt"
            )),
        )+];
        #[test]
        fn test_part1_example() {
            for (actual, expected) in EXAMPLE
                .iter()
                .map(|s| part1(s))
                .zip([$($part1.to_string(),)+])
            {
                assert_eq!(actual, expected);
            }
        }
    };
    ($day:expr, $($example:ident => ($part1:expr, $part2:expr),)+) => {
        shared::example!($day, $($example => ($part1, ),)+);
        #[test]
        fn test_part2_example() {
            for (actual, expected) in EXAMPLE
                .iter()
                .map(|s| part2(s))
                .zip([$($part2.to_string(),)+])
            {
                assert_eq!(actual, expected);
            }
        }
    };
}

#[macro_export]
macro_rules! toc_helper {
    ($day:ident) => {
        shared::Day {
            part: [$day::part1, $day::part2],
            puzzle: $day::PUZZLE,
            example: $day::EXAMPLE,
        }
    };
}

#[macro_export]
macro_rules! table_of_contents {
    () => {
        pub const TABLE_OF_CONTENTS: shared::Year = [
            shared::toc_helper!(day1),
            shared::toc_helper!(day2),
            shared::toc_helper!(day3),
            shared::toc_helper!(day4),
            shared::toc_helper!(day5),
            shared::toc_helper!(day6),
            shared::toc_helper!(day7),
            shared::toc_helper!(day8),
            shared::toc_helper!(day9),
            shared::toc_helper!(day10),
            shared::toc_helper!(day11),
            shared::toc_helper!(day12),
            shared::toc_helper!(day13),
            shared::toc_helper!(day14),
            shared::toc_helper!(day15),
            shared::toc_helper!(day16),
            shared::toc_helper!(day17),
            shared::toc_helper!(day18),
            shared::toc_helper!(day19),
            shared::toc_helper!(day20),
            shared::toc_helper!(day21),
            shared::toc_helper!(day22),
            shared::toc_helper!(day23),
            shared::toc_helper!(day24),
            shared::toc_helper!(day25),
        ];
    };
}
