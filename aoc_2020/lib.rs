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

macro_rules! input {
    ($x:expr) => {
        const INPUT: &str = include_str!(concat!("./day", $x, ".txt"));
    };
}

pub mod day1 {
    input!(1);
    use std::collections::HashSet;

    /// ```
    /// assert_eq!(aoc_2020::day1::part1(), 800_139.to_string());
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
    /// assert_eq!(aoc_2020::day1::part2(), 59_885_340.to_string());
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
    /// assert_eq!(aoc_2020::day2::part1(), 655.to_string());
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
    /// assert_eq!(aoc_2020::day2::part2(), 673.to_string());
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
    /// assert_eq!(aoc_2020::day3::part1(), 250.to_string());
    /// ```
    pub fn part1() -> String {
        let trees = parse_input();
        travel(&trees, 3, 1).to_string()
    }

    /// ```
    /// assert_eq!(aoc_2020::day3::part2(), 1_592_662_500.to_string());
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
    /// assert_eq!(aoc_2020::day4::part1(), 213.to_string());
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
    /// assert_eq!(aoc_2020::day4::part2(), 147.to_string());
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
    /// assert_eq!(aoc_2020::day5::part1(), 935.to_string());
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
    /// assert_eq!(aoc_2020::day5::part2(), 743.to_string());
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
    /// assert_eq!(aoc_2020::day6::part1(), 6903.to_string());
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
    /// assert_eq!(aoc_2020::day6::part2(), 3_493.to_string());
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
    /// assert_eq!(aoc_2020::day7::part1(), 172.to_string());
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
    /// assert_eq!(aoc_2020::day7::part2(), 39_645.to_string());
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
