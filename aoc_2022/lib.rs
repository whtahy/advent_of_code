shared::table_of_contents!();

pub mod day1 {
    use std::collections::BinaryHeap;

    shared::input!(1);
    shared::test!(72_602, 207_410);

    type T = usize;

    pub fn part1() -> String {
        INPUT
            .split("\r\n\r\n")
            .map(inventory_sum)
            .max()
            .unwrap()
            .to_string()
    }

    pub fn part2() -> String {
        INPUT
            .split("\r\n\r\n")
            .map(inventory_sum)
            .collect::<BinaryHeap<_>>()
            .iter()
            .take(3)
            .sum::<T>()
            .to_string()
    }

    fn inventory_sum(s: &str) -> T {
        s.lines().map(|ln| ln.parse::<T>().unwrap()).sum()
    }
}

pub mod day2 {
    shared::input!(2);
    shared::test!(10_624, 14_060);

    use crate::day2::{Outcome::*, Shape::*};

    type T = usize;

    enum Shape {
        Rock,
        Paper,
        Scissors,
    }

    enum Outcome {
        Win,
        Draw,
        Loss,
    }

    pub fn part1() -> String {
        let parse = |s: &str| {
            let (left, right) = s.split_once(' ').unwrap();
            let opponent = parse_left(left);
            let you = match right {
                "X" => Rock,
                "Y" => Paper,
                "Z" => Scissors,
                _ => unreachable!(),
            };
            let outcome = match (&you, opponent) {
                (Rock, Rock) => Draw,
                (Rock, Paper) => Loss,
                (Rock, Scissors) => Win,
                (Paper, Rock) => Win,
                (Paper, Paper) => Draw,
                (Paper, Scissors) => Loss,
                (Scissors, Rock) => Loss,
                (Scissors, Paper) => Win,
                (Scissors, Scissors) => Draw,
            };
            score(you, outcome)
        };
        INPUT.lines().map(parse).sum::<T>().to_string()
    }

    pub fn part2() -> String {
        let parse = |s: &str| {
            let (left, right) = s.split_once(' ').unwrap();
            let opponent = parse_left(left);
            let outcome = match right {
                "X" => Loss,
                "Y" => Draw,
                "Z" => Win,
                _ => unreachable!(),
            };
            let shape = match (&outcome, &opponent) {
                (Loss, Rock) => Scissors,
                (Loss, Paper) => Rock,
                (Loss, Scissors) => Paper,
                (Draw, _) => opponent,
                (Win, Rock) => Paper,
                (Win, Paper) => Scissors,
                (Win, Scissors) => Rock,
            };
            score(shape, outcome)
        };
        INPUT.lines().map(parse).sum::<T>().to_string()
    }

    fn score(shape: Shape, outcome: Outcome) -> T {
        (match shape {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }) + match outcome {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }

    fn parse_left(s: &str) -> Shape {
        match s {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => unreachable!(),
        }
    }
}

pub mod day3 {
    shared::input!(3);
    shared::test!(8_515, 2_434);

    type T = usize;
    type S = HashSet<T>;
    use std::collections::HashSet;

    pub fn part1() -> String {
        INPUT
            .lines()
            .map(|ln| {
                let (left, right) = ln.split_at(ln.len() / 2);
                (sack(left), sack(right))
            })
            .map(|(left, right)| *left.intersection(&right).next().unwrap())
            .sum::<T>()
            .to_string()
    }

    pub fn part2() -> String {
        INPUT
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunk| chunk.iter().map(|x| sack(x)))
            .map(|mut sacks| {
                let mut badge = sacks.next().unwrap();
                badge.retain(|x| sacks.clone().all(|other| other.contains(x)));
                *badge.iter().next().unwrap()
            })
            .sum::<T>()
            .to_string()
    }

    fn sack(s: &str) -> S {
        s.chars()
            .map(|ch| match ch.is_lowercase() {
                true => ch as T - 96,
                false => ch as T - 38,
            })
            .collect()
    }
}

pub mod day4 {
    shared::input!(4);
    shared::test!(477, 830);

