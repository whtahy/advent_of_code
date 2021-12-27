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

pub fn get(year: usize, day: usize, part: usize) -> String {
    TABLE_OF_CONTENTS[year - 2015][day - 1][part - 1]()
}

macro_rules! table_of_contents {
    () => {
        pub const TABLE_OF_CONTENTS: crate::Year = [
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

pub mod aoc_2021 {
    table_of_contents!();

    macro_rules! input {
        ($x:expr) => {
            const INPUT: &str = include_str!(concat!("../input/2021/day", $x, ".txt"));
        };
    }

    pub mod day1 {
        input!(1);

        pub fn part1() -> String {
            // println!("hello world!");
            INPUT.to_string()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day2 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day3 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day4 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day5 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day6 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day7 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day8 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day9 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day10 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day11 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day12 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day13 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day14 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day15 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day16 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day17 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day18 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day19 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day20 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day21 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day22 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day23 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day24 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day25 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }
}

pub mod aoc_2020 {
    table_of_contents!();

    macro_rules! input {
        ($x:expr) => {
            const INPUT: &str = include_str!(concat!("../input/2020/day", $x, ".txt"));
        };
    }

    pub mod day1 {
        input!(1);
        use std::collections::HashSet;

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day1::part1(), 800_139.to_string());
        /// ```
        pub fn part1() -> String {
            let mut history = HashSet::new();
            for line in INPUT.lines() {
                let x = line.parse::<u32>().unwrap();
                let y = 2020 - x;
                if history.contains(&y) {
                    return (x * y).to_string();
                } else {
                    history.insert(x);
                }
            }
            panic!()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day1::part2(), 59_885_340.to_string());
        /// ```
        pub fn part2() -> String {
            let v = INPUT
                .lines()
                .map(|line| line.parse::<_>().unwrap())
                .collect::<Vec<u32>>();
            for a in &v {
                for b in &v {
                    for c in &v {
                        if a + b + c == 2020 {
                            return (a * b * c).to_string();
                        }
                    }
                }
            }
            panic!()
        }
    }

    pub mod day2 {
        input!(2);

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day2::part1(), 655.to_string());
        /// ```
        pub fn part1() -> String {
            INPUT
                .lines()
                .filter(|line| {
                    let (i, j, ch, pw) = parse_line(line);
                    let n = pw.into_iter().filter(|&x| x == ch).count();
                    i <= n && n <= j
                })
                .count()
                .to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day2::part2(), 673.to_string());
        /// ```
        pub fn part2() -> String {
            INPUT
                .lines()
                .filter(|line| {
                    let (i, j, ch, pw) = parse_line(line);
                    (pw[i - 1] == ch) ^ (pw[j - 1] == ch)
                })
                .count()
                .to_string()
        }

        fn parse_line(line: &str) -> (usize, usize, char, Vec<char>) {
            let split = line.split(['-', ' ', ':'].as_ref()).collect::<Vec<_>>();
            let i = split[0].parse::<_>().unwrap();
            let j = split[1].parse::<_>().unwrap();
            let ch = split[2].chars().next().unwrap();
            let pw = split[4].chars().collect::<Vec<_>>();
            (i, j, ch, pw)
        }
    }

    pub mod day3 {
        input!(3);

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day3::part1(), 250.to_string());
        /// ```
        pub fn part1() -> String {
            let trees = parse_input();
            travel(&trees, 3, 1).to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day3::part2(), 1_592_662_500.to_string());
        /// ```
        pub fn part2() -> String {
            let trees = parse_input();
            [
                travel(&trees, 1, 1),
                travel(&trees, 3, 1),
                travel(&trees, 5, 1),
                travel(&trees, 7, 1),
                travel(&trees, 1, 2),
            ]
            .iter()
            .product::<u32>()
            .to_string()
        }

        fn travel(trees: &[Vec<char>], right: usize, down: usize) -> u32 {
            let n_cols = trees[0].len();
            let mut c = 0;
            let mut ans = 0;
            for r in (down..trees.len()).step_by(down) {
                c = (c + right) % n_cols;
                if trees[r][c] == '#' {
                    ans += 1;
                }
            }
            ans
        }

        fn parse_input() -> Vec<Vec<char>> {
            INPUT
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        }
    }

    pub mod day4 {
        input!(4);
        use std::collections::HashSet;

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day4::part1(), 213.to_string());
        /// ```
        pub fn part1() -> String {
            let required_fields: HashSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .cloned()
                .collect();

            INPUT
                .split("\r\n\r\n")
                .map(|passport| {
                    let mut fields = HashSet::new();
                    for kv in passport.split_whitespace() {
                        let k = kv.split(':').next().unwrap();
                        fields.insert(k);
                    }
                    fields
                })
                .filter(|fields| fields.is_superset(&required_fields))
                .count()
                .to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day4::part2(), 147.to_string());
        /// ```
        pub fn part2() -> String {
            let required_fields: HashSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .cloned()
                .collect();

            INPUT
                .split("\r\n\r\n")
                .map(|passport| {
                    let mut fields = HashSet::new();
                    for kv in passport.split_whitespace() {
                        let mut kv = kv.split(':');
                        let k = kv.next().unwrap();
                        let v = kv.next().unwrap();
                        if is_valid(k, v) {
                            fields.insert(k);
                        }
                    }
                    fields
                })
                .filter(|fields| fields.is_superset(&required_fields))
                .count()
                .to_string()
        }

        fn is_valid(k: &str, v: &str) -> bool {
            match k {
                "byr" => (1920..=2002).contains(&v.parse().unwrap()),
                "iyr" => (2010..=2020).contains(&v.parse().unwrap()),
                "eyr" => (2020..=2030).contains(&v.parse().unwrap()),
                "hgt" => {
                    let hgt: String = v.chars().filter(char::is_ascii_digit).collect();
                    let unit: String = v.chars().filter(char::is_ascii_alphabetic).collect();
                    match (hgt.parse(), unit.as_str()) {
                        (Ok(h), "cm") => (150..=193).contains(&h),
                        (Ok(h), "in") => (59..=76).contains(&h),
                        _ => false,
                    }
                }
                "hcl" => {
                    let mut hcl = v.chars();
                    hcl.next().unwrap() == '#' && hcl.all(|ch| ch.is_digit(16))
                }
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
                "pid" => v.len() == 9 && v.chars().all(|ch| ch.is_ascii_digit()),
                _ => false,
            }
        }
    }

    pub mod day5 {
        input!(5);
        use std::collections::HashSet;

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day5::part1(), 935.to_string());
        /// ```
        pub fn part1() -> String {
            INPUT
                .lines()
                .map(decode)
                .map(seat_id)
                .max()
                .unwrap()
                .to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day5::part2(), 743.to_string());
        /// ```
        pub fn part2() -> String {
            let seat_ids: HashSet<_> = INPUT.lines().map(decode).map(seat_id).collect();
            for r in 1..=126 {
                for c in 0..=7 {
                    let seat = seat_id((r, c));
                    if !seat_ids.contains(&seat)
                        && seat_ids.contains(&(seat - 1))
                        && seat_ids.contains(&(seat + 1))
                    {
                        return seat.to_string();
                    }
                }
            }
            panic!()
        }

        fn seat_id(seat: (u32, u32)) -> u32 {
            let (row, col) = seat;
            8 * row + col
        }

        fn decode(s: &str) -> (u32, u32) {
            let binary = s
                .chars()
                .map(|ch| match ch {
                    'F' | 'L' => '0',
                    'B' | 'R' => '1',
                    _ => panic!(),
                })
                .collect::<String>();
            let row = u32::from_str_radix(&binary[0..=6], 2).unwrap();
            let col = u32::from_str_radix(&binary[7..], 2).unwrap();
            (row, col)
        }
    }

    pub mod day6 {
        input!(6);
        use std::collections::HashSet;

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day6::part1(), 6903.to_string());
        /// ```
        pub fn part1() -> String {
            INPUT
                .split("\r\n\r\n")
                .map(|group| {
                    group
                        .chars()
                        .filter(char::is_ascii_alphabetic)
                        .collect::<HashSet<_>>()
                        .len()
                })
                .sum::<usize>()
                .to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day6::part2(), 3_493.to_string());
        /// ```
        pub fn part2() -> String {
            INPUT
                .split("\r\n\r\n")
                .map(|group| {
                    group
                        .lines()
                        .map(|line| line.chars().collect::<HashSet<_>>())
                        .fold(('a'..='z').collect(), |acc, x| &acc & &x)
                        .len()
                })
                .sum::<usize>()
                .to_string()
        }
    }

    pub mod day7 {
        input!(7);
        use std::collections::{HashMap, HashSet};

        type ColorRules<'a> = HashMap<&'a str, HashSet<&'a str>>;
        // type BagRules<'a> = HashMap<&'a str, Vec<(&'a str, &'a str)>>;

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day7::part1(), 172.to_string());
        /// ```
        pub fn part1() -> String {
            let rules: ColorRules = INPUT
                .lines()
                .map(|rule| {
                    let mut kv = rule.split(" bags contain ");
                    let k = kv.next().unwrap();
                    let v = kv.next().unwrap().split(',').map(color).collect();
                    (k, v)
                })
                .collect();
            rules
                .keys()
                .filter(|k| recurse(&rules, k))
                .count()
                .to_string()
        }

        fn recurse(rules: &ColorRules, c: &str) -> bool {
            if c == "other" || rules[c].contains("other") {
                false
            } else {
                rules[c].contains("shiny gold") || rules[c].iter().any(|x| recurse(rules, x))
            }
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day7::part2(), 39_645.to_string());
        /// ```
        pub fn part2() -> String {
            // let rules: BagRules = INPUT
            //     .lines()
            //     .map(|rule| {
            //         let mut kv = rule.split(" bags contain ");
            //         let k = kv.next().unwrap();
            //         let v = kv
            //             .next()
            //             .unwrap()
            //             .split(',')
            //             .map(|b| (count(b), color(b)))
            //             .collect();
            //         (k, v)
            //     })
            //     .collect();
            "wip".to_string()
        }

        fn parse(bags: &str) -> &str {
            bags.rsplitn(2, ' ').last().unwrap().trim()
        }

        fn color(bags: &str) -> &str {
            parse(bags).splitn(2, ' ').last().unwrap()
        }

        // fn count(bags: &str) -> &str {
        //     parse(bags).splitn(2, ' ').next().unwrap()
        // }
    }

    pub mod day8 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day9 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day10 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day11 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day12 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day13 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day14 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day15 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day16 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day17 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day18 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day19 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day20 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day21 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day22 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day23 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day24 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day25 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }
}

