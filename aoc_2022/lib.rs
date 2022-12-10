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

    type T = usize;

    use crate::day2::Shape::*;
    enum Shape {
        Rock,
        Paper,
        Scissors,
    }
    use crate::day2::Outcome::*;
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
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day9 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day10 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day11 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day12 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day13 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day14 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day15 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
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
