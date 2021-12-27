pub const TABLE_OF_CONTENTS: [[fn() -> String; 2]; 25] = [
    [day1::part1, day1::part2],
    [day2::part1, day2::part2],
    [day3::part1, day3::part2],
    [day4::part1, day4::part2],
    [day5::part1, day5::part2],
    [day6::part1, day6::part2],
    [day7::part1, day7::part2],
    [day8::part1, day8::part2],
    [day9::part1, day9::part2],
    [day10::part1, day10::part2],
    [day11::part1, day11::part2],
    [day12::part1, day12::part2],
    [day13::part1, day13::part2],
    [day14::part1, day14::part2],
    [day15::part1, day15::part2],
    [day16::part1, day16::part2],
    [day17::part1, day17::part2],
    [day18::part1, day18::part2],
    [day19::part1, day19::part2],
    [day20::part1, day20::part2],
    [day21::part1, day21::part2],
    [day22::part1, day22::part2],
    [day23::part1, day23::part2],
    [day24::part1, day24::part2],
    [day25::part1, day25::part2],
];

macro_rules! input {
    ($x:expr) => {
        const INPUT: &str = include_str!(concat!("../../input/2021/day", $x, ".txt"));
    };
}

pub mod day1 {
    input!(1);
    use std::collections::HashSet;

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day1::part1(), 533.to_string());
    /// ```
    pub fn part1() -> String {
        INPUT
            .lines()
            .fold(0, |sum, x| sum + x.parse::<u32>().unwrap())
            .to_string()
    }

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day1::part2(), 73_272.to_string());
    /// ```
    pub fn part2() -> String {
        let mut v = Vec::new();
        let mut h = HashSet::new();

        let mut f = 0;
        v.push(f);
        h.insert(f);

        for l in INPUT.lines() {
            let x: i32 = l.parse().unwrap();
            f += x;
            if h.contains(&f) {
                return f.to_string();
            } else {
                v.push(f);
                h.insert(f);
            }
        }

        let offset = f;
        for i in 1.. {
            for x in v.iter().filter(|x| x != &&0) {
                let c = x + i * offset;
                if h.contains(&c) {
                    return c.to_string();
                }
            }
        }

        panic!()
    }
}

pub mod day2 {
    input!(2);
    use std::collections::{HashMap, HashSet};

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day2::part1(), 7_134.to_string());
    /// ```
    pub fn part1() -> String {
        fn count(s: &str) -> HashMap<char, i32> {
            let mut m = HashMap::new();
            for c in s.chars() {
                m.entry(c).and_modify(|x| *x += 1).or_insert(1);
            }
            m
        }

        let mut x2 = 0;
        let mut x3 = 0;
        for l in INPUT.lines() {
            let c = count(l);
            if c.values().any(|x| x == &2) {
                x2 += 1;
            }
            if c.values().any(|x| x == &3) {
                x3 += 1;
            }
        }
        (x2 * x3).to_string()
    }

    /// ```
    /// let ans = "kbqwtcvzhmhpoelrnaxydifyb";
    /// assert_eq!(advent_of_code::aoc_2018::day2::part2(), ans);
    /// ```
    pub fn part2() -> String {
        for i in 0..INPUT.lines().next().unwrap().len() {
            let mut m = HashSet::new();
            for l in INPUT.lines() {
                let b = l
                    .char_indices()
                    .filter(|(j, _)| j != &i)
                    .map(|(_, x)| x)
                    .collect();
                if m.contains(&b) {
                    return b;
                } else {
                    m.insert(b);
                }
            }
        }
        panic!()
    }
}

pub mod day3 {
    input!(3);
    use std::collections::{HashMap, HashSet};

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day3::part1(), 112_378.to_string());
    /// ```
    pub fn part1() -> String {
        let mut c = HashSet::new();
        let mut d = HashSet::new();

        for l in INPUT.lines() {
            let sx = l
                .split(&[' ', ',', ':', 'x'][..])
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<i32>>();

            let (p_x, p_y) = (sx[0], sx[1]);
            let (w, h) = (sx[2], sx[3]);

            for x in p_x..(p_x + w) {
                for y in p_y..(p_y + h) {
                    if !c.insert((x, y)) {
                        d.insert((x, y));
                    }
                }
            }
        }

        d.len().to_string()
    }

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day3::part2(), 603.to_string());
    /// ```
    pub fn part2() -> String {
        fn disjoint(a: &[i32], b: &[i32]) -> bool {
            let (x1, y1) = (a[0], a[1]);
            let (w1, h1) = (a[2], a[3]);

            let (x2, y2) = (b[0], b[1]);
            let (w2, h2) = (b[2], b[3]);

            !((x1 <= x2 && x2 <= (x1 + w1 - 1)) || (x2 <= x1 && x1 <= (x2 + w2 - 1)))
                || !((y1 <= y2 && y2 <= (y1 + h1 - 1)) || (y2 <= y1 && y1 <= (y2 + h2 - 1)))
        }

        let mut m = HashMap::new();

        for l in INPUT.lines() {
            let sx = l
                .split(&['#', ' ', ',', ':', 'x'][..])
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<i32>>();
            m.insert(sx[0], sx[1..].to_vec());
        }

        for id in m.keys() {
            if m.keys().all(|i| id == i || disjoint(&m[id], &m[i])) {
                return id.to_string();
            }
        }

        panic!()
    }
}

