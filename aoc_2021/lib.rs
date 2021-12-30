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
        INPUT
            .lines()
            .flat_map(|x| x.parse::<u32>())
            .collect::<Vec<_>>()
            .windows(2)
            .filter(|w| w[0] < w[1])
            .count()
            .to_string()
    }

    /// ```
    /// assert_eq!(aoc_2021::day1::part2(), 1_543.to_string());
    /// ```
    pub fn part2() -> String {
        INPUT
            .lines()
            .flat_map(|x| x.parse::<u32>())
            .collect::<Vec<_>>()
            .windows(4)
            .filter(|w| w[0] < w[3])
            .count()
            .to_string()
    }
}

pub mod day2 {
    input!(2);

    /// ```
    /// assert_eq!(aoc_2021::day2::part1(), 1_882_980.to_string());
    /// ```
    pub fn part1() -> String {
        let mut horizontal = 0;
        let mut depth = 0;
        for (dir, x) in INPUT.lines().map(parse) {
            match dir {
                "forward" => horizontal += x,
                "down" => depth += x,
                "up" => depth -= x,
                _ => panic!(),
            }
        }
        (horizontal * depth).to_string()
    }

    /// ```
    /// assert_eq!(aoc_2021::day2::part2(), 1_971_232_560.to_string());
    /// ```
    pub fn part2() -> String {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;
        for (dir, x) in INPUT.lines().map(parse) {
            match dir {
                "forward" => {
                    horizontal += x;
                    depth += aim * x;
                }
                "down" => aim += x,
                "up" => aim -= x,
                _ => panic!(),
            }
        }
        (horizontal * depth).to_string()
    }

    fn parse(line: &str) -> (&str, u32) {
        let (dir, val) = line.split_once(' ').unwrap();
        (dir, val.parse().unwrap())
    }
}

pub mod day3 {
    input!(3);

    /// ```
    /// assert_eq!(aoc_2021::day3::part1(), 1_307_354.to_string());
    /// ```
    pub fn part1() -> String {
        let mut counts = [[0, 0]; 12];

        for line in INPUT.lines() {
            for (i, ch) in line.chars().enumerate() {
                let x = ch.to_digit(10).unwrap() as usize;
                counts[i][x] += 1;
            }
        }

        let mut gamma = String::new();
        let mut epsilon = String::new();
        for x in counts {
            if x[0] > x[1] {
                gamma.push('0');
                epsilon.push('1');
            } else {
                gamma.push('1');
                epsilon.push('0');
            }
        }

        binary_product(&gamma, &epsilon).to_string()
    }

    /// ```
    /// assert_eq!(aoc_2021::day3::part2(), 482_500.to_string());
    /// ```
    pub fn part2() -> String {
        let (mut o2, mut co2) = parse(0, INPUT.lines().collect());
        for i in 1.. {
            o2 = parse(i, o2).0;
            if o2.len() == 1 {
                break;
            }
        }
        for i in 1.. {
            co2 = parse(i, co2).1;
            if co2.len() == 1 {
                break;
            }
        }
        binary_product(o2[0], co2[0]).to_string()
    }

    fn parse(i: usize, numbers: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
        let mut o2 = Vec::new();
        let mut co2 = Vec::new();
        for s in numbers {
            if s.chars().nth(i) == Some('1') {
                o2.push(s);
            } else {
                co2.push(s);
            }
        }
        if co2.len() > o2.len() {
            std::mem::swap(&mut co2, &mut o2);
        }
        (o2, co2)
    }

    fn binary_product(a: &str, b: &str) -> u32 {
        [a, b]
            .iter()
            .flat_map(|x| u32::from_str_radix(x, 2))
            .product()
    }
}

pub mod day4 {
    input!(4);

    type Board<T> = Vec<Vec<T>>;

    #[derive(Debug, Clone)]
    struct Bingo {
        board: Board<u8>,
        marks: Board<bool>,
    }

    impl Bingo {
        fn new(b: &str) -> Bingo {
            Bingo {
                board: parse(b),
                marks: vec![vec![false; 5]; 5],
            }
        }

        fn mark(&mut self, n: u8) -> Option<u32> {
            for (i, row) in self.board.iter().enumerate() {
                for (j, x) in row.iter().enumerate() {
                    if x == &n {
                        self.marks[i][j] = true;
                        if self.victory() {
                            return Some(n as u32 * self.score());
                        } else {
                            return None;
                        }
                    }
                }
            }
            None
        }

        fn victory(&self) -> bool {
            for row in self.marks.iter() {
                if row.iter().all(|&x| x) {
                    return true;
                }
            }
            for col in 0..=4 {
                if self.marks.iter().all(|row| row[col]) {
                    return true;
                }
            }
            false
        }

        fn score(&self) -> u32 {
            let mut sum = 0;
            for (i, row) in self.marks.iter().enumerate() {
                for (j, x) in row.iter().enumerate() {
                    if !x {
                        sum += self.board[i][j] as u32;
                    }
                }
            }
            sum
        }
    }

    fn parse(board: &str) -> Board<u8> {
        board
            .lines()
            .map(|line| line.split_whitespace().flat_map(str::parse).collect())
            .collect()
    }

    /// ```
    /// assert_eq!(aoc_2021::day4::part1(), 54_275.to_string());
    /// ```
    pub fn part1() -> String {
        let mut s = INPUT.split("\r\n\r\n");
        let draws = s.next().unwrap().split(',').flat_map(str::parse);
        let mut games: Vec<Bingo> = s.map(Bingo::new).collect();

        for draw in draws {
            for game in games.iter_mut() {
                match game.mark(draw) {
                    Some(score) => return score.to_string(),
                    None => continue,
                }
            }
        }

        panic!()
    }

    /// ```
    /// assert_eq!(aoc_2021::day4::part2(), 13_158.to_string());
    /// ```
    pub fn part2() -> String {
        let mut s = INPUT.split("\r\n\r\n");
        let draws = s.next().unwrap().split(',').flat_map(str::parse);
        let mut games: Vec<Bingo> = s.map(Bingo::new).collect();

        for draw in draws {
            if games.len() == 1 {
                match games[0].mark(draw) {
                    Some(score) => return score.to_string(),
                    None => continue,
                }
            }
            for game in games.iter_mut() {
                game.mark(draw);
            }
            games = games.into_iter().filter(|g| !g.victory()).collect();
        }

        panic!()
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
