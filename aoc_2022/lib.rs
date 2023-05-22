shared::table_of_contents!();

pub mod day1 {
    shared::puzzle!(1, 72_602, 207_410);
    shared::example!(1, 24_000, 45_000);

    use std::collections::BinaryHeap;

    type T = usize;

    pub fn part1(puzzle_input: &str) -> String {
        puzzle_input
            .split("\r\n\r\n")
            .map(inventory_sum)
            .max()
            .unwrap()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        puzzle_input
            .split("\r\n\r\n")
            .map(inventory_sum)
            .collect::<BinaryHeap<_>>()
            .iter()
            .take(3)
            .sum::<T>()
            .to_string()
    }

    fn inventory_sum(s: &str) -> T {
        s.lines().flat_map(str::parse::<T>).sum()
    }
}

pub mod day2 {
    shared::puzzle!(2, 10_624, 14_060);
    shared::example!(2, 15, 12);

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

    pub fn part1(puzzle_input: &str) -> String {
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
        puzzle_input.lines().map(parse).sum::<T>().to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
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
        puzzle_input.lines().map(parse).sum::<T>().to_string()
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
    shared::puzzle!(3, 8_515, 2_434);
    shared::example!(3, 157, 70);

    type T = usize;
    type S = HashSet<T>;
    use std::collections::HashSet;

    pub fn part1(puzzle_input: &str) -> String {
        puzzle_input
            .lines()
            .map(|ln| {
                let (left, right) = ln.split_at(ln.len() / 2);
                (sack(left), sack(right))
            })
            .map(|(left, right)| *left.intersection(&right).next().unwrap())
            .sum::<T>()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        puzzle_input
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
    shared::puzzle!(4, 477, 830);
    shared::example!(4, 2, 4);

    type T = usize;
    type Pair = ((T, T), (T, T));

    pub fn part1(puzzle_input: &str) -> String {
        puzzle_input
            .lines()
            .map(parse_pair)
            .filter(|(left, right)| {
                (left.0 >= right.0 && left.1 <= right.1)
                    || (right.0 >= left.0 && right.1 <= left.1)
            })
            .count()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        puzzle_input
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
            .flat_map(str::parse)
            .collect::<Vec<_>>();
        ((v[0], v[1]), (v[2], v[3]))
    }
}

pub mod day5 {
    shared::puzzle!(5, "WCZTHTMPS", "BLSGJSDTS");
    shared::example!(5, "CMZ", "MCD");

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
        let v = ln.split(' ').flat_map(str::parse).collect::<Vec<_>>();
        Step {
            n: v[0],
            from: v[1] - 1,
            to: v[2] - 1,
        }
    }

    pub fn part1(puzzle_input: &str) -> String {
        let (mut stacks, steps) = parse(puzzle_input);
        for step in steps {
            for _ in 1..=step.n {
                let c = stacks[step.from].pop().unwrap();
                stacks[step.to].push(c);
            }
        }
        stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let (mut stacks, steps) = parse(puzzle_input);
        for step in steps {
            let from = &mut stacks[step.from];
            let c = from.split_off(from.len() - step.n);
            stacks[step.to].extend(c);
        }
        stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
    }
}

pub mod day6 {
    shared::puzzle!(6, 1_647, 2_447);
    shared::example!(
        6,
        a => (7, 19),
        b => (5, 23),
        c => (6, 23),
        d => (10, 29),
        e => (11, 26),
    );

    use std::collections::HashSet;

    type T = usize;

    pub fn part1(puzzle_input: &str) -> String {
        find_marker(puzzle_input, 4).to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        find_marker(puzzle_input, 14).to_string()
    }

    fn find_marker(s: &str, len: T) -> T {
        len + s
            .chars()
            .collect::<Vec<_>>()
            .windows(len)
            .take_while(|w| w.iter().collect::<HashSet<_>>().len() < len)
            .count()
    }
}

pub mod day7 {
    shared::puzzle!(7, 1_783_610, 4_370_655);
    shared::example!(7, 95_437, 24_933_642);