pub mod day4 {
    input!(4);
    use std::collections::HashMap;

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day4::part1(), 131_469.to_string());
    /// ```
    pub fn part1() -> String {
        let m = parse_input();
        let g = m.keys().max_by_key(|k| m[k].values().sum::<i32>()).unwrap();
        let t = m[g].keys().max_by_key(|k| m[g][k]).unwrap();
        (g * t).to_string()
    }

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day4::part2(), 96_951.to_string());
    /// ```
    pub fn part2() -> String {
        let m = parse_input();
        let g = m.keys().max_by_key(|k| m[k].values().max()).unwrap();
        let t = m[g].keys().max_by_key(|k| m[g][k]).unwrap();
        (g * t).to_string()
    }

    fn parse_input() -> HashMap<i32, HashMap<i32, i32>> {
        fn parse_line(s: &str) -> (&str, i32, &str) {
            let v = s.split(&[' ', ':', ']'][..]).collect::<Vec<_>>();
            (v[4], v[2].parse().unwrap(), v[5])
        }

        let mut v = INPUT.lines().collect::<Vec<_>>();
        v.sort_unstable();

        let mut m = HashMap::new();
        let mut t0 = 0;
        let mut id = 0;

        for l_i in v {
            let (s_i, t_i, id_i) = parse_line(l_i);
            if s_i == "Guard" {
                id = id_i[1..].parse().unwrap()
            } else if s_i == "falls" {
                t0 = t_i;
            } else if s_i == "wakes" {
                m.entry(id).or_insert_with(HashMap::new);
                for t in t0..t_i {
                    m.get_mut(&id)
                        .unwrap()
                        .entry(t)
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                }
            }
        }

        m
    }
}

pub mod day5 {
    input!(5);
    use std::collections::{BTreeSet, VecDeque};

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day5::part1(), 10_804.to_string());
    /// ```
    pub fn part1() -> String {
        let left = VecDeque::new();
        let right = INPUT.chars().collect::<VecDeque<_>>();
        react(left, right).len().to_string()
    }

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day5::part2(), 6_650.to_string());
    /// ```
    pub fn part2() -> String {
        let a = VecDeque::new();
        let b = INPUT.chars().collect::<VecDeque<_>>();
        let v = react(a, b);
        let mut nx = BTreeSet::new();
        for i in b'a'..=b'z' {
            let left = VecDeque::new();
            let right = v
                .iter()
                .filter(|x| x.to_ascii_lowercase() != i as char)
                .cloned()
                .collect::<VecDeque<char>>();
            let n = react(left, right).len() as i32;
            nx.insert(n);
        }

        nx.iter().min().unwrap().to_string()
    }

    fn react(mut left: VecDeque<char>, mut right: VecDeque<char>) -> VecDeque<char> {
        left.push_front(' ');
        right.push_back(' ');
        loop {
            if right.front().unwrap() == &' ' {
                break;
            }

            let l = left.back().unwrap();
            let r = right.pop_front().unwrap();

            if l != &r && l.to_ascii_lowercase() == r.to_ascii_lowercase() {
                left.pop_back();
            } else {
                left.push_back(r);
            }
        }

        left.pop_front();
        left
    }
}

