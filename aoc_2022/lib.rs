shared::table_of_contents!();

pub mod day1 {
    use std::collections::BinaryHeap;

    shared::input!(1);
    shared::test!(72_602, 207_410); // examples: 24_000, 45_000

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
    shared::test!(10_624, 14_060); // examples: 15, 12

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
    shared::test!(8_515, 2_434); // examples: 157, 70

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
    shared::test!(477, 830); // examples: 2, 4

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
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day6 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day7 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day8 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day9 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day10 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day11 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day12 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day13 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day14 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day15 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day16 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day17 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day18 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day19 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day20 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day21 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day22 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day23 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day24 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day25 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}