pub mod aoc_2019 {
    table_of_contents!();

    macro_rules! input {
        ($x:expr) => {
            const INPUT: &str = include_str!(concat!("../input/2019/day", $x, ".txt"));
        };
    }

    pub mod day1 {
        input!(1);

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day1::part1(), 3_376_997.to_string());
        /// ```
        pub fn part1() -> String {
            INPUT
                .lines()
                .fold(0, |sum, x| sum + fuel(x.parse().unwrap()))
                .to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day1::part2(), 5_062_623.to_string());
        /// ```
        pub fn part2() -> String {
            INPUT
                .lines()
                .fold(0, |sum, x| sum + total_fuel(x.parse().unwrap()))
                .to_string()
        }

        fn fuel(x: i32) -> i32 {
            x / 3 - 2
        }

        fn total_fuel(mut x: i32) -> i32 {
            let mut f = 0;
            while fuel(x) > 0 {
                x = fuel(x);
                f += x;
            }
            f
        }
    }

    pub mod day2 {
        input!(2);

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day2::part1(), 3_562_624.to_string());
        /// ```
        pub fn part1() -> String {
            run_program(12, 2).to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day2::part2(), 8_298.to_string());
        /// ```
        pub fn part2() -> String {
            for n in 0..=99 {
                for v in 0..=99 {
                    if run_program(n, v) == 19_690_720 {
                        return (100 * n + v).to_string();
                    }
                }
            }
            panic!()
        }

        fn run_program(noun: usize, verb: usize) -> usize {
            let mut v = INPUT
                .split(',')
                .map(|x| str::replace(x, "\r\n", "").parse().unwrap())
                .collect::<Vec<_>>();

            v[1] = noun;
            v[2] = verb;

            for i in (0..).step_by(4) {
                let j = v[i + 3];
                match v[i] {
                    99 => break,
                    1 => {
                        v[j] = v[v[i + 1]] + v[v[i + 2]];
                    }
                    2 => {
                        v[j] = v[v[i + 1]] * v[v[i + 2]];
                    }
                    _ => panic!(),
                };
            }

            v[0]
        }
    }

    pub mod day3 {
        input!(3);
        use std::cmp::{max, min};
        use std::collections::BTreeSet;

        // HLine: y, x1, x2
        // VLine: x, y1, y2
        type Line = (i32, i32, i32);

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day3::part1(), 209.to_string());
        /// ```
        pub fn part1() -> String {
            type WireH = BTreeSet<(i32, Line)>;
            type WireV = BTreeSet<(i32, Line)>;

            fn parse_wire(wire: &str) -> (WireH, WireV) {
                let mut pt: (i32, i32) = (0, 0);
                let mut h = BTreeSet::new();
                let mut v = BTreeSet::new();

                for dir in wire.split(',') {
                    let (dir, len) = parse_dir(dir);

                    let (x, y) = pt;
                    pt = travel(pt, dir, len);

                    match dir {
                        "L" => h.insert((y.abs(), (y, x, x - len))),
                        "R" => h.insert((y.abs(), (y, x, x + len))),
                        "U" => v.insert((x.abs(), (x, y, y + len))),
                        "D" => v.insert((x.abs(), (x, y, y - len))),
                        _ => panic!(),
                    };
                }

                (h, v)
            }

            let mut wires = INPUT.lines();
            let (red_h, red_v) = parse_wire(wires.next().unwrap());
            let (black_h, black_v) = parse_wire(wires.next().unwrap());

            let mut dx = BTreeSet::new();

            fn compare(mut dx: BTreeSet<i32>, wire_h: &WireH, wire_v: &WireV) -> BTreeSet<i32> {
                for (_, h) in wire_h {
                    for (_, v) in wire_v {
                        if let Some((x, y)) = cross(*h, *v) {
                            let d = x.abs() + y.abs();
                            if d > 0 {
                                dx.insert(d);
                                break;
                            }
                        }
                    }
                }
                dx
            }

            dx = compare(dx, &red_h, &black_v);
            dx = compare(dx, &black_h, &red_v);

            dx.iter().next().unwrap().to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day3::part2(), 43258.to_string());
        /// ```
        pub fn part2() -> String {
            type Wire = Vec<(String, Line)>;

            fn parse_wire(wire: &str) -> Wire {
                let mut pt: (i32, i32) = (0, 0);
                let mut w = Vec::new();

                for dir in wire.split(',') {
                    let (dir, len) = parse_dir(dir);

                    let (x, y) = pt;
                    pt = travel(pt, dir, len);

                    let l = match dir {
                        "L" => ("H", (y, x, x - len)),
                        "R" => ("H", (y, x, x + len)),
                        "U" => ("V", (x, y, y + len)),
                        "D" => ("V", (x, y, y - len)),
                        _ => panic!(),
                    };
                    w.push((l.0.to_string(), l.1));
                }

                w
            }

            let mut wires = INPUT.lines();
            let red = parse_wire(wires.next().unwrap());
            let black = parse_wire(wires.next().unwrap());

            let mut dx = BTreeSet::new();

            let mut d_red = 0;
            for (dir_r, r) in &red {
                let mut d_black = 0;
                for (dir_b, b) in &black {
                    if dir_r != dir_b && cross(*r, *b).is_some() {
                        let d = d_red + d_black + (b.0 - r.1).abs() + (r.0 - b.1).abs();
                        if d > 0 {
                            dx.insert(d);
                        }
                    }
                    d_black += (b.1 - b.2).abs();
                }
                d_red += (r.1 - r.2).abs();
            }

            dx.iter().next().unwrap().to_string()
        }

        fn parse_dir(s: &str) -> (&str, i32) {
            let (dir, len) = s.split_at(1);
            (dir, len.parse::<_>().unwrap())
        }

        fn travel(pt: (i32, i32), dir: &str, len: i32) -> (i32, i32) {
            let (x, y) = pt;
            match dir {
                "L" => (x - len, y),
                "R" => (x + len, y),
                "U" => (x, y + len),
                "D" => (x, y - len),
                _ => panic!(),
            }
        }

        fn cross(a: Line, b: Line) -> Option<(i32, i32)> {
            if min(a.1, a.2) < b.0
                && b.0 < max(a.1, a.2)
                && min(b.1, b.2) < a.0
                && a.0 < max(b.1, b.2)
            {
                Some((b.0, a.0))
            } else {
                None
            }
        }
    }

    pub mod day4 {
        input!(4);

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day4::part1(), 579.to_string());
        /// ```
        pub fn part1() -> String {
            let (min, max) = parse_input();

            fn adjacent(x: i32) -> bool {
                digits(x).windows(2).any(|s| s[0] == s[1])
            }

            (min..=max)
                .filter(|&x| adjacent(x) && monotonic(x))
                .count()
                .to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day4::part2(), 358.to_string());
        /// ```
        pub fn part2() -> String {
            let (min, max) = parse_input();

            fn adjacent(x: i32) -> bool {
                let d = digits(x);
                (d[0] == d[1] && d[1] != d[2])
                    || (d[5] == d[4] && d[4] != d[3])
                    || d.windows(4)
                        .any(|s| s[0] != s[1] && s[1] == s[2] && s[2] != s[3])
            }

            (min..=max)
                .filter(|&x| adjacent(x) && monotonic(x))
                .count()
                .to_string()
        }

        fn parse_input() -> (i32, i32) {
            let mut iter = INPUT.lines().next().unwrap().split('-');
            let min = iter.next().unwrap().parse::<i32>().unwrap();
            let max = iter.next().unwrap().parse::<i32>().unwrap();
            (min, max)
        }

        fn monotonic(x: i32) -> bool {
            digits(x).windows(2).all(|s| s[0] <= s[1])
        }

        fn digits(x: i32) -> Vec<u32> {
            x.to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<_>()
        }
    }

    pub mod day5 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day6 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day7 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day8 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day9 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day10 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day11 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day12 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day13 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day14 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day15 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day16 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day17 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day18 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day19 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day20 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day21 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day22 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day23 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day24 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day25 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }
}

pub mod aoc_2017 {
    table_of_contents!();

    pub mod day1 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day2 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day3 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day4 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day5 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day6 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day7 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day8 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day9 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day10 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day11 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day12 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day13 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day14 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day15 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day16 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day17 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day18 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day19 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day20 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day21 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day22 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day23 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day24 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day25 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }
}

pub mod aoc_2016 {
    table_of_contents!();

    pub mod day1 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day2 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day3 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day4 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day5 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day6 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day7 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day8 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day9 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day10 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day11 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day12 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day13 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day14 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day15 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day16 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day17 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day18 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day19 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day20 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day21 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day22 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day23 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day24 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day25 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }
}

pub mod aoc_2015 {
    table_of_contents!();

    pub mod day1 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day2 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day3 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day4 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day5 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day6 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day7 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day8 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day9 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day10 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day11 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day12 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day13 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day14 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day15 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day16 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day17 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day18 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day19 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day20 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day21 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day22 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day23 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day24 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }

    pub mod day25 {
        pub fn part1() -> String {
            todo!()
        }

        pub fn part2() -> String {
            todo!()
        }
    }
}