    type T = usize;
    type Pair = ((T, T), (T, T));

    pub fn part1() -> String {
        INPUT
            .lines()
            .map(parse_pair)
            .filter(|(left, right)| {
                (left.0 >= right.0 && left.1 <= right.1)
                    || (right.0 >= left.0 && right.1 <= left.1)
            })
            .count()
            .to_string()
    }

    pub fn part2() -> String {
        INPUT
            .lines()
            .map(parse_pair)
            .filter(|(left, right)| {
                (left.0 <= right.0 && right.0 <= left.1)
                    || (right.0 <= left.0 && left.0 <= right.1)
            })
            .count()
            .to_string()
    }

    fn parse_pair(s: &str) -> Pair {
        let v = s
            .split(&[',', '-'])
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();
        ((v[0], v[1]), (v[2], v[3]))
    }
}

pub mod day5 {
    shared::input!(5);
    shared::test!("WCZTHTMPS", "BLSGJSDTS");

    type Stacks = Vec<Vec<char>>;
    type T = usize;
    struct Step {
        n: T,
        from: T,
        to: T,
    }

    fn parse(s: &str) -> (Stacks, Vec<Step>) {
        let (upper, lower) = s.split_once("\r\n\r\n").unwrap();
        let stacks = parse_crates(upper);
        let steps = lower.lines().map(parse_step).collect();
        (stacks, steps)
    }

    fn parse_crates(s: &str) -> Stacks {
        let n_stacks = s.lines().next().unwrap().len() / 4 + 1;
        let mut stacks = vec![Vec::new(); n_stacks];
        for ln in s.lines().rev() {
            for (i, ch) in ln
                .chars()
                .enumerate()
                .filter(|(_, ch)| ch.is_ascii_uppercase())
            {
                stacks[i / 4].push(ch);
            }
        }
        stacks
    }

    fn parse_step(ln: &str) -> Step {
        let v = ln.split(' ').flat_map(|x| x.parse()).collect::<Vec<_>>();
        Step {
            n: v[0],
            from: v[1] - 1,
            to: v[2] - 1,
        }
    }

    pub fn part1() -> String {
        let (mut stacks, steps) = parse(INPUT);
        for step in steps {
            for _ in 1..=step.n {
                let c = stacks[step.from].pop().unwrap();
                stacks[step.to].push(c);
            }
        }
        stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
    }

    pub fn part2() -> String {
        let (mut stacks, steps) = parse(INPUT);
        for step in steps {
            let from = &mut stacks[step.from];
            let c = from.split_off(from.len() - step.n);
            stacks[step.to].extend(c);
        }
        stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
    }
}

pub mod day6 {
    shared::input!(6);
    shared::test!(1_647, 2_447);

    use std::collections::HashSet;

    type T = usize;

    pub fn part1() -> String {
        find_marker(4).to_string()
    }

    pub fn part2() -> String {
        find_marker(14).to_string()
    }

    fn find_marker(len: T) -> T {
        len + INPUT
            .chars()
            .collect::<Vec<_>>()
            .windows(len)
            .take_while(|w| w.iter().collect::<HashSet<_>>().len() < len)
            .count()
    }
}

pub mod day7 {
    shared::input!(7);
    shared::test!(1_783_610, 4_370_655);

    use crate::day7::FsData::*;
    use std::{collections::HashMap, path::PathBuf};

    type T = usize;
    type FsTree = HashMap<PathBuf, Vec<FsData>>;

    #[derive(Debug)]
    enum FsData {
        File(T),
        Folder(PathBuf),
    }

    fn parse() -> FsTree {
        let mut fs_tree: FsTree = HashMap::new();
        let mut path = PathBuf::new();
        for cmd in INPUT.lines().map(|ln| ln.split(' ').collect::<Vec<_>>()) {
            match cmd[..] {
                ["$", "ls"] => continue,
                ["$", "cd", ".."] => {
                    path.pop();
                }
                ["$", "cd", s] => {
                    path.push(s);
                    fs_tree.entry(path.clone()).or_insert_with(Vec::new);
                }
                ["dir", s] => {
                    let mut subfolder = path.clone();
                    subfolder.push(s);
                    fs_tree.get_mut(&path).unwrap().push(Folder(subfolder));
                }
                [n, _] => {
                    let file = File(n.parse().unwrap());
                    fs_tree.get_mut(&path).unwrap().push(file);
                }
                _ => unreachable!(),
            }
        }
        fs_tree
    }

