#[macro_export]
macro_rules! input {
    () => {};
    ($x:expr) => {
        const INPUT: &str = include_str!(concat!("./day", $x, ".txt"));
    };
    ($x:expr, example) => {
        const INPUT: &str = include_str!(concat!("./day", $x, "_example.txt"));
    };
}

#[macro_export]
macro_rules! table_of_contents {
    () => {
        pub const TABLE_OF_CONTENTS: [[fn() -> String; 2]; 25] = [
            [day1::part1, day1::part2],
            [day2::part1, day2::part2],
            [day3::part1, day3::part2],
            [day4::part1, day4::part2],
            [day5::part1, day5::part2],
            [day6::part1, day6::part2],
            [day7::part1, day7::part2],
            [day8::part1, day8::part2],
            [day9::part1, day9::part2],
            [day10::part1, day10::part2],
            [day11::part1, day11::part2],
            [day12::part1, day12::part2],
            [day13::part1, day13::part2],
            [day14::part1, day14::part2],
            [day15::part1, day15::part2],
            [day16::part1, day16::part2],
            [day17::part1, day17::part2],
            [day18::part1, day18::part2],
            [day19::part1, day19::part2],
            [day20::part1, day20::part2],
            [day21::part1, day21::part2],
            [day22::part1, day22::part2],
            [day23::part1, day23::part2],
            [day24::part1, day24::part2],
            [day25::part1, day25::part2],
        ];
    };
}

#[macro_export]
macro_rules! test {
    () => {};
    ($part1:expr) => {
        #[test]
        fn test_part1() {
            assert_eq!(part1(), $part1.to_string())
        }
    };
    ($part1:expr, $part2:expr) => {
        #[test]
        fn test_part1() {
            assert_eq!(part1(), $part1.to_string())
        }
        #[test]
        fn test_part2() {
            assert_eq!(part2(), $part2.to_string())
        }
    };
}