pub mod day6 {
    input!(6);
    use std::cmp::Ordering;
    use std::collections::{HashMap, HashSet};

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day6::part1(), 4_976.to_string());
    /// ```
    pub fn part1() -> String {
        let targets = parse_input();
        let (x_min, x_max, y_min, y_max) = bounds(&targets);

        let mut areas = HashMap::new();
        let mut infinite = HashSet::new();

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let p = (x, y);
                let mut t_min = Vec::new();
                let mut d_min = manhattan(p, targets[0]);

                for t in &targets {
                    let d = manhattan(p, *t);
                    match d.cmp(&d_min) {
                        Ordering::Less => {
                            d_min = d;
                            t_min.clear();
                            t_min.push(t);
                        }
                        Ordering::Equal => {
                            t_min.push(t);
                        }
                        _ => continue,
                    }
                }

                if t_min.len() == 1 && !infinite.contains(t_min[0]) {
                    *areas.entry(t_min[0]).or_insert(0) += 1;
                }

                if x == x_min || x == x_max || y == y_min || y == y_max {
                    for p_i in t_min {
                        infinite.insert(p_i);
                        areas.remove(p_i);
                    }
                }
            }
        }

        areas.values().max().unwrap().to_string()
    }

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day6::part2(), 46_462.to_string());
    /// ```
    pub fn part2() -> String {
        let targets = parse_input();
        let (x_min, x_max, y_min, y_max) = bounds(&targets);

        let mut n = 0;

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let p = (x, y);
                if targets.iter().map(|t| manhattan(p, *t)).sum::<i32>() < 10_000 {
                    n += 1;
                }
            }
        }

        n.to_string()
    }

    fn manhattan(a: (i32, i32), b: (i32, i32)) -> i32 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }

    fn bounds(targets: &[(i32, i32)]) -> (i32, i32, i32, i32) {
        let mut x_min = &targets[0].0;
        let mut x_max = x_min;
        let mut y_min = &targets[0].1;
        let mut y_max = y_min;

        for (x, y) in targets {
            if x < x_min {
                x_min = x;
            }
            if x > x_max {
                x_max = x;
            }
            if y < y_min {
                y_min = y;
            }
            if y > y_max {
                y_max = y;
            }
        }

        (*x_min, *x_max, *y_min, *y_max)
    }

    fn parse_input() -> Vec<(i32, i32)> {
        let mut v = Vec::new();
        for l in INPUT.lines() {
            let pt: Vec<i32> = l.split(", ").map(|s| s.parse().unwrap()).collect();
            v.push((pt[0], pt[1]));
        }
        v
    }
}

pub mod day7 {
    input!(7);
    use std::collections::{BTreeSet, HashMap};

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day7::part1(), "EUGJKYFQSCLTWXNIZMAPVORDBH");
    /// ```
    pub fn part1() -> String {
        let (pop, mut prereqs) = parse_input();

        let mut completed = Vec::new();
        let mut pool: BTreeSet<char> = pop
            .into_iter()
            .filter(|x| !prereqs.contains_key(x))
            .collect();

        while !prereqs.is_empty() {
            let mut buffer = Vec::new();

            for (&k, v) in &prereqs {
                if v.iter().all(|x| completed.contains(x)) {
                    pool.insert(k);
                    buffer.push(k);
                }
            }

            for k in buffer {
                prereqs.remove(&k);
            }

            let next = *pool.iter().next().unwrap();
            pool.remove(&next);
            completed.push(next);
        }

        completed.into_iter().collect()
    }

    /// ```
    /// assert_eq!(advent_of_code::aoc_2018::day7::part2(), 1014.to_string());
    /// ```
    pub fn part2() -> String {
        let (pop, mut prereqs) = parse_input();

        let mut completed = BTreeSet::new();
        let mut pool: BTreeSet<char> = pop
            .into_iter()
            .filter(|x| !prereqs.contains_key(x))
            .collect();

        let mut t = 0;
        let mut workers = Vec::new();
        let mut timers = Vec::new();
        while !prereqs.is_empty() || !pool.is_empty() {
            let mut buffer = Vec::new();

            // update pool
            for (&k, v) in &prereqs {
                if v.is_subset(&completed) {
                    pool.insert(k);
                    buffer.push(k);
                }
            }

            // update prereqs
            for k in buffer {
                prereqs.remove(&k);
            }

            // load workers
            while workers.len() < 5 && !pool.is_empty() {
                let next = *pool.iter().next().unwrap();
                pool.remove(&next);
                workers.push(next);
                timers.push(char_to_u32(next));
            }

            // step forward until worker is done
            let t_next = *timers.iter().min().unwrap();
            let i_next = timers.iter().position(|&x| x == t_next).unwrap();
            let next = workers[i_next];
            let delta_t = timers[i_next];

            // update timers
            for t in &mut timers {
                *t -= delta_t;
            }

            // update totals
            t += delta_t;
            completed.insert(next);

            // cleanup
            workers.remove(i_next);
            timers.remove(i_next);
        }

        t.to_string()
    }

    fn char_to_u32(ch: char) -> u32 {
        60 + ch as u32 - 'A' as u32 + 1
    }

    fn parse_input() -> (BTreeSet<char>, HashMap<char, BTreeSet<char>>) {
        let mut h = BTreeSet::new();
        let mut m = HashMap::new();
        for l in INPUT.lines() {
            let tokens: Vec<&str> = l.split(' ').collect();
            let x: char = tokens[1].parse().unwrap();
            let y: char = tokens[7].parse().unwrap();
            h.insert(x);
            h.insert(y);
            m.entry(y).or_insert_with(BTreeSet::new).insert(x);
        }
        (h, m)
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
