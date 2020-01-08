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
        const INPUT: &str = include_str!("./2018/day1.txt");

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
        const INPUT: &str = include_str!("./2018/day2.txt");

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
        /// assert_eq!(advent_of_code::aoc_2018::day2::part2(), "kbqwtcvzhmhpoelrnaxydifyb");
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
        const INPUT: &str = include_str!("./2018/day3.txt");

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
        const INPUT: &str = include_str!("./2018/day4.txt");

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
        const INPUT: &str = include_str!("./2018/day5.txt");

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
        const INPUT: &str = include_str!("./2018/day6.txt");

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
        const INPUT: &str = include_str!("./2018/day7.txt");

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
        // const INPUT: &str = include_str!("./2018/day8.txt");

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
        const INPUT: &str = include_str!("./2019/day1.txt");

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
        const INPUT: &str = include_str!("./2019/day2.txt");

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
                // println!("{:?}", v);
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
        use std::collections::BTreeSet;
        const INPUT: &str = include_str!("./2019/day3.txt");

        type Line = (i32, i32, i32);

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
                        "L" => h.insert((y.abs(), (y, x - len, x))),
                        "R" => h.insert((y.abs(), (y, x, x + len))),
                        "U" => v.insert((x.abs(), (x, y, y + len))),
                        "D" => v.insert((x.abs(), (x, y - len, y))),
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
        /// assert_eq!(advent_of_code::aoc_2019::day3::part2(), );
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
                        "L" => ("H", (y, x - len, x)),
                        "R" => ("H", (y, x, x + len)),
                        "U" => ("V", (x, y, y + len)),
                        "D" => ("V", (x, y - len, y)),
                        _ => panic!(),
                    };
                    w.push((l.0.to_string(), l.1));
                }

                w
            }

            let mut wires = INPUT.lines();
            let red = parse_wire(wires.next().unwrap());
            let black = parse_wire(wires.next().unwrap());

            let dx = BTreeSet::new();

            let d_red = 0;
            for (dir_r, r) in &red {
                let d_black = 0;
                for (dir_b, b) in &black {
                    // if dir_r != dir_b
                    // if let cross(r,b)
                    // dx.push(d_red + d_black + d_cross)
                }
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
            if (a.1..=a.2).contains(&b.0) && (b.1..=b.2).contains(&a.0) {
                Some((b.0, a.0))
            } else {
                None
            }
        }
    }

    pub mod day4 {
        // const INPUT: &str = include_str!("./2019/day4.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day4::part1(), );
        /// ```
        pub fn part1() -> i32 {
            panic!()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day4::part2(), );
        /// ```
        pub fn part2() -> i32 {
            panic!()
        }
    }

    pub mod day5 {
        // const INPUT: &str = include_str!("./2019/day5.txt");

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day5::part1(), );
        /// ```
        pub fn part1() -> i32 {
            panic!()
        }

        /// ```
        /// assert_eq!(advent_of_code::aoc_2019::day5::part2(), );
        /// ```
        pub fn part2() -> i32 {
            panic!()
        }
    }
}