    use crate::day7::FsData::*;
    use std::{collections::HashMap, path::PathBuf};

    type T = usize;
    type FsTree = HashMap<PathBuf, Vec<FsData>>;

    #[derive(Debug)]
    enum FsData {
        File(T),
        Folder(PathBuf),
    }

    fn parse(s: &str) -> FsTree {
        let mut fs_tree: FsTree = HashMap::new();
        let mut path = PathBuf::new();
        for cmd in s.lines().map(|ln| ln.split(' ').collect::<Vec<_>>()) {
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

    pub fn part1(puzzle_input: &str) -> String {
        let fs_tree = parse(puzzle_input);
        fs_tree
            .keys()
            .map(|k| size(k, &fs_tree))
            .filter(|&n| n < 100_000)
            .sum::<T>()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let fs_tree = parse(puzzle_input);
        let mut sizes = fs_tree
            .keys()
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
    shared::puzzle!(8, 1_823, 211_680);
    shared::example!(8, 21, 8);

    type T = usize;
    type Trees = Vec<Vec<T>>;

    fn parse(s: &str) -> Trees {
        s.lines()
            .map(|ln| ln.chars().flat_map(|ch| ch.to_digit(10)).map(|x| x as T))
            .map(|iter| iter.collect())
            .collect()
    }

    pub fn part1(puzzle_input: &str) -> String {
        let trees = parse(puzzle_input);
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

    pub fn part2(puzzle_input: &str) -> String {
        let trees = parse(puzzle_input);
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
    shared::puzzle!(9, 6_367, 2_536);
    shared::example!(
        9,
        a => (13, 1),
        b => (88, 36),
    );

    use std::collections::HashSet;

    type T = i16;
    type Coord = (T, T);
    type Motion = (String, T);

    fn parse(s: &str) -> impl Iterator<Item = Motion> + '_ {
        s.lines().map(|ln| {
            let (dir, n) = ln.split_once(' ').unwrap();
            (dir.to_string(), n.parse().unwrap())
        })
    }

    pub fn part1(puzzle_input: &str) -> String {
        move_rope(parse(puzzle_input), 2).len().to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        move_rope(parse(puzzle_input), 10).len().to_string()
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
    shared::puzzle!(
        10,
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
    shared::example!(
        10,
        13_140,
        "
        ##..##..##..##..##..##..##..##..##..##..
        ###...###...###...###...###...###...###.
        ####....####....####....####....####....
        #####.....#####.....#####.....#####.....
        ######......######......######......####
        #######.......#######.......#######....."
            .replace(' ', "")
    );

    use crate::day10::Instruction::*;

    type T = i32;

    #[derive(Debug)]
    enum Instruction {
        Noop,
        Addx(T),
    }

    fn parse(s: &str) -> impl Iterator<Item = Instruction> + '_ {
        s.lines().map(|ln| ln.split(' ').collect::<Vec<_>>()).map(
            |v| match &v[..] {
                ["noop"] => Noop,
                ["addx", n] => Addx(n.parse().unwrap()),
                _ => unreachable!(),
            },
        )
    }

    pub fn part1(puzzle_input: &str) -> String {
        let register = register(puzzle_input);
        (20..=220)
            .step_by(40)
            .map(|x| x as T * register[x - 1])
            .sum::<T>()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let register = register(puzzle_input);
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

    fn register(s: &str) -> Vec<T> {
        let mut history = vec![1];
        for instruction in parse(s) {
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
    shared::puzzle!(11, 50_616, 11_309_046_332_u64);
    shared::example!(11, 10_605, 2_713_310_158_u64);

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
            let v = s.lines().collect::<Vec<_>>();
            let items = v[1]
                .replace("  Starting items: ", "")
                .split(", ")
                .flat_map(str::parse)
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

    fn parse(s: &str) -> Troop {
        s.split("\r\n\r\n").map(Monkey::new).collect()
    }

    pub fn part1(puzzle_input: &str) -> String {
        let mut troop = parse(puzzle_input);
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

    pub fn part2(puzzle_input: &str) -> String {
        let mut troop = parse(puzzle_input);
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
    shared::puzzle!(12, 383, 377);
    shared::example!(12, 31, 29);

    use crate::day12::Goal::*;
    use std::collections::VecDeque;

    type T = usize;
    type Coord = (T, T);
    type Heightmap = Vec<Vec<T>>;

    enum Goal {
        Letter(char),
        Square(Coord),
    }

    fn parse(s: &str) -> (Heightmap, Coord, Coord) {
        let (mut heightmap, mut start, mut end) = (Vec::new(), (0, 0), (0, 0));
        for (r, ln) in s.lines().enumerate() {
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

    pub fn part1(puzzle_input: &str) -> String {
        let (height, start, end) = parse(puzzle_input);
        dfs(&height, start, Square(end), |a, b| a + 1 >= b).to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let (height, _, end) = parse(puzzle_input);
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
    shared::puzzle!(13, 6_484, 19_305);
    shared::example!(13, 13, 140);

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

    fn parse(s: &str) -> Vec<Data> {
        s.lines()
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

    pub fn part1(puzzle_input: &str) -> String {
        parse(puzzle_input)
            .chunks(2)
            .enumerate()
            .filter_map(|(i, chunk)| match chunk[0] < chunk[1] {
                true => Some(i + 1),
                _ => None,
            })
            .sum::<usize>()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let signal = parse(puzzle_input);
        let [key_a, key_b] = ["[[2]]", "[[6]]"].map(|k| parse_data(k.chars()));
        let index = |key| signal.iter().filter(|&p| p < key).count() + 1;
        (index(&key_a) * (index(&key_b) + 1)).to_string()
    }
}

pub mod day14 {
    shared::puzzle!(14, 614, 26_170);
    shared::example!(14, 24, 93);

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

    fn parse(s: &str) -> (Cave, T) {
        let mut r_max = T::MIN;
        let parse_line = |ln: &str| {
            ln.split([',', ' '])
                .flat_map(str::parse)
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|chunk| {
                    r_max = r_max.max(chunk[1]);
                    (chunk[1], chunk[0])
                })
                .collect()
        };
        let rocks = s.lines().map(parse_line).collect::<Vec<Vec<_>>>();
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

    pub fn part1(puzzle_input: &str) -> String {
        let (cave, start_col) = parse(puzzle_input);
        simulate(cave, start_col).to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let (mut cave, start_col) = parse(puzzle_input);
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
        }
        ans
    }

    fn cartesian(r1: T, r2: T, c1: T, c2: T) -> impl Iterator<Item = (T, T)> {
        let (r1, r2) = (r1.min(r2), r1.max(r2));
        let (c1, c2) = (c1.min(c2), c1.max(c2));
        (r1..=r2).flat_map(move |r| (c1..=c2).map(move |c| (r, c)))
    }

    fn _print_cave(cave: &Cave) {
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
    shared::puzzle!(15, 5_299_855, 13_615_843_289_729_i64);
    shared::example!(15, 26, 56_000_011);

    use std::collections::HashSet;

    type T = i64;
    type Coord = (T, T);

    #[derive(Debug)]
    struct Data {
        sensor: Coord,
        beacon: Coord,
        radius: T,
    }

    fn parse(s: &str) -> Vec<Data> {
        s.lines()
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

    pub fn part1(puzzle_input: &str) -> String {
        let data = parse(puzzle_input);
        let row = if data.len() == 14 { 10 } else { 2_000_000 };
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

    pub fn part2(puzzle_input: &str) -> String {
        let data = parse(puzzle_input);
        let min = 0;
        let max = if data.len() == 14 { 20 } else { 4_000_000 };
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
                return (x1 * 4_000_000 + y1).to_string();
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
    shared::puzzle!(16, 2_114, 2_666);
    shared::example!(16, 1_651, 1_707);

    use std::collections::{BinaryHeap, HashMap};

    type T = usize;

    type Distances = Vec<Vec<T>>;
    type ValveIds<'a> = HashMap<&'a str, T>;
    type FlowRates = Vec<T>;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct Path {
        flow: T,
        time: T,
        valve: T,
        todos: Vec<bool>,
    }

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct DuoPath {
        flow: T,
        time: [T; 2],
        valve: [T; 2],
        todos: Vec<bool>,
    }

    fn parse_line(line: &str) -> (&str, usize, impl Iterator<Item = &str>) {
        let mut words =
            line.split([' ', '=', ';', ',']).filter(|x| !x.is_empty());
        let valve_name = words.nth(1).unwrap();
        let flow_rate = words.nth(3).unwrap().parse().unwrap();
        let tunnels = words.skip(4);
        (valve_name, flow_rate, tunnels)
    }

    fn parse(s: &str) -> (ValveIds, FlowRates, Distances) {
        let mut valve_ids = HashMap::new();
        let mut flow_rates = Vec::new();
        let mut empty_valves = Vec::new();
        for ln in s.lines() {
            let (valve_name, flow_rate, _) = parse_line(ln);
            if flow_rate > 0 || valve_name == "AA" {
                flow_rates.push(flow_rate);
                valve_ids.insert(valve_name, flow_rates.len() - 1);
            } else {
                empty_valves.push(valve_name);
            }
        }

        // truncate empty valves after floyd warshall
        for (i, valve) in empty_valves.iter().enumerate() {
            valve_ids.insert(valve, flow_rates.len() + i);
        }

        // beware overflow
        let infinity = flow_rates.iter().sum::<T>() + 1;
        let mut dist = vec![vec![infinity; valve_ids.len()]; valve_ids.len()];
        for ln in s.lines() {
            let (valve_name, _, tunnels) = parse_line(ln);
            let i = *valve_ids.get(valve_name).unwrap();
            dist[i][i] = 0;
            for v in tunnels {
                let j = *valve_ids.get(v).unwrap();
                dist[i][j] = 1;
                dist[j][i] = 1;
            }
        }

        // floyd warshall
        let n = dist.len();
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }

        // remove empty valves
        dist.truncate(flow_rates.len());
        for row in dist.iter_mut() {
            row.truncate(flow_rates.len());
        }

        (valve_ids, flow_rates, dist)
    }

    pub fn part1(puzzle_input: &str) -> String {
        let (valve_ids, flow_rates, distances) = parse(puzzle_input);
        let start = Path {
            flow: 0,
            time: 30,
            valve: *valve_ids.get("AA").unwrap(),
            todos: flow_rates.iter().map(|&flow| flow != 0).collect(),
        };
        let mut best = 0;
        let mut pqueue = BinaryHeap::from([start]);
        while let Some(path) = pqueue.pop() {
            for next in (0..flow_rates.len()).filter(|&v| path.todos[v]) {
                let delta_t = distances[path.valve][next] + 1;
                if delta_t > path.time {
                    continue;
                }
                let new_time = path.time - delta_t;
                let new_flow = path.flow + new_time * flow_rates[next];
                best = best.max(new_flow);
                let mut new_todos = path.todos.clone();
                new_todos[next] = false;
                let upper_bound = flow_rates
                    .iter()
                    .enumerate()
                    .filter(|&(v, _)| new_todos[v])
                    .map(|(v, flow)| {
                        flow * new_time.saturating_sub(distances[next][v] + 1)
                    })
                    .sum::<T>()
                    + new_flow;
                if upper_bound <= best {
                    continue;
                }
                pqueue.push(Path {
                    flow: new_flow,
                    time: new_time,
                    valve: next,
                    todos: new_todos,
                });
            }
        }
        best.to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let (valve_ids, flow_rates, distances) = parse(puzzle_input);
        let valve_aa = *valve_ids.get("AA").unwrap();
        let start = DuoPath {
            flow: 0,
            time: [26; 2],
            valve: [valve_aa; 2],
            todos: flow_rates.iter().map(|&flow| flow != 0).collect(),
        };
        let mut best = 0;
        let mut pqueue = BinaryHeap::from([start]);
        while let Some(mut path) = pqueue.pop() {
            for next in (0..flow_rates.len()).filter(|&v| path.todos[v]) {
                if path.time[0] < path.time[1] {
                    path.time.swap(0, 1);
                    path.valve.swap(0, 1);
                }
                let delta_t = distances[path.valve[0]][next] + 1;
                if delta_t > path.time[0] {
                    continue;
                }
                let new_time = path.time[0] - delta_t;
                let new_flow = path.flow + new_time * flow_rates[next];
                best = best.max(new_flow);
                let mut new_todos = path.todos.clone();
                new_todos[next] = false;
                let upper_bound = flow_rates
                    .iter()
                    .enumerate()
                    .filter(|&(v, _)| new_todos[v])
                    .map(|(v, flow)| {
                        let time = new_time.max(path.time[1]);
                        let distance =
                            distances[next][v].min(distances[path.valve[1]][v]);
                        flow * time.saturating_sub(distance + 1)
                    })
                    .sum::<T>()
                    + new_flow;
                if upper_bound <= best {
                    continue;
                }
                pqueue.push(DuoPath {
                    flow: new_flow,
                    time: [new_time, path.time[1]],
                    valve: [next, path.valve[1]],
                    todos: new_todos,
                });
            }
        }
        best.to_string()
    }
}

pub mod day17 {
    shared::puzzle!(17, 3_239, 1_594_842_406_882_u64);
    shared::example!(17, 3_068, 1_514_285_714_288_u64);

    use std::collections::{HashMap, HashSet};

    const CAVERN_WIDTH: T = 7;
    const SPAWN_X: T = 3;
    const SPAWN_Y: T = 4;

    // Shapes
    //  A       B       C       D       E
    //                          x
    //           x        x     x
    //          xxx       x     x       xx
    //  xxxx     x      xxx     x       xx
    const SHAPES: [[Coord; 5]; 5] = [
        [(0, 0), (1, 0), (2, 0), (3, 0), (3, 0)],
        [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        [(0, 0), (0, 1), (0, 2), (0, 3), (0, 3)],
        [(0, 0), (0, 1), (1, 0), (1, 1), (1, 1)],
    ];
    const SHAPES_LEN: [usize; 5] = [4, 5, 5, 4, 4];

    type T = i16;
    type U = u64;
    type Coord = (T, T); // x, y
    type Hash = [u8; 3]; // shape, x coord, delta height

    #[derive(Debug)]
    struct Tetris {
        block: Coord,
        shape: u8,
        stopped: HashSet<Coord>,
        height: T,
    }

    impl Tetris {
        fn new() -> Self {
            Tetris {
                block: (SPAWN_X, SPAWN_Y),
                shape: 0,
                stopped: HashSet::new(),
                height: 0,
            }
        }

        fn collision(&self, coord: &Coord) -> bool {
            self.stopped.contains(coord)
                || coord.1 <= 0
                || coord.0 <= 0
                || coord.0 > CAVERN_WIDTH
        }

        fn shift(coord: Coord, delta_x: T, delta_y: T) -> Coord {
            (coord.0 + delta_x, coord.1 + delta_y)
        }

        fn coords(&self) -> impl Iterator<Item = Coord> + '_ {
            let i = self.shape as usize;
            SHAPES[i].into_iter().take(SHAPES_LEN[i]).map(
                |(delta_x, delta_y)| {
                    Tetris::shift(self.block, delta_x, delta_y)
                },
            )
        }

        fn push(&mut self, dir: T) {
            if !self
                .coords()
                .map(|coord| Tetris::shift(coord, dir, 0))
                .any(|coord| self.collision(&coord))
            {
                self.block.0 += dir;
            }
        }

        fn fall(&mut self) -> Option<Hash> {
            if !self
                .coords()
                .map(|coord| Tetris::shift(coord, 0, -1))
                .any(|coord| self.collision(&coord))
            {
                self.block.1 -= 1;
                None
            } else {
                let old_x = self.block.0 as u8;
                let old_shape = self.shape;
                let old_height = self.height;

                let coords = self.coords().collect::<Vec<_>>();
                self.stopped.extend(coords.iter());
                self.height = coords
                    .iter()
                    .map(|c| c.1)
                    .chain([self.height])
                    .max()
                    .unwrap();
                self.shape = (self.shape + 1) % SHAPES.len() as u8;
                self.block = (SPAWN_X, self.height + SPAWN_Y);

                Some([old_shape, old_x, (self.height - old_height) as u8])
            }
        }

        fn _print(&self) {
            let max_height = 25;
            if self.height >= max_height {
                return;
            }
            print!("{}c", 27 as char);
            for coord in (1..max_height + 1)
                .rev()
                .flat_map(|y| (1..CAVERN_WIDTH + 1).map(move |x| (x, y)))
            {
                if self.stopped.contains(&coord) {
                    print!("#");
                } else if self.coords().collect::<HashSet<_>>().contains(&coord)
                {
                    print!("@");
                } else {
                    print!(".");
                }
                if coord.0 == CAVERN_WIDTH {
                    println!();
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    }

    fn parse(s: &str) -> impl Iterator<Item = T> + '_ {
        s.chars()
            .flat_map(|ch| match ch {
                '<' => Some(-1),
                '>' => Some(1),
                _ => None,
            })
            .cycle()
    }

    pub fn part1(puzzle_input: &str) -> String {
        let mut tetris = Tetris::new();
        parse(puzzle_input)
            .flat_map(|delta_x| {
                tetris.push(delta_x);
                tetris.fall()
            })
            .nth(2021);
        tetris.height.to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let mut tetris = Tetris::new();
        let mut hashes = Vec::new();
        let mut indexes = HashMap::new();
        let mut hash_iter = parse(puzzle_input)
            .flat_map(|delta_x| {
                tetris.push(delta_x);
                tetris.fall()
            })
            .enumerate();
        let (cycle_i, cycle_len) = (0..)
            .find_map(|i| {
                for (i, hash) in
                    hash_iter.by_ref().take((i + 1) * 3 - hashes.len())
                {
                    hashes.push(hash);
                    indexes.entry(hash).or_insert(Vec::new()).push(i);
                }
                let hash = hashes[i];
                for &ii in
                    indexes.get(&hash).unwrap().iter().filter(|&&ii| ii < i)
                {
                    let n = i - ii;
                    if hashes[ii..i] == hashes[i..i + n]
                        && hashes[ii..i] == hashes[i + n..i + n * 2]
                    {
                        return Some((ii as U, (i - ii) as U));
                    }
                }
                None
            })
            .unwrap();
        let partial_height = |n_blocks, skip| {
            hashes
                .iter()
                .map(|&[_, _, h]| h as U)
                .skip(skip as usize)
                .take(n_blocks as usize)
                .sum::<U>()
        };
        let n_blocks: U = 1_000_000_000_000;
        let n_cycles = (n_blocks - cycle_i) / cycle_len;
        let height = partial_height(cycle_i, 0)
            + n_cycles * partial_height(cycle_len, cycle_i)
            + partial_height(
                n_blocks - n_cycles * cycle_len - cycle_i,
                cycle_i,
            );
        height.to_string()
    }
}

pub mod day18 {
    shared::puzzle!(18, 3_494, 2_062);
    shared::example!(
        18,
        a => (10, 10),
        b => (64, 58),
    );

    use std::collections::{HashSet, VecDeque};

    type T = usize;
    type Coord = (T, T, T);

    fn parse(s: &str) -> HashSet<Coord> {
        let parse_line = |ln: &str| {
            let coord: Vec<_> =
                ln.split(',').map(|s| s.parse::<T>().unwrap() + 1).collect();
            (coord[0], coord[1], coord[2])
        };
        s.lines().map(parse_line).collect()
    }

    pub fn part1(puzzle_input: &str) -> String {
        let cubes = parse(puzzle_input);
        let adj = |(x, y, z)| {
            [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]
            .into_iter()
        };
        cubes
            .iter()
            .fold(0, |acc, &c| {
                acc + adj(c).filter(|cc| !cubes.contains(cc)).count()
            })
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let cubes = parse(puzzle_input);
        let adj = |(x, y, z)| {
            [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]
            .into_iter()
        };

        // bfs flood fill
        let (x_max, y_max, z_max) =
            cubes.iter().fold((0, 0, 0), |(x1, y1, z1), &(x2, y2, z2)| {
                (x1.max(x2), y1.max(y2), z1.max(z2))
            });
        let start = [(0, 0, 0)];
        let mut exterior = HashSet::from(start);
        let mut queue = VecDeque::from(start);
        while !queue.is_empty() {
            for coord in adj(queue.pop_front().unwrap()) {
                let (x, y, z) = coord;
                if exterior.contains(&coord)
                    || cubes.contains(&coord)
                    || x > x_max + 1
                    || y > y_max + 1
                    || z > z_max + 1
                {
                    continue;
                }
                exterior.insert(coord);
                queue.push_back(coord);
            }
        }

        cubes
            .iter()
            .fold(0, |acc, &c| {
                acc + adj(c)
                    .filter(|cc| !cubes.contains(cc) && exterior.contains(cc))
                    .count()
            })
            .to_string()
    }
}

pub mod day19 {
    shared::puzzle!(19, 1_092, 3_542);
    shared::example!(19, 33, 3_472);

    use std::collections::BinaryHeap;

    type T = u16;
    type Blueprint = [[T; 3]; 4]; // ore, clay, obsidian, geode

    const OBSIDIAN: usize = 2;
    const GEODE: usize = 3;

    #[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
    struct State {
        robots: [T; 4],
        resources: [T; 4],
        time: T,
    }

    fn parse_blueprint(s: &str) -> Blueprint {
        let costs = s.split(' ').flat_map(str::parse).collect::<Vec<_>>();
        [
            [costs[0], 0, 0],
            [costs[1], 0, 0],
            [costs[2], costs[3], 0],
            [costs[4], 0, costs[5]],
        ]
    }

    pub fn part1(puzzle_input: &str) -> String {
        puzzle_input
            .lines()
            .map(parse_blueprint)
            .map(|b| eval_blueprint(b, 24))
            .enumerate()
            .map(|(i, x)| (i + 1) as T * x)
            .sum::<T>()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        puzzle_input
            .lines()
            .map(parse_blueprint)
            .map(|b| eval_blueprint(b, 32))
            .take(3)
            .product::<T>()
            .to_string()
    }

    fn eval_blueprint(blueprint: Blueprint, time_limit: T) -> T {
        let max_robots = blueprint.iter().fold(vec![0; GEODE], |max, cost| {
            max.into_iter().zip(cost).map(|(m, &c)| m.max(c)).collect()
        });
        let mut best = 0;
        let mut pqueue = BinaryHeap::from_iter([State {
            robots: [1, 0, 0, 0],
            resources: [0; 4],
            time: 0,
        }]);
        while let Some(state) = pqueue.pop() {
            let t = time_limit - state.time;
            let run_rate = |i| state.resources[i] + t * state.robots[i];
            let upper_bound = |i| run_rate(i) + t * (t - 1) / 2;
            best = best.max(run_rate(GEODE));
            if upper_bound(GEODE) <= best
                || upper_bound(OBSIDIAN) < blueprint[GEODE][OBSIDIAN]
            {
                continue;
            }
            (0..=GEODE)
                .filter(|&i| i == GEODE || state.robots[i] < max_robots[i])
                .flat_map(|i| build_robot(i, &state, blueprint[i], time_limit))
                .for_each(|new_state| pqueue.push(new_state));
        }
        best
    }

    fn build_robot(
        robot: usize,
        state: &State,
        cost: [T; GEODE],
        time_limit: T,
    ) -> Option<State> {
        // check required robots
        if cost.iter().zip(state.robots).any(|(&c, r)| c > 0 && r == 0) {
            return None;
        }
        // check required time
        let t = state
            .resources
            .iter()
            .zip(state.robots)
            .zip(cost)
            .map(|((&r, n), c)| if r >= c { 0 } else { (c - r + n - 1) / n })
            .max()
            .unwrap();
        if state.time + t + 2 > time_limit {
            return None;
        }
        // collect (t) + build (1)
        let mut new_state = state.clone();
        for ((r, n), c) in new_state
            .resources
            .iter_mut()
            .zip(new_state.robots)
            .zip(cost.iter().chain(&[0]))
        {
            *r += n * (t + 1) - c;
        }
        new_state.time += t + 1;
        new_state.robots[robot] += 1;
        Some(new_state)
    }
}

pub mod day20 {
    shared::puzzle!(20, 2_827, 7_834_270_093_909_i64);
    shared::example!(20, 3, 1_623_178_306);

    type T = i64;

    fn parse(s: &str, key: T) -> Vec<T> {
        s.lines().flat_map(str::parse).map(|x: T| x * key).collect()
    }

    pub fn part1(puzzle_input: &str) -> String {
        let coords = parse(puzzle_input, 1);
        let mut idx = Vec::from_iter(0..coords.len());
        (0..coords.len()).for_each(|i| mix(i, &coords, &mut idx));
        let i_zero_old = coords.iter().position(|&i| i == 0).unwrap();
        let i_zero_new = idx.iter().position(|&i| i == i_zero_old).unwrap();
        [1000, 2000, 3000]
            .into_iter()
            .map(|n| (i_zero_new + n).rem_euclid(coords.len()))
            .map(|i| coords[idx[i]])
            .sum::<T>()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let coords = parse(puzzle_input, 811_589_153);
        let mut idx = Vec::from_iter(0..coords.len());
        for _ in 1..=10 {
            (0..coords.len()).for_each(|i| mix(i, &coords, &mut idx));
        }
        let i_zero_old = coords.iter().position(|&i| i == 0).unwrap();
        let i_zero_new = idx.iter().position(|&i| i == i_zero_old).unwrap();
        [1000, 2000, 3000]
            .into_iter()
            .map(|n| (i_zero_new + n).rem_euclid(coords.len()))
            .map(|i| coords[idx[i]])
            .sum::<T>()
            .to_string()
    }

    fn mix(index: usize, coords: &[T], idx: &mut Vec<usize>) {
        let n_forward = if coords[index] == 0 {
            return;
        } else {
            coords[index].rem_euclid(coords.len() as T - 1) as usize
        };
        let i_old = idx.iter().position(|&i| i == index).unwrap();
        idx.remove(i_old);
        let i_new = (i_old + n_forward) % idx.len();
        idx.insert(i_new, index);
    }
}

pub mod day21 {
    shared::puzzle!();
    shared::example!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day22 {
    shared::puzzle!();
    shared::example!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day23 {
    shared::puzzle!();
    shared::example!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day24 {
    shared::puzzle!();
    shared::example!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day25 {
    shared::puzzle!();
    shared::example!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}
