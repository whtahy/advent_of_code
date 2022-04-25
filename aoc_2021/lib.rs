shared::table_of_contents!();

pub mod day1 {
    shared::input!(1);
    shared::test!(1_521, 1_543); // examples: 7, 5

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
    shared::test!(1_882_980, 1_971_232_560); // examples: 150, 900

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
    shared::test!(1_307_354, 482_500); // examples: 198, 230

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
    shared::test!(54_275, 13_158); // examples: 4_512, 1_924

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
    shared::input!(5);
    shared::test!(5_576, 18_144); // examples: 5, 12

    use std::collections::HashMap;

    type Point = (i32, i32);

    struct Vent {
        p1: Point,
        p2: Point,
        step: (i32, i32),
    }

    impl Vent {
        fn new(s: &str) -> Self {
            let (p1, p2) = s.split_once(" -> ").unwrap();
            let p1 = Self::parse(p1);
            let p2 = Self::parse(p2);
            let step = ((p2.0 - p1.0).signum(), (p2.1 - p1.1).signum());
            Vent { p1, p2, step }
        }

        fn parse(s: &str) -> Point {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        }
    }

    impl Iterator for Vent {
        type Item = Point;

        fn next(&mut self) -> Option<Point> {
            let (x1, y1) = self.p1;
            let (x2, y2) = self.p2;
            let (dx, dy) = self.step;
            if x1 == x2 + dx && y1 == y2 + dy {
                None
            } else {
                self.p1.0 += dx;
                self.p1.1 += dy;
                Some((x1, y1))
            }
        }
    }

    pub fn part1() -> String {
        INPUT
            .lines()
            .map(Vent::new)
            .filter(|v| v.p1.0 == v.p2.0 || v.p1.1 == v.p2.1)
            .flatten()
            .fold(HashMap::new(), |mut counts, p| {
                *counts.entry(p).or_insert(0) += 1;
                counts
            })
            .values()
            .filter(|&&x| x >= 2)
            .count()
            .to_string()
    }

    pub fn part2() -> String {
        let mut counts = HashMap::new();
        for p in INPUT.lines().flat_map(Vent::new) {
            *counts.entry(p).or_insert(0) += 1;
        }
        counts.values().filter(|&&x| x >= 2).count().to_string()
    }
}

pub mod day6 {
    shared::input!(6);
    shared::test!(380_612, 1_710_166_656_900_u64);
    // examples: 5_934, 26_984_457_539_u64

    type Counter = [u64; 9];

    pub fn part1() -> String {
        steps(80).to_string()
    }

    pub fn part2() -> String {
        steps(256).to_string()
    }

    fn parse() -> Counter {
        INPUT
            .split(',')
            .map(str::trim)
            .flat_map(str::parse::<usize>)
            .fold([0; 9], |mut counter, x| {
                counter[x] += 1;
                counter
            })
    }

    fn steps(n: usize) -> u64 {
        (1..=n)
            .fold(parse(), |mut c, _| {
                c.rotate_left(1);
                c[6] += c[8];
                c
            })
            .iter()
            .sum()
    }
}

pub mod day7 {
    shared::input!(7);
    shared::test!(344_297, 97_164_301); // examples: 37, 168

    pub fn part1() -> String {
        let mut input = parse();
        input.sort_unstable();
        let median = input[input.len() / 2];
        input
            .iter()
            .map(|x| (x - median).abs())
            .sum::<i32>()
            .to_string()
    }

    pub fn part2() -> String {
        let input = parse();
        let mean = input.iter().sum::<i32>() as f32 / input.len() as f32;
        let a = triangle_sum(mean.floor() as i32, &input);
        let b = triangle_sum(mean.ceil() as i32, &input);
        a.min(b).to_string()
    }

    fn triangle_sum(pos: i32, input: &[i32]) -> i32 {
        input
            .iter()
            .map(|x| {
                let n = (x - pos).abs();
                n * (n + 1) / 2
            })
            .sum()
    }

    fn parse() -> Vec<i32> {
        INPUT
            .split(',')
            .map(str::trim)
            .flat_map(str::parse)
            .collect()
    }
}