    pub fn part1() -> String {
        let fs_tree = parse();
        fs_tree
            .keys()
            .into_iter()
            .map(|k| size(k, &fs_tree))
            .filter(|&n| n < 100_000)
            .sum::<T>()
            .to_string()
    }

    pub fn part2() -> String {
        let fs_tree = parse();
        let mut sizes = fs_tree
            .keys()
            .into_iter()
            .map(|k| size(k, &fs_tree))
            .collect::<Vec<_>>();
        sizes.sort_unstable();
        let threshold = sizes.last().unwrap() - (70_000_000 - 30_000_000);
        sizes.iter().find(|&&x| x >= threshold).unwrap().to_string()
    }

    fn size(path: &PathBuf, fs_tree: &FsTree) -> T {
        fs_tree
            .get(path)
            .unwrap()
            .iter()
            .map(|x| match x {
                File(n) => *n,
                Folder(p) => size(p, fs_tree),
            })
            .sum()
    }
}

pub mod day8 {
    shared::input!(8);
    shared::test!(1_823, 211_680);

    type T = usize;
    type Trees = Vec<Vec<T>>;

    fn parse() -> Trees {
        INPUT
            .lines()
            .map(|ln| ln.chars().flat_map(|ch| ch.to_digit(10)).map(|x| x as T))
            .map(|iter| iter.collect())
            .collect()
    }

    pub fn part1() -> String {
        let trees = parse();
        let r_max = trees.len() - 1;
        let c_max = trees[0].len() - 1;
        let interior =
            cartesian(1, r_max - 1, 1, c_max - 1).filter(|&(r, c)| {
                let left = cartesian(r, r, 0, c - 1);
                let right = cartesian(r, r, c + 1, c_max);
                let up = cartesian(0, r - 1, c, c);
                let down = cartesian(r + 1, r_max, c, c);
                [left, right, up, down].into_iter().any(|mut iter| {
                    iter.all(|(rr, cc)| trees[r][c] > trees[rr][cc])
                })
            });
        (interior.count() + 2 * r_max + 2 * c_max).to_string()
    }

    pub fn part2() -> String {
        let trees = parse();
        let r_max = trees.len() - 1;
        let c_max = trees[0].len() - 1;
        let interior = cartesian(1, r_max - 1, 1, c_max - 1).map(|(r, c)| {
            let left = cartesian(r, r, 1, c - 1).rev();
            let right = cartesian(r, r, c + 1, c_max - 1);
            let up = cartesian(1, r - 1, c, c).rev();
            let down = cartesian(r + 1, r_max - 1, c, c);
            let a = [left, up].into_iter().map(|iter| {
                iter.take_while(|&(rr, cc)| trees[r][c] > trees[rr][cc])
                    .count()
            });
            let b = [right, down].into_iter().map(|iter| {
                iter.take_while(|&(rr, cc)| trees[r][c] > trees[rr][cc])
                    .count()
            });
            a.chain(b).map(|view| view + 1).product::<T>()
        });
        interior.max().unwrap().to_string()
    }

    fn cartesian(
        r1: T,
        r2: T,
        c1: T,
        c2: T,
    ) -> impl DoubleEndedIterator<Item = (T, T)> {
        (r1..=r2).flat_map(move |r| (c1..=c2).map(move |c| (r, c)))
    }
}

pub mod day9 {
    shared::input!(9);
    shared::test!(6_367, 2_536);

    use std::collections::HashSet;

    type T = i16;
    type Coord = (T, T);
    type Motion = (String, T);

    fn parse() -> impl Iterator<Item = Motion> {
        INPUT.lines().map(|ln| {
            let (dir, n) = ln.split_once(' ').unwrap();
            (dir.to_string(), n.parse().unwrap())
        })
    }

