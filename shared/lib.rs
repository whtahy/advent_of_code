pub type Year = [Day; 25];
pub struct Day {
    pub part: [Part; 2],
    pub puzzle: &'static str,
    pub example: &'static [&'static str],
}
type Part = fn(&str) -> String;

#[macro_export]
macro_rules! day {
    () => {
        pub const PUZZLE: &str = "";
        pub const EXAMPLE: &'static [&str] = &[""];
    };
    ($day:expr) => {
        pub const PUZZLE: &str = include_str!(concat!("./day", $day, ".txt"));
        pub const EXAMPLE: &'static [&str] =
            &[include_str!(concat!("./example", $day, ".txt"))];
    };
    ($day:expr, $($example:ident),+) => {
        pub const PUZZLE: &str = include_str!(concat!("./day", $day, ".txt"));
        pub const EXAMPLE: &'static [&str] = &[$(
            include_str!(concat!(
                "./example", $day, stringify!($example), ".txt"
            ))
        ),+];
    };
}

#[macro_export]
macro_rules! test_puzzle {
    ($part:ident, $test:ident, $soln:expr) => {
        #[test]
        fn $test() {
            assert_eq!($part(PUZZLE), $soln.to_string());
        }
    };
}

#[macro_export]
macro_rules! test_example {
    ($part:ident, $test:ident, [$($soln:expr),+]) => {
        #[test]
        fn $test() {
            let actual = vec![$($soln.to_string()),+];
            let expected = EXAMPLE.iter().map(|s| $part(s)).collect::<Vec<_>>();
            assert_eq!(actual, expected);
        }
    };
    ($part:ident, $test:ident, $soln:expr) => {
        #[test]
        fn $test() {
            assert_eq!($part(EXAMPLE[0]), $soln.to_string());
        }
    };
}

#[macro_export]
macro_rules! part1 {
    () => {};
    ([$($example:expr),+]) => {
        shared::test_example!(part1, test_part1_example, [$($example),+]);
    };
    ($example:expr) => {
        shared::test_example!(part1, test_part1_example, $example);
    };
    ([$($example:expr),+], $puzzle:expr) => {
        shared::test_example!(part1, test_part1_example, [$($example),+]);
        shared::test_puzzle!(part1, test_part1, $puzzle);
    };
    ($example:expr, $puzzle:expr) => {
        shared::test_example!(part1, test_part1_example, $example);
        shared::test_puzzle!(part1, test_part1, $puzzle);
    };
}

#[macro_export]
macro_rules! part2 {
    () => {};
    ([$($example:expr),+]) => {
        shared::test_example!(part2, test_part2_example, [$($example),+]);
    };
    ($example:expr) => {
        shared::test_example!(part2, test_part2_example, $example);
    };
    ([$($example:expr),+], $puzzle:expr) => {
        shared::test_example!(part2, test_part2_example, [$($example),+]);
        shared::test_puzzle!(part2, test_part2, $puzzle);
    };
    ($example:expr, $puzzle:expr) => {
        shared::test_example!(part2, test_part2_example, $example);
        shared::test_puzzle!(part2, test_part2, $puzzle);
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
