pub mod aoc_2015 {
    pub mod day1 {}
    pub mod day2 {}
    pub mod day3 {}
    pub mod day4 {}
    pub mod day5 {}
}

pub mod aoc_2016 {
    pub mod day1 {}
    pub mod day2 {}
    pub mod day3 {}
    pub mod day4 {}
    pub mod day5 {}
}

pub mod aoc_2017 {
    pub mod day1 {}
    pub mod day2 {}
    pub mod day3 {}
    pub mod day4 {}
    pub mod day5 {}
}

pub mod aoc_2018 {
    pub mod day1 {
        use std::collections::HashSet;
        const INPUT: &str = include_str!("../input/2018/day1.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2018::day1::part1(), 533);
        /// ```
        pub fn part1() -> i32 {
            INPUT.lines().map(|x| x.parse::<i32>().unwrap()).sum()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2018::day1::part2(), 73_272);
        /// ```
        pub fn part2() -> i32 {
            let mut v = Vec::new();
            let mut h = HashSet::new();

            let mut f = 0;
            v.push(f);
            h.insert(f);

            for l in INPUT.lines() {
                let x: i32 = l.parse().unwrap();
                f += x;
                if h.contains(&f) {
                    return f;
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
                        return c;
                    }
                }
            }

            panic!()
        }
    }

    pub mod day2 {
        use std::collections::{HashMap, HashSet};
        const INPUT: &str = include_str!("../input/2018/day2.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2018::day2::part1(), 7_134);
        /// ```
        pub fn part1() -> i32 {
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
            x2 * x3
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
        use std::collections::{HashMap, HashSet};
        const INPUT: &str = include_str!("../input/2018/day3.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2018::day3::part1(), 112_378);
        /// ```
        pub fn part1() -> i32 {
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

            d.len() as i32
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2018::day3::part2(), 603);
        /// ```
        pub fn part2() -> i32 {
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
                    return *id;
                }
            }

            panic!()
        }
    }

