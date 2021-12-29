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

    /// ```
    /// assert_eq!(aoc_2021::day1::part1(), 1_521.to_string());
    /// ```
    pub fn part1() -> String {
        let mut sum = 0;
        let mut prev = u32::MAX;
        INPUT
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .for_each(|x| {
                if x > prev {
                    sum += 1;
                }
                prev = x;
            });
        sum.to_string()
    }

    /// ```
    /// assert_eq!(aoc_2021::day1::part2(), 1_543.to_string());
    /// ```
    pub fn part2() -> String {
        let mut sum = 0;
        let mut prev = u32::MAX;
        INPUT
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .windows(3)
            .for_each(|w| match w {
                [a, b, c] => {
                    let x = a + b + c;
                    if x > prev {
                        sum += 1;
                    }
                    prev = x;
                }
                _ => panic!(),
            });
        sum.to_string()
    }
}

pub mod day2 {
    input!(2);

    /// ```
    /// assert_eq!(aoc_2021::day2::part1(), 1_882_980.to_string());
    /// ```
    pub fn part1() -> String {
        let mut forward = 0;
        let mut down = 0;
        let mut up = 0;
        for x in INPUT.lines() {
            let (dir, val) = parse(x);
            match dir {
                "forward" => forward += val,
                "down" => down += val,
                "up" => up += val,
                _ => panic!(),
            }
        }
        (forward * (down - up)).to_string()
    }

    pub fn part2() -> String {
        todo!()
    }

    fn parse(line: &str) -> (&str, u32) {
        let mut split = line.splitn(2, ' ');
        let dir = split.next().unwrap();
        let val = split.next().unwrap().parse().unwrap();
        (dir, val)
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