    pub fn part1() -> String {
        move_rope(parse(), 2).len().to_string()
    }

    pub fn part2() -> String {
        move_rope(parse(), 10).len().to_string()
    }

    fn move_rope(
        motions: impl Iterator<Item = Motion>,
        rope_length: usize,
    ) -> HashSet<Coord> {
        let mut rope = vec![(0, 0); rope_length];
        let mut visited = HashSet::from([rope[0]]);
        for (dir, n) in motions {
            match dir.as_str() {
                "R" => rope[0].0 += n,
                "L" => rope[0].0 -= n,
                "U" => rope[0].1 += n,
                "D" => rope[0].1 -= n,
                _ => unreachable!(),
            }
            for _ in 1..=n {
                follow(&mut rope);
                visited.insert(*rope.last().unwrap());
            }
        }
        visited
    }

    fn follow(rope: &mut Vec<Coord>) {
        for i in 1..rope.len() {
            if !is_adjacent(rope[i], rope[i - 1]) {
                rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
            }
        }
    }

    fn is_adjacent(a: Coord, b: Coord) -> bool {
        (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1
    }
}

pub mod day10 {
    shared::input!(10);
    shared::test!(
        13_760,
        "
        ###..####.#..#.####..##..###..####.####.
        #..#.#....#.#.....#.#..#.#..#.#....#....
        #..#.###..##.....#..#....#..#.###..###..
        ###..#....#.#...#...#....###..#....#....
        #.#..#....#.#..#....#..#.#....#....#....
        #..#.#....#..#.####..##..#....####.#...."
            .replace(' ', "")
    );

    use crate::day10::Instruction::*;

    type T = i32;

    #[derive(Debug)]
    enum Instruction {
        Noop,
        Addx(T),
    }

    fn parse() -> impl Iterator<Item = Instruction> {
        INPUT
            .lines()
            .map(|ln| ln.split(' ').collect::<Vec<_>>())
            .map(|v| match &v[..] {
                ["noop"] => Noop,
                ["addx", n] => Addx(n.parse().unwrap()),
                _ => unreachable!(),
            })
    }

    pub fn part1() -> String {
        let register = register();
        (20..=220)
            .step_by(40)
            .map(|x| x as T * register[x - 1])
            .sum::<T>()
            .to_string()
    }

    pub fn part2() -> String {
        let register = register();
        let (n_cols, n_pixels) = (40, 40 * 6);
        let mut image = String::with_capacity(n_pixels);
        for (i, x) in register.iter().enumerate().take(n_pixels) {
            let j = (i % n_cols) as T;
            if j == 0 {
                image.push('\n');
            }
            match x - 1 <= j && j <= x + 1 {
                true => image.push('#'),
                false => image.push('.'),
            }
        }
        image
    }

    fn register() -> Vec<T> {
        let mut history = vec![1];
        for instruction in parse() {
            let x = history.last().unwrap();
            match instruction {
                Noop => history.push(*x),
                Addx(n) => history.extend([*x, *x + n]),
            }
        }
        history
    }
}

pub mod day11 {
    shared::input!(11);
    shared::test!(50_616, 11_309_046_332_u64);

    type T = u64;
    type Troop = Vec<Monkey>;
    struct Monkey {
        items: Vec<T>,
        operation: Box<dyn Fn(T) -> T>,
        test: Box<dyn Fn(T) -> usize>,
        test_divisor: T,
    }

    impl Monkey {
        fn new(s: &str) -> Self {
            let v = s.lines().into_iter().collect::<Vec<_>>();
            let items = v[1]
                .replace("  Starting items: ", "")
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            let operation: Box<dyn Fn(T) -> T> =
                match v[2].rsplit(' ').take(2).collect::<Vec<_>>().as_slice() {
                    ["old", "*"] => Box::new(move |old| old * old),
                    [s, "*"] => {
                        let n = s.parse::<T>().unwrap();
                        Box::new(move |old| old * n)
                    }
                    [s, "+"] => {
                        let n = s.parse::<T>().unwrap();
                        Box::new(move |old| old + n)
                    }
                    _ => unreachable!(),
                };
            let last = |s: &str| s.rsplit_once(' ').unwrap().1.parse().unwrap();
            let test_divisor = last(v[3]);
            let (test_true, test_false) = (last(v[4]), last(v[5]));
            let test = Box::new(move |x| match x % test_divisor == 0 {
                true => test_true as usize,
                false => test_false as usize,
            });
            Monkey {
                items,
                operation,
                test,
                test_divisor,
            }
        }
    }

