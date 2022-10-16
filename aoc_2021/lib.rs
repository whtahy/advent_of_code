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
                .filter(|&(r, c)| {
                    grid.get(r).and_then(|row| row.get(c)).is_some()
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
    shared::input!(10);
    shared::test!(367_227, 3_583_341_858_u32); // examples: 26_397, 288_957

    use SyntaxError::*;

    enum SyntaxError {
        Corrupt(char),
        Incomplete(Vec<char>),
    }

    pub fn part1() -> String {
        INPUT
            .lines()
            .filter_map(|line| match check_syntax(line) {
                Corrupt(ch) => Some(ch),
                Incomplete(_) => None,
            })
            .map(|ch| match ch {
                ')' => 3,
                ']' => 57,
                '}' => 1_197,
                '>' => 25_137,
                _ => unreachable!(),
            })
            .sum::<usize>()
            .to_string()
    }

    pub fn part2() -> String {
        let mut scores = INPUT
            .lines()
            .filter_map(|line| match check_syntax(line) {
                Corrupt(_) => None,
                Incomplete(v) => Some(v),
            })
            .map(|v| {
                v.iter().rev().fold(0, |score, ch| {
                    score * 5
                        + match ch {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => unreachable!(),
                        }
                })
            })
            .collect::<Vec<_>>();
        scores.sort_unstable();
        scores[scores.len() / 2].to_string()
    }

    fn check_syntax(line: &str) -> SyntaxError {
        let open = |ch: char| match ch {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            '>' => '<',
            _ => unreachable!(),
        };

        let mut stack = Vec::new();
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                ')' | ']' | '}' | '>' => {
                    if stack.pop() != Some(open(ch)) {
                        return Corrupt(ch);
                    }
                }
                _ => unreachable!(),
            }
        }

        Incomplete(stack)
    }
}

pub mod day11 {
    shared::input!(11);
    shared::test!(1_669, 351); // examples: 1_656, 195

    pub fn part1() -> String {
        let mut grid = grid();
        let cartesian = (0..10).flat_map(|r| (0..10).map(move |c| (r, c)));

        let mut n_flashes = 0;
        for _ in 1..=100 {
            // step 1
            for (r, c) in cartesian.clone() {
                grid[r][c] += 1;
            }

            // step 2
            let mut todo: Vec<_> = cartesian.clone().collect();
            while !todo.is_empty() {
                let (r, c) = todo.pop().unwrap();
                if grid[r][c] > 9 {
                    n_flashes += 1;
                    grid[r][c] = 0;
                    for (rr, cc) in adjacent(r, c) {
                        if grid[rr][cc] > 0 {
                            grid[rr][cc] += 1;
                            todo.push((rr, cc));
                        }
                    }
                }
            }
        }

        n_flashes.to_string()
    }

    pub fn part2() -> String {
        let mut grid = grid();
        let cartesian = (0..10).flat_map(|r| (0..10).map(move |c| (r, c)));

        for i in 1.. {
            let mut n_flashes = 0;

            // step 1
            for (r, c) in cartesian.clone() {
                grid[r][c] += 1;
            }

            // step 2
            let mut todo: Vec<_> = cartesian.clone().collect();
            while !todo.is_empty() {
                let (r, c) = todo.pop().unwrap();
                if grid[r][c] > 9 {
                    n_flashes += 1;
                    grid[r][c] = 0;
                    for (rr, cc) in adjacent(r, c) {
                        if grid[rr][cc] > 0 {
                            grid[rr][cc] += 1;
                            todo.push((rr, cc));
                        }
                    }
                }
            }

            // return
            if n_flashes == 100 {
                return i.to_string();
            }
        }

        unreachable!()
    }