pub mod day8 {
    shared::input!(8);
    shared::test!(352, 936_117); // examples: 26, 61_229

    use std::collections::{HashMap, HashSet};

    pub fn part1() -> String {
        INPUT
            .lines()
            .flat_map(|x| x.split_once(" | "))
            .flat_map(|(_, x)| x.split_whitespace())
            .map(str::trim)
            .filter(|s| [2, 3, 4, 7].iter().any(|&x| x == s.len()))
            .count()
            .to_string()
    }

    pub fn part2() -> String {
        INPUT.lines().map(decode_line).sum::<u32>().to_string()
    }

    fn decode_line(s: &str) -> u32 {
        let (prefix, suffix) = s.split_once(" | ").unwrap();
        let set = |w: &str| HashSet::<_>::from_iter(w.chars());

        // prefix
        let mut code = HashMap::new();
        for word in prefix.split_whitespace() {
            match word.len() {
                2 => code.insert(1, set(word)),
                4 => code.insert(4, set(word)),
                _ => continue,
            };
        }

        // suffix
        let decode_word = |w: &str| {
            let intersect = |x| (&set(w) & &code[&x]).len();
            match (w.len(), intersect(1), intersect(4)) {
                (2, _, _) => '1',
                (3, _, _) => '7',
                (4, _, _) => '4',
                (7, _, _) => '8',
                (5, _, 2) => '2',
                (5, 2, _) => '3',
                (5, _, _) => '5',
                (6, 1, _) => '6',
                (6, _, 4) => '9',
                (6, _, _) => '0',
                _ => unreachable!(),
            }
        };

        suffix
            .split_whitespace()
            .map(decode_word)
            .collect::<String>()
            .parse()
            .unwrap()
    }
}

pub mod day9 {
    shared::input!(9);
    shared::test!(575, 1_019_700); // examples: 15, 1_134

    use std::collections::{BTreeSet, HashSet};

    pub fn part1() -> String {
        let grid = grid();
        let cartesian = grid.iter().enumerate().flat_map(|(r, row)| {
            row.iter().enumerate().map(move |(c, val)| (r, c, val))
        });

        let adjacent = |r: usize, c: usize| {
            [(1, 0), (0, 1)]
                .into_iter()
                .flat_map(move |(dy, dx)| [(r + dy, c + dx), (r - dy, c - dx)])
                .filter_map(|(r, c)| grid.get(r)?.get(c))
        };

        cartesian
            .filter(|&(r, c, val)| adjacent(r, c).all(|nbr| val < nbr))
            .map(|(_, _, val)| val + 1)
            .sum::<usize>()
            .to_string()
    }

    pub fn part2() -> String {
        let grid = grid();
        let cartesian = (0..grid.len())
            .flat_map(|r| (0..grid[r].len()).map(move |c| (r, c)));

        let adjacent = |r: usize, c: usize| {
            [(1, 0), (0, 1)]
                .into_iter()
                .flat_map(move |(dy, dx)| [(r + dy, c + dx), (r - dy, c - dx)])
                .filter(|(r, c)| {
                    grid.get(*r).and_then(|row| row.get(*c)).is_some()
                })
        };

        let low_points = cartesian.filter(|&(r, c)| {
            adjacent(r, c).all(|(rr, cc)| grid[r][c] < grid[rr][cc])
        });

        let mut basin_sizes = BTreeSet::new();
        let mut stack = Vec::new();
        for (r, c) in low_points {
            stack.push((r, c));
            let mut history = HashSet::new();
            while !stack.is_empty() {
                let cell = stack.pop().unwrap();
                for (rr, cc) in adjacent(cell.0, cell.1) {
                    if grid[rr][cc] != 9 && !history.contains(&(rr, cc)) {
                        stack.push((rr, cc));
                    }
                }
                history.insert(cell);
            }
            basin_sizes.insert(history.len());
        }

        basin_sizes
            .iter()
            .rev()
            .take(3)
            .product::<usize>()
            .to_string()
    }

    fn grid() -> Vec<Vec<usize>> {
        INPUT
            .lines()
            .map(|ln| {
                ln.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect()
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