    pub mod day4 {
        use std::collections::HashMap;
        const INPUT: &str = include_str!("../input/2018/day4.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2018::day4::part1(), 131_469);
        /// ```
        pub fn part1() -> i32 {
            let m = parse_input();
            let g = m.keys().max_by_key(|k| m[k].values().sum::<i32>()).unwrap();
            let t = m[&g].keys().max_by_key(|k| m[g][k]).unwrap();
            g * t
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2018::day4::part2(), 96_951);
        /// ```
        pub fn part2() -> i32 {
            let m = parse_input();
            let g = m.keys().max_by_key(|k| m[k].values().max()).unwrap();
            let t = m[&g].keys().max_by_key(|k| m[&g][k]).unwrap();
            g * t
        }

        fn parse_input() -> HashMap<i32, HashMap<i32, i32>> {
            fn parse_line(s: &str) -> (&str, i32, &str) {
                let v = s.split(&[' ', ':', ']'][..]).collect::<Vec<_>>();
                (v[4], v[2].parse().unwrap(), v[5])
            }

            let mut v = INPUT.lines().collect::<Vec<_>>();
            v.sort();

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
        use std::collections::{BTreeSet, VecDeque};
        const INPUT: &str = include_str!("../input/2018/day5.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2018::day5::part1(), 10_804);
        /// ```
        pub fn part1() -> i32 {
            let left = VecDeque::new();
            let right = INPUT.chars().collect::<VecDeque<_>>();
            react(left, right).len() as i32
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2018::day5::part2(), 6_650);
        /// ```
        pub fn part2() -> i32 {
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

            *nx.iter().min().unwrap()
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
        use std::cmp::Ordering;
        use std::collections::{HashMap, HashSet};
        const INPUT: &str = include_str!("../input/2018/day6.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2018::day6::part1(), 4_976);
        /// ```
        pub fn part1() -> i32 {
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

            *areas.values().max().unwrap()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2018::day6::part2(), 46_462);
        /// ```
        pub fn part2() -> i32 {
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

            n
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
        use std::collections::{BTreeSet, HashMap};
        const INPUT: &str = include_str!("../input/2018/day7.txt");

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
        /// assert_eq!(advent_of_code::aoc_2018::day7::part2(), 1014);
        /// ```
        pub fn part2() -> u32 {
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

            t
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
        // const INPUT: &str = include_str!("../input/2018/day8.txt");

        pub fn part1() -> String {
            panic!()
        }

        pub fn part2() -> String {
            panic!()
        }
    }
}

pub mod aoc_2019 {
    pub mod day1 {
        const INPUT: &str = include_str!("../input/2019/day1.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day1::part1(), 3_376_997);
        /// ```
        pub fn part1() -> i32 {
            INPUT.lines().map(|x| fuel(x.parse::<i32>().unwrap())).sum()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day1::part2(), 5_062_623);
        /// ```
        pub fn part2() -> i32 {
            INPUT
                .lines()
                .map(|x| total_fuel(x.parse::<i32>().unwrap()))
                .sum()
        }

        fn fuel(x: i32) -> i32 {
            x / 3 - 2
        }

        fn total_fuel(mut x: i32) -> i32 {
            let mut f = 0;

            while fuel(x) > 0 {
                x = fuel(x);
                f += x;
            }

            f
        }
    }

    pub mod day2 {
        const INPUT: &str = include_str!("../input/2019/day2.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day2::part1(), 3_562_624);
        /// ```
        pub fn part1() -> i32 {
            run_program(12, 2)
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day2::part2(), 8_298);
        /// ```
        pub fn part2() -> i32 {
            for n in 0..=99 {
                for v in 0..=99 {
                    if run_program(n, v) == 19_690_720 {
                        return 100 * n + v;
                    }
                }
            }

            panic!()
        }

        fn run_program(noun: i32, verb: i32) -> i32 {
            let mut v = INPUT
                .split(',')
                .map(|x| str::replace(x, "\r\n", "").parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            v[1] = noun;
            v[2] = verb;

            for i in (0..).step_by(4) {
                let j = v[i + 3] as usize;
                match v[i] {
                    99 => break,
                    1 => {
                        v[j] = v[v[i + 1] as usize] + v[v[i + 2] as usize];
                    }
                    2 => {
                        v[j] = v[v[i + 1] as usize] * v[v[i + 2] as usize];
                    }
                    _ => panic!(),
                };
            }

            v[0]
        }
    }

    pub mod day3 {
        use std::cmp::{max, min};
        use std::collections::BTreeSet;
        const INPUT: &str = include_str!("../input/2019/day3.txt");

        type Line = (i32, i32, i32); // HLine: y, x1, x2; VLine: x, y1, y2

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day3::part1(), 209);
        /// ```
        pub fn part1() -> i32 {
            type WireH = BTreeSet<(i32, Line)>;
            type WireV = BTreeSet<(i32, Line)>;

            fn parse_wire(wire: &str) -> (WireH, WireV) {
                let mut pt: (i32, i32) = (0, 0);
                let mut h = BTreeSet::new();
                let mut v = BTreeSet::new();

                for dir in wire.split(',') {
                    let (dir, len) = parse_dir(dir);

                    let (x, y) = pt;
                    pt = travel(pt, dir, len);

                    match dir {
                        "L" => h.insert((y.abs(), (y, x, x - len))),
                        "R" => h.insert((y.abs(), (y, x, x + len))),
                        "U" => v.insert((x.abs(), (x, y, y + len))),
                        "D" => v.insert((x.abs(), (x, y, y - len))),
                        _ => panic!(),
                    };
                }

                (h, v)
            }

            let mut wires = INPUT.lines();
            let (red_h, red_v) = parse_wire(wires.next().unwrap());
            let (black_h, black_v) = parse_wire(wires.next().unwrap());

            let mut dx = BTreeSet::new();

            fn compare(mut dx: BTreeSet<i32>, wire_h: &WireH, wire_v: &WireV) -> BTreeSet<i32> {
                for (_, h) in wire_h {
                    for (_, v) in wire_v {
                        if let Some((x, y)) = cross(*h, *v) {
                            let d = x.abs() + y.abs();
                            if d > 0 {
                                dx.insert(d);
                                break;
                            }
                        }
                    }
                }
                dx
            }

            dx = compare(dx, &red_h, &black_v);
            dx = compare(dx, &black_h, &red_v);

            *dx.iter().next().unwrap()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day3::part2(), 43258);
        /// ```
        pub fn part2() -> i32 {
            type Wire = Vec<(String, Line)>;

            fn parse_wire(wire: &str) -> Wire {
                let mut pt: (i32, i32) = (0, 0);
                let mut w = Vec::new();

                for dir in wire.split(',') {
                    let (dir, len) = parse_dir(dir);

                    let (x, y) = pt;
                    pt = travel(pt, dir, len);

                    let l = match dir {
                        "L" => ("H", (y, x, x - len)),
                        "R" => ("H", (y, x, x + len)),
                        "U" => ("V", (x, y, y + len)),
                        "D" => ("V", (x, y, y - len)),
                        _ => panic!(),
                    };
                    w.push((l.0.to_string(), l.1));
                }

                w
            }

            let mut wires = INPUT.lines();
            let red = parse_wire(wires.next().unwrap());
            let black = parse_wire(wires.next().unwrap());

            let mut dx = BTreeSet::new();

            let mut d_red = 0;
            for (dir_r, r) in &red {
                let mut d_black = 0;
                for (dir_b, b) in &black {
                    if dir_r != dir_b && cross(*r, *b).is_some() {
                        let d = d_red + d_black + (b.0 - r.1).abs() + (r.0 - b.1).abs();
                        if d > 0 {
                            dx.insert(d);
                        }
                    }
                    d_black += (b.1 - b.2).abs();
                }
                d_red += (r.1 - r.2).abs();
            }

            *dx.iter().next().unwrap()
        }

        fn parse_dir(s: &str) -> (&str, i32) {
            let (dir, len) = s.split_at(1);
            (dir, len.parse::<_>().unwrap())
        }

        fn travel(pt: (i32, i32), dir: &str, len: i32) -> (i32, i32) {
            let (x, y) = pt;
            match dir {
                "L" => (x - len, y),
                "R" => (x + len, y),
                "U" => (x, y + len),
                "D" => (x, y - len),
                _ => panic!(),
            }
        }

        fn cross(a: Line, b: Line) -> Option<(i32, i32)> {
            if min(a.1, a.2) < b.0
                && b.0 < max(a.1, a.2)
                && min(b.1, b.2) < a.0
                && a.0 < max(b.1, b.2)
            {
                Some((b.0, a.0))
            } else {
                None
            }
        }
    }

    pub mod day4 {
        const INPUT: &str = include_str!("../input/2019/day4.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day4::part1(), 579);
        /// ```
        pub fn part1() -> i32 {
            let (min, max) = parse_input();

            fn adjacent(x: i32) -> bool {
                digits(x).windows(2).any(|s| s[0] == s[1])
            };

            (min..=max).filter(|&x| adjacent(x) && monotonic(x)).count() as i32
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day4::part2(), 358);
        /// ```
        pub fn part2() -> i32 {
            let (min, max) = parse_input();

            fn adjacent(x: i32) -> bool {
                let d = digits(x);
                (d[0] == d[1] && d[1] != d[2])
                    || (d[5] == d[4] && d[4] != d[3])
                    || d.windows(4)
                        .any(|s| s[0] != s[1] && s[1] == s[2] && s[2] != s[3])
            };

            (min..=max).filter(|&x| adjacent(x) && monotonic(x)).count() as i32
        }

        fn parse_input() -> (i32, i32) {
            let mut iter = INPUT.lines().next().unwrap().split('-');
            let min = iter.next().unwrap().parse::<i32>().unwrap();
            let max = iter.next().unwrap().parse::<i32>().unwrap();
            (min, max)
        }

        fn monotonic(x: i32) -> bool {
            digits(x).windows(2).all(|s| s[0] <= s[1])
        }

        fn digits(x: i32) -> Vec<u32> {
            x.to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<_>()
        }
    }

    pub mod day5 {
        // const INPUT: &str = include_str!("../input/2019/day5.txt");
        pub fn part1() {}
        pub fn part2() {}
    }
}

pub mod aoc_2020 {
    const TABLE_OF_CONTENTS: [[fn() -> String; 2]; 7] = [
        [day1::part1, day1::part2],
        [day2::part1, day2::part2],
        [day3::part1, day3::part2],
        [day4::part1, day4::part2],
        [day5::part1, day5::part2],
        [day6::part1, day6::part2],
        [day7::part1, day7::part2],
        // [day8::part1, day8::part2],
        // [day9::part1, day9::part2],
        // [day10::part1, day10::part2],
    ];

    pub fn main(day: usize, part: usize) {
        println!("{}", TABLE_OF_CONTENTS[day - 1][part - 1]());
    }

    pub mod day1 {
        use std::collections::HashSet;

        const INPUT: &str = include_str!("../input/2020/day1.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day1::part1(), 800_139.to_string());
        /// ```
        pub fn part1() -> String {
            let mut history = HashSet::new();
            for line in INPUT.lines() {
                let x = line.parse::<u32>().unwrap();
                let y = 2020 - x;
                if history.contains(&y) {
                    return (x * y).to_string();
                } else {
                    history.insert(x);
                }
            }
            panic!()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day1::part2(), 59_885_340.to_string());
        /// ```
        pub fn part2() -> String {
            let v = INPUT
                .lines()
                .map(|line| line.parse::<_>().unwrap())
                .collect::<Vec<u32>>();
            for a in &v {
                for b in &v {
                    for c in &v {
                        if a + b + c == 2020 {
                            return (a * b * c).to_string();
                        }
                    }
                }
            }
            panic!()
        }
    }

    pub mod day2 {
        const INPUT: &str = include_str!("../input/2020/day2.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day2::part1(), 655.to_string());
        /// ```
        pub fn part1() -> String {
            INPUT
                .lines()
                .filter(|line| {
                    let (i, j, ch, pw) = parse_line(line);
                    let n = pw.into_iter().filter(|&x| x == ch).count();
                    i <= n && n <= j
                })
                .count()
                .to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day2::part2(), 673.to_string());
        /// ```
        pub fn part2() -> String {
            INPUT
                .lines()
                .filter(|line| {
                    let (i, j, ch, pw) = parse_line(line);
                    (pw[i - 1] == ch) ^ (pw[j - 1] == ch)
                })
                .count()
                .to_string()
        }

        fn parse_line(line: &str) -> (usize, usize, char, Vec<char>) {
            let split = line.split(['-', ' ', ':'].as_ref()).collect::<Vec<_>>();
            let i = split[0].parse::<_>().unwrap();
            let j = split[1].parse::<_>().unwrap();
            let ch = split[2].chars().next().unwrap();
            let pw = split[4].chars().collect::<Vec<_>>();
            (i, j, ch, pw)
        }
    }

    pub mod day3 {
        const INPUT: &str = include_str!("../input/2020/day3.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day3::part1(), 250.to_string());
        /// ```
        pub fn part1() -> String {
            let trees = parse_input();
            travel(&trees, 3, 1).to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day3::part2(), 1_592_662_500.to_string());
        /// ```
        pub fn part2() -> String {
            let trees = parse_input();
            [
                travel(&trees, 1, 1),
                travel(&trees, 3, 1),
                travel(&trees, 5, 1),
                travel(&trees, 7, 1),
                travel(&trees, 1, 2),
            ]
            .iter()
            .product::<u32>()
            .to_string()
        }

        fn travel(trees: &Vec<Vec<char>>, right: usize, down: usize) -> u32 {
            let n_cols = trees[0].len();
            let mut c = 0;
            let mut ans = 0;
            for r in (down..trees.len()).step_by(down) {
                c = (c + right) % n_cols;
                if trees[r][c] == '#' {
                    ans += 1;
                }
            }
            ans
        }

        fn parse_input() -> Vec<Vec<char>> {
            INPUT
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        }
    }

    pub mod day4 {
        use std::collections::HashSet;

        const INPUT: &str = include_str!("../input/2020/day4.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day4::part1(), 213.to_string());
        /// ```
        pub fn part1() -> String {
            let required_fields: HashSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .cloned()
                .collect();

            INPUT
                .split("\r\n\r\n")
                .map(|passport| {
                    let mut fields = HashSet::new();
                    for kv in passport.split_whitespace() {
                        let k = kv.split(':').next().unwrap();
                        fields.insert(k);
                    }
                    fields
                })
                .filter(|fields| fields.is_superset(&required_fields))
                .count()
                .to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day4::part2(), 147.to_string());
        /// ```
        pub fn part2() -> String {
            let required_fields: HashSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .cloned()
                .collect();

            INPUT
                .split("\r\n\r\n")
                .map(|passport| {
                    let mut fields = HashSet::new();
                    for kv in passport.split_whitespace() {
                        let mut kv = kv.split(':');
                        let k = kv.next().unwrap();
                        let v = kv.next().unwrap();
                        if is_valid(k, v) {
                            fields.insert(k);
                        }
                    }
                    fields
                })
                .filter(|fields| fields.is_superset(&required_fields))
                .count()
                .to_string()
        }

        fn is_valid(k: &str, v: &str) -> bool {
            match k {
                "byr" => (1920..=2002).contains(&v.parse().unwrap()),
                "iyr" => (2010..=2020).contains(&v.parse().unwrap()),
                "eyr" => (2020..=2030).contains(&v.parse().unwrap()),
                "hgt" => {
                    let hgt: String = v.chars().filter(char::is_ascii_digit).collect();
                    let unit: String = v.chars().filter(char::is_ascii_alphabetic).collect();
                    match (hgt.parse(), unit.as_str()) {
                        (Ok(h), "cm") => (150..=193).contains(&h),
                        (Ok(h), "in") => (59..=76).contains(&h),
                        _ => false,
                    }
                }
                "hcl" => {
                    let mut hcl = v.chars();
                    hcl.next().unwrap() == '#' && hcl.all(|ch| ch.is_digit(16))
                }
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
                "pid" => v.len() == 9 && v.chars().all(|ch| ch.is_ascii_digit()),
                _ => false,
            }
        }
    }

    pub mod day5 {
        use std::collections::HashSet;

        const INPUT: &str = include_str!("../input/2020/day5.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day5::part1(), 935.to_string());
        /// ```
        pub fn part1() -> String {
            INPUT
                .lines()
                .map(decode)
                .map(seat_id)
                .max()
                .unwrap()
                .to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day5::part2(), 743.to_string());
        /// ```
        pub fn part2() -> String {
            let seat_ids: HashSet<_> = INPUT.lines().map(decode).map(seat_id).collect();
            for r in 1..=126 {
                for c in 0..=7 {
                    let seat = seat_id((r, c));
                    if !seat_ids.contains(&seat)
                        && seat_ids.contains(&(seat - 1))
                        && seat_ids.contains(&(seat + 1))
                    {
                        return seat.to_string();
                    }
                }
            }
            panic!()
        }

        fn seat_id(seat: (u32, u32)) -> u32 {
            let (row, col) = seat;
            8 * row + col
        }

        fn decode(s: &str) -> (u32, u32) {
            let binary = s
                .chars()
                .map(|ch| match ch {
                    'F' | 'L' => '0',
                    'B' | 'R' => '1',
                    _ => panic!(),
                })
                .collect::<String>();
            let row = u32::from_str_radix(&binary[0..=6], 2).unwrap();
            let col = u32::from_str_radix(&binary[7..], 2).unwrap();
            (row, col)
        }
    }

    pub mod day6 {
        use std::collections::HashSet;

        const INPUT: &str = include_str!("../input/2020/day6.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day6::part1(), 6903.to_string());
        /// ```
        pub fn part1() -> String {
            INPUT
                .split("\r\n\r\n")
                .map(|group| {
                    group
                        .chars()
                        .filter(char::is_ascii_alphabetic)
                        .collect::<HashSet<_>>()
                        .len()
                })
                .sum::<usize>()
                .to_string()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day6::part2(), 3_493.to_string());
        /// ```
        pub fn part2() -> String {
            INPUT
                .split("\r\n\r\n")
                .map(|group| {
                    group
                        .lines()
                        .map(|line| line.chars().collect::<HashSet<_>>())
                        .fold(('a'..='z').collect(), |acc, x| &acc & &x)
                        .len()
                })
                .sum::<usize>()
                .to_string()
        }
    }

    pub mod day7 {
        use std::collections::{HashMap, HashSet};

        const INPUT: &str = include_str!("../input/2020/day7.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2020::day7::part1(), 172.to_string());
        /// ```
        pub fn part1() -> String {
            let mut rules: HashMap<_, HashSet<_>> = HashMap::new();
            for rule in INPUT.lines() {
                let mut kv = rule.splitn(2, " bags contain ");
                let k = kv.next().unwrap();
                let v = kv.next().unwrap().split(',').map(bag_color).collect();
                rules.insert(k, v);
            }
            let mut colors = HashSet::new();
            let mut grow = true;
            while grow {
                grow = false;
                for (k, v) in &rules {
                    if v.contains("shiny gold") || !v.is_disjoint(&colors) {
                        if colors.insert(k) {
                            grow = true;
                        }
                    }
                }
            }
            colors.len().to_string()
        }

        pub fn part2() -> String {
            let mut rules: HashMap<_, Vec<_>> = HashMap::new();
            for rule in INPUT.lines() {
                let mut kv = rule.splitn(2, " bags contain ");
                let k = kv.next().unwrap();
                let v = kv
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|b| (bag_count(b), bag_color(b)))
                    .collect();
                rules.insert(k, v);
            }
            let a = count_inner(&rules, "shiny gold");
            let b = count_outer(&rules, "shiny gold");
            println!("inner: {}, outer: {}", a, b);
            "wip".to_string()
        }

        fn count_outer(rules: &HashMap<&str, Vec<(&str, &str)>>, color: &str) -> u32 {
            let bags = rules.get(color).unwrap();
            match bags[..] {
                [("no", "other")] => 1,
                _ => bags
                    .iter()
                    .map(|(n, c)| n.parse::<u32>().unwrap() * count_outer(rules, c))
                    .sum(),
            }
        }

        fn count_inner(rules: &HashMap<&str, Vec<(&str, &str)>>, color: &str) -> u32 {
            let bags = rules.get(color).unwrap();
            match bags[..] {
                [("no", "other")] => 1,
                _ => bags
                    .iter()
                    .map(|(n, c)| n.parse::<u32>().unwrap() * count_inner(rules, c))
                    .sum(),
            }
        }

        fn parse(bags: &str) -> &str {
            bags.rsplitn(2, ' ').last().unwrap().trim()
        }

        fn bag_color(bags: &str) -> &str {
            parse(bags).splitn(2, ' ').last().unwrap()
        }

        fn bag_count(bags: &str) -> &str {
            parse(bags).splitn(2, ' ').next().unwrap()
        }
    }

    // pub mod day8 {
    //     pub fn part1() -> String {
    //         panic!()
    //     }
    //
    //     pub fn part2() -> String {
    //         panic!()
    //     }
    // }
    //
    // pub mod day9 {
    //     pub fn part1() -> String {
    //         panic!()
    //     }
    //
    //     pub fn part2() -> String {
    //         panic!()
    //     }
    // }
    //
    // pub mod day10 {
    //     pub fn part1() -> String {
    //         panic!()
    //     }
    //
    //     pub fn part2() -> String {
    //         panic!()
    //     }
    // }
}