    fn parse() -> Troop {
        INPUT.split("\r\n\r\n").map(Monkey::new).collect()
    }

    pub fn part1() -> String {
        let mut troop = parse();
        let mut counts = vec![0; troop.len()];
        for _ in 1..=20 {
            for i in 0..troop.len() {
                while let Some(mut item) = troop[i].items.pop() {
                    item = (troop[i].operation)(item) / 3;
                    counts[i] += 1;
                    let new = (troop[i].test)(item);
                    troop[new].items.push(item);
                }
            }
        }
        counts.sort_unstable();
        counts.iter().rev().take(2).product::<T>().to_string()
    }

    pub fn part2() -> String {
        let mut troop = parse();
        let mut counts = vec![0; troop.len()];
        let worry: T = troop.iter().map(|monkey| monkey.test_divisor).product();
        for _ in 1..=10_000 {
            for m in 0..troop.len() {
                while let Some(mut item) = troop[m].items.pop() {
                    item = (troop[m].operation)(item) % worry;
                    counts[m] += 1;
                    let new = (troop[m].test)(item);
                    troop[new].items.push(item);
                }
            }
        }
        counts.sort_unstable();
        counts.iter().rev().take(2).product::<T>().to_string()
    }
}

pub mod day12 {
    shared::input!(12);
    shared::test!(383, 377);

    use crate::day12::Goal::*;
    use std::collections::VecDeque;

    type T = usize;
    type Coord = (T, T);
    type Heightmap = Vec<Vec<T>>;

    enum Goal {
        Letter(char),
        Square(Coord),
    }

    fn parse() -> (Heightmap, Coord, Coord) {
        let (mut heightmap, mut start, mut end) = (Vec::new(), (0, 0), (0, 0));
        for (r, ln) in INPUT.lines().enumerate() {
            heightmap.push(Vec::new());
            for (c, mut ch) in ln.chars().enumerate() {
                if ch == 'S' {
                    start = (r, c);
                    ch = 'a';
                } else if ch == 'E' {
                    end = (r, c);
                    ch = 'z';
                }
                heightmap[r].push(ch as T);
            }
        }
        (heightmap, start, end)
    }

    pub fn part1() -> String {
        let (height, start, end) = parse();
        dfs(&height, start, Square(end), |a, b| a + 1 >= b).to_string()
    }

    pub fn part2() -> String {
        let (height, _, end) = parse();
        dfs(&height, end, Letter('a'), |a, b| b + 1 >= a).to_string()
    }

    fn dfs(
        height: &Heightmap,
        start: Coord,
        goal: Goal,
        valid: fn(T, T) -> bool,
    ) -> T {
        let mut queue = VecDeque::from([(start.0, start.1, 0)]);
        let (n_rows, n_cols) = (height.len(), height[0].len());
        let mut history = vec![vec![false; n_cols]; n_rows];
        let adj = |r, c| {
            [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                .into_iter()
                .filter(|&(rr, cc)| rr < n_rows && cc < n_cols)
        };
        while let Some((r, c, n)) = queue.pop_front() {
            if match goal {
                Square(sq) => (r, c) == sq,
                Letter(ch) => height[r][c] == ch as T,
            } {
                return n;
            }
            for (rr, cc) in adj(r, c) {
                if history[rr][cc] || !valid(height[r][c], height[rr][cc]) {
                    continue;
                }
                queue.push_back((rr, cc, n + 1));
                history[rr][cc] = true;
            }
        }
        unreachable!()
    }
}

pub mod day13 {
    shared::input!(13);
    shared::test!(6_484, 19_305);

