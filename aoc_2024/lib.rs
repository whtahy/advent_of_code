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
    shared::day!(3);
    shared::part1!(161, 160_672_468);
    shared::part2!(48, 84_893_551);

    type T = u32;

    fn parse(s: &str) -> T {
        s.split("mul(")
            .flat_map(|s| {
                let inner = s.split(')').next()?;
                let (a, b) = inner.split_once(',')?;
                Some(a.parse::<T>().ok()? * b.parse::<T>().ok()?)
            })
            .sum()
    }

    pub fn part1(puzzle_input: &str) -> String {
        parse(puzzle_input).to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        puzzle_input
            .split("do()")
            .map(|s| s.split("don't()").next().unwrap_or(s))
            .map(parse)
            .sum::<T>()
            .to_string()
    }
}

pub mod day4 {
    shared::day!(4);
    shared::part1!(18, 2_662);
    shared::part2!(9, 2_034);

    fn parse(s: &str) -> Vec<Vec<char>> {
        s.lines().map(|ln| ln.chars().collect()).collect()
    }

    pub fn part1(puzzle_input: &str) -> String {
        let board = parse(puzzle_input);
        let target = ['X', 'M', 'A', 'S'];
        let search = |r, c, dr, dc| {
            (0..target.len() as isize)
                .map(|i| {
                    let row = board.get((r + dr * i) as usize)?;
                    let val = row.get((c + dc * i) as usize)?;
                    Some(target[i as usize] == *val)
                })
                .all(|x| x.unwrap_or(false))
        };
        let (n_rows, n_cols) = (board.len() as isize, board[0].len() as isize);
        let rc = (0..n_rows).flat_map(|r| (0..n_cols).map(move |c| (r, c)));
        let delta = (-1..=1)
            .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
            .filter(|&(dr, dc)| dr != 0 || dc != 0);
        rc.flat_map(|rc| delta.clone().map(move |drc| (rc, drc)))
            .filter(|&((r, c), (dr, dc))| search(r, c, dr, dc))
            .count()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let board = parse(puzzle_input);
        let search = |r: usize, c| {
            let ms = |a, b| matches!((a, b), ('M', 'S') | ('S', 'M'));
            board[r][c] == 'A'
                && ms(board[r + 1][c + 1], board[r - 1][c - 1])
                && ms(board[r + 1][c - 1], board[r - 1][c + 1])
        };
        let (r_max, c_max) = (board.len() - 1, board[0].len() - 1);
        let rc = (1..r_max).flat_map(|r| (1..c_max).map(move |c| (r, c)));
        rc.filter(|&(r, c)| search(r, c)).count().to_string()
    }
}

pub mod day5 {
    shared::day!(5);
    shared::part1!(143, 5_108);
    shared::part2!(123, 7_380);

    use std::{cmp::Ordering, collections::HashMap};

    type T = u32;
    type Rules = HashMap<T, Vec<T>>;
    type Pages = Vec<Book>;
    type Book = Vec<T>;

    fn parse(s: &str) -> (Rules, Pages) {
        let (upper, lower) = s.split_once("\r\n\r\n").unwrap();
        let rules: Rules = upper.lines().fold(HashMap::new(), |mut map, ln| {
            let kv = ln
                .split('|')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();
            map.entry(kv[0]).or_default().push(kv[1]);
            map
        });
        let pages = lower
            .lines()
            .map(|ln| ln.split(',').map(|s| s.parse().unwrap()).collect())
            .collect();
        (rules, pages)
    }

    pub fn part1(puzzle_input: &str) -> String {
        let (rules, pages) = parse(puzzle_input);
        pages
            .iter()
            .filter(|book| safe(&rules, book))
            .map(|book| book[book.len() / 2])
            .sum::<T>()
            .to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let (rules, pages) = parse(puzzle_input);
        pages
            .into_iter()
            .filter(|book| !safe(&rules, book))
            .map(|mut book| {
                book.sort_by(|&a, &b| ord(&rules, a, b));
                book[book.len() / 2]
            })
            .sum::<T>()
            .to_string()
    }

    fn safe(rules: &Rules, book: &Book) -> bool {
        book.is_sorted_by(|&a, &b| matches!(ord(rules, a, b), Ordering::Less))
    }

    fn ord(rules: &Rules, a: T, b: T) -> Ordering {
        match rules.get(&a).map(|v| v.contains(&b)) {
            Some(true) => Ordering::Less,
            _ => Ordering::Greater,
        }
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
