shared::table_of_contents!();

pub mod day1 {
    shared::input!(1);
    shared::test!(1_521, 1_543);

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
    shared::input!(2);
    shared::test!(1_882_980, 1_971_232_560);

    pub fn part1() -> String {
        let mut horizontal = 0;
        let mut depth = 0;
        for (dir, x) in INPUT.lines().map(parse) {
            match dir {
                "forward" => horizontal += x,
                "down" => depth += x,
                "up" => depth -= x,
                _ => unreachable!(),
            }
        }
        (horizontal * depth).to_string()
    }

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
                _ => unreachable!(),
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
    shared::input!(3);
    shared::test!(1_307_354, 482_500);

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
    shared::input!(4);
    shared::test!(54_275, 13_158);

    type Board = [[Option<u32>; 5]; 5];

    #[derive(Debug, Default)]
    struct Bingo {
        board: Board,
        sum: u32,
    }

    impl Bingo {
        fn new(s: &str) -> Bingo {
            s.split_whitespace().flat_map(str::parse).enumerate().fold(
                Bingo {
                    ..Default::default()
                },
                |mut game, (i, val)| {
                    let (r, c) = Bingo::index(i);
                    game.board[r][c] = Some(val);
                    game.sum += val;
                    game
                },
            )
        }

        fn mark(&mut self, n: u32) {
            if let Some(i) =
                self.board.iter().flatten().position(|&x| x == Some(n))
            {
                let (r, c) = Bingo::index(i);
                self.board[r][c] = None;
                self.sum -= n;
            };
        }

        fn victory(&self) -> bool {
            (0..5).any(|i| {
                (0..5).all(|j| self.board[i][j].is_none()) // row
                    || (0..5).all(|j| self.board[j][i].is_none()) // col
            })
        }

        fn index(i: usize) -> (usize, usize) {
            (i / 5, i % 5)
        }
    }

    pub fn part1() -> String {
        let (draws, mut games) = parse();
        for draw in draws {
            for game in games.iter_mut() {
                game.mark(draw);
                if game.victory() {
                    return (draw * game.sum).to_string();
                }
            }
        }
        unreachable!()
    }

    pub fn part2() -> String {
        let (draws, mut games) = parse();
        for draw in draws {
            for game in games.iter_mut() {
                game.mark(draw);
            }
            if games.len() == 1 && games[0].victory() {
                return (draw * games[0].sum).to_string();
            }
            games.retain(|g| !g.victory());
        }
        unreachable!()
    }

    fn parse() -> (impl Iterator<Item = u32>, Vec<Bingo>) {
        let mut s = INPUT.split("\r\n\r\n");
        let draws = s.next().unwrap().split(',').flat_map(str::parse);
        let games = s.map(Bingo::new).collect();
        (draws, games)
    }
}

pub mod day5 {
    use std::collections::HashSet;
    use std::iter::repeat;

    shared::input!(5);
    shared::test!(5_576, 18_144);

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Point {
        x: u32,
        y: u32,
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Line {
        p1: Point,
        p2: Point,
    }

    impl Point {
        fn new(s: &str) -> Point {
            let (x, y) = s.split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        }
    }

    impl Line {
        fn new(s: &str) -> Line {
            let (p1, p2) = s.split_once(" -> ").unwrap();
            let mut p1 = Point::new(p1);
            let mut p2 = Point::new(p2);
            if p1.x > p2.x {
                std::mem::swap(&mut p1, &mut p2);
            }
            Line { p1, p2 }
        }

        fn points(&self) -> Vec<Point> {
            let (x1, y1) = (self.p1.x, self.p1.y);
            let (x2, y2) = (self.p2.x, self.p2.y);
            let mut v = Vec::new();

            if x1 == x2 {
                for (x, y) in repeat(x1).zip(y1.min(y2)..=y1.max(y2)) {
                    v.push(Point { x, y });
                }
            } else if y1 == y2 {
                for (x, y) in (x1..=x2).zip(repeat(y1)) {
                    v.push(Point { x, y });
                }
            } else if y1 < y2 {
                for (x, y) in (x1..=x2).zip(y1..=y2) {
                    v.push(Point { x, y });
                }
            } else {
                for (x, y) in (x1..=x2).zip((y2..=y1).rev()) {
                    v.push(Point { x, y });
                }
            }
            v
        }

        fn is_not_diag(&self) -> bool {
            self.p1.x == self.p2.x || self.p1.y == self.p2.y
        }
    }

    pub fn part1() -> String {
        let mut points = HashSet::new();
        let mut duplicates = HashSet::new();
        let mut count = 0;
        for line in INPUT.lines().map(Line::new).filter(Line::is_not_diag) {
            for p in line.points() {
                if duplicates.contains(&p) {
                    continue;
                } else if points.contains(&p) {
                    duplicates.insert(p);
                    count += 1;
                } else {
                    points.insert(p);
                }
            }
        }
        count.to_string()
    }

    pub fn part2() -> String {
        let mut points = HashSet::new();
        let mut duplicates = HashSet::new();
        let mut count = 0;
        for line in INPUT.lines().map(Line::new) {
            for p in line.points() {
                if duplicates.contains(&p) {
                    continue;
                } else if points.contains(&p) {
                    duplicates.insert(p);
                    count += 1;
                } else {
                    points.insert(p);
                }
            }
        }
        count.to_string()
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