    use crate::day13::Data::*;
    use std::{cmp::Ordering, slice::from_ref};

    const TEN: char = char::REPLACEMENT_CHARACTER;

    type T = u8;

    #[derive(Debug, Clone, Eq, PartialEq)]
    enum Data {
        Int(T),
        List(Vec<Data>),
    }

    impl Ord for Data {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (Int(x), Int(y)) => x.cmp(y),
                (List(v), List(w)) => v.cmp(w),
                (Int(_), List(v)) => from_ref(self).cmp(v.as_slice()),
                (List(v), Int(_)) => v.as_slice().cmp(from_ref(other)),
            }
        }
    }

    impl PartialOrd for Data {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    fn parse() -> Vec<Data> {
        INPUT
            .lines()
            .filter(|ln| !ln.is_empty())
            .map(|ln| ln.replace("10", &TEN.to_string()))
            .map(|ln| parse_data(ln.chars()))
            .collect()
    }

    fn parse_data(tokens: impl Iterator<Item = char>) -> Data {
        let (mut data, mut subdata) = (Vec::new(), Vec::new());
        let mut depth = 0;
        for ch in tokens {
            match ch {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => continue,
                _ => (),
            }
            match (depth, ch) {
                (1, '[') => continue,
                (1, ']') => {
                    data.push(parse_data(subdata.into_iter()));
                    subdata = Vec::new();
                }
                (1, TEN) => data.push(Int(10)),
                (1, _) => data.push(Int(ch.to_digit(10).unwrap() as T)),
                _ => subdata.push(ch),
            }
        }
        List(data)
    }

    pub fn part1() -> String {
        parse()
            .chunks(2)
            .enumerate()
            .filter_map(|(i, chunk)| match chunk[0] < chunk[1] {
                true => Some(i + 1),
                _ => None,
            })
            .sum::<usize>()
            .to_string()
    }

    pub fn part2() -> String {
        let signal = parse();
        let [key_a, key_b] = ["[[2]]", "[[6]]"].map(|k| parse_data(k.chars()));
        let index = |key| signal.iter().filter(|&p| p < key).count() + 1;
        (index(&key_a) * (index(&key_b) + 1)).to_string()
    }
}

pub mod day14 {
    shared::input!(14);
    shared::test!(614, 26_170);

    use crate::day14::Tile::*;

    const COLUMN_OFFSET: T = 500;

    type T = usize;
    type Cave = Vec<Vec<Tile>>;

    #[derive(Clone, PartialEq, Eq)]
    enum Tile {
        Empty,
        Rock,
        Sand,
    }