    fn adjacent(r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> {
        [
            (r - 1, c - 1),
            (r - 1, c),
            (r - 1, c + 1),
            (r, c - 1),
            (r, c + 1),
            (r + 1, c - 1),
            (r + 1, c),
            (r + 1, c + 1),
        ]
        .into_iter()
        .filter(|&(r, c)| r <= 9 && c <= 9)
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

pub mod day12 {
    shared::input!(12);
    shared::test!(4_573, 117_509); // examples: 10, 19, 226; 36, 103, 3_509

    use std::collections::HashMap;

    fn parse() -> (Vec<Vec<usize>>, Vec<bool>) {
        let mut cavern = vec![vec![], vec![]];
        let mut small_caves = vec![false, false];

        let mut caves = HashMap::from([("start", 0), ("end", 1)]);
        let mut counter = caves.len();
        INPUT
            .lines()
            .map(|line| line.split_once('-').unwrap())
            .flat_map(|(a, b)| [(a, b), (b, a)])
            .filter(|&(a, b)| a != "end" && b != "start")
            .for_each(|(a, b)| {
                let mut update = |s| {
                    if !caves.contains_key(s) {
                        caves.insert(s, counter);
                        cavern.push(Vec::new());
                        small_caves.push(s.chars().all(|ch| ch.is_lowercase()));
                        counter += 1;
                    }
                };
                update(a);
                update(b);
                cavern[caves[a]].push(caves[b]);
            });

        (cavern, small_caves)
    }

    struct Path {
        current: usize,
        past: Vec<bool>,
    }

    pub fn part1() -> String {
        let (cavern, small_caves) = parse();
        let mut n_paths = 0;
        let mut stack = vec![Path {
            current: 0,
            past: vec![false; cavern.len()],
        }];
        while !stack.is_empty() {
            let path = stack.pop().unwrap();
            for &cave in &cavern[path.current] {
                if cave == 1 {
                    n_paths += 1;
                    continue;
                } else if path.past[cave] {
                    continue;
                }
                let mut past = path.past.clone();
                past[path.current] = small_caves[path.current];
                stack.push(Path {
                    current: cave,
                    past,
                })
            }
        }

        n_paths.to_string()
    }

    pub fn part2() -> String {
        let (cavern, small_caves) = parse();
        let mut n_paths = 0;
        let mut stack = vec![(
            Path {
                current: 0,
                past: vec![false; cavern.len()],
            },
            false, // backtrack
        )];
        while !stack.is_empty() {
            let (path, backtrack) = stack.pop().unwrap();
            for &cave in &cavern[path.current] {
                let mut backtrack = backtrack;
                if cave == 1 {
                    n_paths += 1;
                    continue;
                } else if path.past[cave] {
                    if backtrack {
                        continue;
                    } else {
                        backtrack = true;
                    }
                }
                let mut past = path.past.clone();
                past[path.current] = small_caves[path.current];
                stack.push((
                    Path {
                        current: cave,
                        past,
                    },
                    backtrack,
                ))
            }
        }

        n_paths.to_string()
    }
}

pub mod day13 {
    shared::input!(13);
    shared::test!(621, 95); // examples: 17, 16 -> HKUJGAJZ

    use std::collections::{HashSet, VecDeque};

    type Dots = HashSet<(usize, usize)>;
    type Folds = VecDeque<(String, usize)>;

    fn parse() -> (Dots, Folds) {
        let mut dots = HashSet::new();
        let mut folds = VecDeque::new();
        for line in INPUT.lines() {
            if let Some((a, b)) = line.split_once(',') {
                dots.insert((a.parse().unwrap(), b.parse().unwrap()));
            } else if let Some((a, b)) = line.split_once('=') {
                let dir = match a {
                    "fold along y" => "y",
                    "fold along x" => "x",
                    _ => unreachable!(),
                };
                folds.push_back((dir.to_string(), b.parse().unwrap()));
            }
        }
        (dots, folds)
    }

    pub fn part1() -> String {
        let (dots, mut folds) = parse();
        let (dir, f) = folds.pop_front().unwrap();
        fold(dots, &dir, f).len().to_string()
    }

    pub fn part2() -> String {
        let (mut dots, folds) = parse();
        let (mut x_bound, mut y_bound) = (usize::MAX, usize::MAX);
        for (dir, f) in folds {
            dots = fold(dots, &dir, f);
            match dir.as_str() {
                "x" => x_bound = x_bound.min(f),
                "y" => y_bound = y_bound.min(f),
                _ => unreachable!(),
            }
        }

        for y in 0..y_bound {
            for x in 0..x_bound {
                if dots.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        dots.len().to_string()
    }

    fn fold(dots: Dots, dir: &str, fold: usize) -> Dots {
        dots.iter()
            .map(|&(x, y)| match (dir, x <= fold, y <= fold) {
                ("x", true, _) => (x, y),
                ("y", _, true) => (x, y),
                ("x", _, _) => (2 * fold - x, y),
                ("y", _, _) => (x, 2 * fold - y),
                _ => unreachable!(),
            })
            .collect()
    }
}

pub mod day14 {
    shared::input!(14);
    shared::test!(2_602, 2_942_885_922_173_u64); // 1_588, 2_188_189_693_529_u64

    use std::collections::HashMap;

    type T = u64;
    type Pair = [char; 2];
    type ElementCount = HashMap<char, T>;
    type PairCount = HashMap<Pair, T>;
    type Rules = HashMap<Pair, char>;

    fn parse() -> (ElementCount, PairCount, Rules) {
        let mut element_count = HashMap::new();
        let mut pair_count = HashMap::new();
        let mut rules = HashMap::new();
        let (template, r) = INPUT.split_once("\r\n\r\n").unwrap();

        for ln in r.lines() {
            let (k, v) = ln.split_once(" -> ").unwrap();
            let k = k.chars().collect::<Vec<_>>();
            let v = v.chars().next().unwrap();
            rules.insert([k[0], k[1]], v);
        }

        for w in template.chars().collect::<Vec<_>>().windows(2) {
            let w = [w[0], w[1]];
            *pair_count.entry(w).or_insert(0) += 1;
        }

        for ch in template.chars() {
            *element_count.entry(ch).or_insert(0) += 1;
        }

        (element_count, pair_count, rules)
    }

    pub fn part1() -> String {
        insert(10).to_string()
    }

    pub fn part2() -> String {
        insert(40).to_string()
    }

    fn insert(n: usize) -> T {
        let (mut element_count, mut pair_count, rules) = parse();
        for _ in 0..n {
            let mut new_pairs = HashMap::new();
            for p in pair_count.keys() {
                let elem = rules.get(p).unwrap();
                let n = *pair_count.get(p).unwrap();
                *new_pairs.entry([p[0], *elem]).or_insert(0) += n;
                *new_pairs.entry([*elem, p[1]]).or_insert(0) += n;
                *element_count.entry(*elem).or_insert(0) += n;
            }
            pair_count = new_pairs;
        }
        let min = element_count.values().min().unwrap();
        let max = element_count.values().max().unwrap();

        max - min
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
