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
    shared::day!(6);
    shared::part1!(41, 5_318);
    shared::part2!(6, 1_831);

    use std::collections::{HashMap, HashSet};

    type T = usize;
    type Obstacles = HashMap<T, (Vec<T>, Vec<T>)>;

    use Dir::*;
    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    enum Dir {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    struct Coord {
        x: T,
        y: T,
    }

    struct Room {
        width: T,
        height: T,
        obstacles: Obstacles,
    }

    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    struct Guard {
        x: T,
        y: T,
        dir: Dir,
    }

    fn parse(s: &str) -> (Guard, Room) {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let mut start = None;
        let mut obstacles = HashMap::new();
        for (y, ln) in s.lines().enumerate() {
            let y = height - 1 - y;
            for (x, ch) in ln.chars().enumerate() {
                if ch == '#' {
                    obstacles.entry(x).or_insert((vec![], vec![])).1.push(y);
                    obstacles.entry(y).or_insert((vec![], vec![])).0.push(x);
                } else if ch == '^' {
                    start = Some((x, y));
                }
            }
        }
        obstacles.values_mut().for_each(|pair| {
            pair.0.sort_unstable();
            pair.1.sort_unstable();
        });
        let (x, y) = start.unwrap();
        (
            Guard { x, y, dir: Up },
            Room {
                width,
                height,
                obstacles,
            },
        )
    }

    pub fn part1(puzzle_input: &str) -> String {
        let (mut guard, room) = parse(puzzle_input);
        let mut history = HashSet::new();
        while guard.x < room.width && guard.y < room.height {
            history.insert((guard.x, guard.y));
            let next = forward(&guard);
            if room
                .obstacles
                .get(&next.x)
                .and_then(|(_, v)| v.binary_search(&next.y).ok())
                .is_some()
            {
                guard.dir = clockwise(guard.dir);
            } else {
                (guard.x, guard.y) = (next.x, next.y);
            }
        }
        history.len().to_string()
    }

    pub fn part2(puzzle_input: &str) -> String {
        let (mut guard, room) = parse(puzzle_input);
        let start = Coord {
            x: guard.x,
            y: guard.y,
        };
        let mut new_obstacles = HashSet::new();
        while guard.x < room.width && guard.y < room.height {
            let next = forward(&guard);
            if room
                .obstacles
                .get(&next.x)
                .and_then(|(_, y)| y.binary_search(&next.y).ok())
                .is_some()
            {
                guard.dir = clockwise(guard.dir);
            } else {
                if cycle(&start, &next, &room.obstacles) {
                    new_obstacles.insert(next.clone());
                }
                (guard.x, guard.y) = (next.x, next.y);
            }
        }
        new_obstacles.len().to_string()
    }

    fn forward(&Guard { x, y, dir }: &Guard) -> Coord {
        let (x, y) = match dir {
            Up => (x, y + 1),
            Down => (x, y - 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        };
        Coord { x, y }
    }

    fn clockwise(dir: Dir) -> Dir {
        match dir {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn cycle(
        start: &Coord,
        new_obstacle: &Coord,
        obstacle_map: &Obstacles,
    ) -> bool {
        let jump = |guard| {
            let next = match (
                jump_to_map(&guard, obstacle_map),
                jump_to_obstacle(&guard, new_obstacle),
            ) {
                (Some(a), Some(b)) => match guard.dir {
                    Up if a.y < b.y => a,
                    Right if a.x < b.x => a,
                    Down if b.y < a.y => a,
                    Left if b.x < a.x => a,
                    _ => b,
                },
                (Some(a), _) => a,
                (_, Some(b)) => b,
                (None, None) => return None,
            };
            Some(next)
        };
        let mut tortoise = Some(Guard {
            x: start.x,
            y: start.y,
            dir: Up,
        });
        let mut hare = tortoise.clone();
        loop {
            tortoise = tortoise.and_then(jump);
            hare = hare.and_then(jump).and_then(jump);
            if tortoise.is_none() || hare.is_none() {
                return false;
            } else if hare == tortoise {
                return true;
            }
        }
    }

    fn jump_to_obstacle(guard: &Guard, obstacle: &Coord) -> Option<Guard> {
        if guard.x == obstacle.x && guard.y == obstacle.y {
            return None;
        }
        let (x, y) = match (
            guard.dir,
            guard.x == obstacle.x,
            guard.y < obstacle.y,
            guard.y == obstacle.y,
            obstacle.x < guard.x,
        ) {
            (Up, true, true, _, _) => (guard.x, obstacle.y - 1),
            (Down, true, false, _, _) => (guard.x, obstacle.y + 1),
            (Left, _, _, true, true) => (obstacle.x + 1, guard.y),
            (Right, _, _, true, false) => (obstacle.x - 1, guard.y),
            _ => return None,
        };
        Some(Guard {
            x,
            y,
            dir: clockwise(guard.dir),
        })
    }

    fn jump_to_map(guard: &Guard, obstacle_map: &Obstacles) -> Option<Guard> {
        let (fix, jmp) = match guard.dir {
            Up | Down => (guard.x, guard.y),
            Left | Right => (guard.y, guard.x),
        };
        obstacle_map
            .get(&fix)
            .map(|(xs, ys)| match guard.dir {
                Up | Down => ys,
                Left | Right => xs,
            })
            .filter(|vec| !vec.is_empty())
            .filter(|vec| match guard.dir {
                Up | Right => jmp < *vec.last().unwrap(),
                Down | Left => vec[0] < jmp,
            })
            .map(|vec| {
                let i = vec.binary_search(&jmp).unwrap_err();
                let next = match guard.dir {
                    Up | Right => vec[i] - 1,
                    Down | Left => vec[i - 1] + 1,
                };
                let (x, y) = match guard.dir {
                    Up | Down => (fix, next),
                    Left | Right => (next, fix),
                };
                Guard {
                    x,
                    y,
                    dir: clockwise(guard.dir),
                }
            })
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