    impl std::fmt::Display for Tile {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Empty => write!(f, "."),
                Rock => write!(f, "#"),
                Sand => write!(f, "o"),
            }
        }
    }

    fn parse() -> (Cave, T) {
        let mut r_max = T::MIN;
        let parse_line = |ln: &str| {
            ln.split([',', ' '])
                .flat_map(|s| s.parse::<T>())
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|chunk| {
                    r_max = r_max.max(chunk[1]);
                    (chunk[1], chunk[0])
                })
                .collect()
        };
        let rocks = INPUT.lines().map(parse_line).collect::<Vec<Vec<_>>>();
        let n_rows = r_max + 2;
        let n_cols = n_rows * 2 + 1;
        let mut cave = vec![vec![Empty; n_cols]; n_rows];
        for ln in rocks.iter() {
            for w in ln.windows(2) {
                for (r, c) in cartesian(w[0].0, w[1].0, w[0].1, w[1].1) {
                    cave[r][c - COLUMN_OFFSET + n_cols / 2] = Rock;
                }
            }
        }
        (cave, n_cols / 2)
    }

    pub fn part1() -> String {
        let (cave, start_col) = parse();
        simulate(cave, start_col).to_string()
    }

    pub fn part2() -> String {
        let (mut cave, start_col) = parse();
        cave.push(vec![Rock; cave[0].len()]);
        simulate(cave, start_col).to_string()
    }

    fn simulate(mut cave: Cave, start_col: T) -> T {
        let start = (0, start_col);
        let (mut r, mut c) = start;
        let mut ans = 0;
        loop {
            if r >= cave.len() - 1 {
                break; // part 1
            } else if cave[r + 1][c] == Empty {
                cave[r][c] = Empty;
                r += 1;
            } else if cave[r + 1][c - 1] == Empty {
                cave[r][c] = Empty;
                r += 1;
                c -= 1;
            } else if cave[r + 1][c + 1] == Empty {
                cave[r][c] = Empty;
                r += 1;
                c += 1;
            } else if (r, c) == start {
                ans += 1;
                break; // part 2
            } else {
                (r, c) = start;
                ans += 1;
            }
            cave[r][c] = Sand;
            print_cave(&cave);
        }
        ans
    }

    fn cartesian(r1: T, r2: T, c1: T, c2: T) -> impl Iterator<Item = (T, T)> {
        let (r1, r2) = (r1.min(r2), r1.max(r2));
        let (c1, c2) = (c1.min(c2), c1.max(c2));
        (r1..=r2).flat_map(move |r| (c1..=c2).map(move |c| (r, c)))
    }

    fn print_cave(cave: &Cave) {
        if cave.len() > 20 {
            return;
        }
        print!("{}c", 27 as char);
        for row in cave.iter() {
            for item in row.iter() {
                print!("{item}");
            }
            println!();
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

pub mod day15 {
    shared::input!(15);
    shared::test!(5_299_855, 13_615_843_289_729_i64);

    use std::collections::HashSet;

    type T = i64;
    type Coord = (T, T);

    #[derive(Debug)]
    struct Data {
        sensor: Coord,
        beacon: Coord,
        radius: T,
    }

    fn parse() -> Vec<Data> {
        INPUT
            .lines()
            .map(|ln| ln.split(['=', ',', ':']).flat_map(str::parse).collect())
            .map(|v: Vec<_>| {
                let sensor = (v[0], v[1]);
                let beacon = (v[2], v[3]);
                let radius = manhattan(&sensor, &beacon);
                Data {
                    sensor,
                    beacon,
                    radius,
                }
            })
            .collect()
    }

    pub fn part1() -> String {
        let data = parse();
        let row = 2_000_000;
        let n_beacons = data
            .iter()
            .filter(|d| d.beacon.1 == row)
            .map(|d| d.beacon.0)
            .collect::<HashSet<_>>()
            .len() as T;
        let (start, end) = data.iter().fold((T::MAX, T::MIN), |acc, d| {
            let delta_y = (d.sensor.1 - row).abs();
            let delta_x = (d.radius - delta_y).max(0);
            let start = acc.0.min(d.sensor.0 - delta_x);
            let end = acc.1.max(d.sensor.0 + delta_x);
            (start, end)
        });
        (end - start + 1 - n_beacons).to_string()
    }

    pub fn part2() -> String {
        let data = parse();
        let (min, max) = (0, 4_000_000);
        let mut stack = vec![(min, max, min, max)];
        while !stack.is_empty() {
            let (x1, x2, y1, y2) = stack.pop().unwrap();
            if x1 > x2 || y1 > y2 {
                continue;
            }
            let corners = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];
            if !data.iter().all(|d| {
                corners.iter().any(|c| manhattan(&d.sensor, c) > d.radius)
            }) {
                continue;
            };
            if x1 == x2 && y1 == y2 {
                return (x1 * max + y1).to_string();
            }
            let mid_x = (x1 + x2) / 2;
            let mid_y = (y1 + y2) / 2;
            stack.extend([
                (x1, mid_x, y1, mid_y),
                (x1, mid_x, mid_y + 1, y2),
                (mid_x + 1, x2, y1, mid_y),
                (mid_x + 1, x2, mid_y + 1, y2),
            ]);
        }
        unreachable!()
    }

    fn manhattan(p1: &Coord, p2: &Coord) -> T {
        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
    }
}

pub mod day16 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day17 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day18 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day19 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day20 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day21 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day22 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day23 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day24 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day25 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}
