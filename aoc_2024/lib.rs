shared::table_of_contents!();

pub mod day1 {
    shared::day!(1);
    shared::part1!(11, 2_000_468);
    shared::part2!(31, 18_567_089);

    use std::collections::HashMap;

    type T = u32;

    fn parse_line(ln: &str) -> (T, T) {
        let split = ln.split_once("   ").unwrap();
        (split.0.parse().unwrap(), split.1.parse().unwrap())
    }

    pub fn part1(puzzle_input: &str) -> String {
        let (mut lhs, mut rhs) = (Vec::<T>::new(), Vec::new());
        for ln in puzzle_input.lines() {
            let (left, right) = parse_line(ln);
            lhs.push(left);
            rhs.push(right);
        }
        lhs.sort();
        rhs.sort();
        lhs.iter()
            .zip(rhs.iter())
            .map(|(x, y)| x.abs_diff(*y))
            .sum::<T>()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let (mut lhs, mut rhs) = (Vec::<T>::new(), HashMap::<T, T>::new());
        for ln in puzzle_input.lines() {
            let (left, right) = parse_line(ln);
            lhs.push(left);
            *rhs.entry(right).or_insert(0) += 1;
        }
        lhs.iter()
            .map(|x| x * rhs.get(x).unwrap_or(&0))
            .sum::<T>()
            .to_string()
    }
}

pub mod day2 {
    shared::day!(2);
    shared::part1!(2, 564);
    shared::part2!(4, 604);

    type T = i32;

    fn parse_line(ls: &str) -> Vec<T> {
        ls.split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    }

    fn delta(v: Vec<T>) -> Vec<T> {
        v.windows(2).map(|w| w[0] - w[1]).collect()
    }

    pub fn part1(puzzle_input: &str) -> String {
        puzzle_input
            .lines()
            .map(parse_line)
            .map(delta)
            .filter(|delta| {
                delta.iter().all(|x| [1, 2, 3].contains(x))
                    || delta.iter().all(|x| [-1, -2, -3].contains(x))
            })
            .count()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        puzzle_input
            .lines()
            .map(parse_line)
            .map(delta)
            .filter(|delta| {
                safe(delta, &[1, 2, 3]) || safe(delta, &[-1, -2, -3])
            })
            .count()
            .to_string()
    }

    fn safe(delta: &[T], valid: &[T]) -> bool {
        let invalid = delta
            .iter()
            .enumerate()
            .filter(|(_, x)| !valid.contains(x))
            .map(|(i, _)| i)
            .take(3)
            .collect::<Vec<_>>();
        match invalid.len() {
            0 => true,
            1 => {
                let i = invalid[0];
                i == 0
                    || i == delta.len() - 1
                    || valid.contains(&(delta[i] + delta[i + 1]))
                    || valid.contains(&(delta[i] + delta[i - 1]))
            }
            2 => {
                let (i, j) = (invalid[0], invalid[1]);
                i.abs_diff(j) == 1 && valid.contains(&(delta[i] + delta[j]))
            }
            3 => false,
            _ => unreachable!(),
        }
    }
}

pub mod day3 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day4 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day5 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day6 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day7 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day8 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day9 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day10 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day11 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day12 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day13 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day14 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day15 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day16 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day17 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day18 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day19 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day20 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day21 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day22 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day23 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day24 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}

pub mod day25 {
    shared::day!();
    shared::part1!();
    shared::part2!();

    pub fn part1(_: &str) -> String {
        todo!()
    }

    pub fn part2(_: &str) -> String {
        todo!()
    }
}
