pub mod day1 {
    use std::collections::HashSet;

    const INPUT: &str = include_str!("./2018/day1.txt");

    /// ```
    /// assert_eq!(advent_of_code::day1::part1(), 533);
    /// ```
    pub fn part1() -> i32 {
        INPUT.lines().map(|x| x.parse::<i32>().unwrap()).sum()
    }

    /// ```
    /// assert_eq!(advent_of_code::day1::part2(), 73_272);
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
    /// assert_eq!(advent_of_code::day2::part1(), 7_134);
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
    /// assert_eq!(advent_of_code::day2::part2(), "kbqwtcvzhmhpoelrnaxydifyb");
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
    /// assert_eq!(advent_of_code::day3::part1(), 112_378);
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
    /// assert_eq!(advent_of_code::day3::part2(), 603);
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

    /// ```
    /// assert_eq!(advent_of_code::day4::part1(), 131_469);
    /// ```
    pub fn part1() -> i32 {
        let m = parse_input();
        let g = m.keys().max_by_key(|k| m[k].values().sum::<i32>()).unwrap();
        let t = m[&g].keys().max_by_key(|k| m[g][k]).unwrap();
        g * t
    }

    /// ```
    /// assert_eq!(advent_of_code::day4::part2(), 96_951);
    /// ```
    pub fn part2() -> i32 {
        let m = parse_input();
        let g = m.keys().max_by_key(|k| m[k].values().max()).unwrap();
        let t = m[&g].keys().max_by_key(|k| m[&g][k]).unwrap();
        g * t
    }
}

pub mod day5 {
    use std::collections::BTreeSet;
    use std::iter::FromIterator;

    const INPUT: &str = include_str!("./2018/day5.txt");

    pub fn part1() -> i32 {
        let v = INPUT.chars().collect::<Vec<_>>();
        let ix = 0..(v.len());

        let mut h = BTreeSet::<usize>::from_iter(ix.clone());

        let mut b = true;
        for i in ix.cycle() {
            if h.contains(&i) {
                let mut j = i + 1;

                loop {
                    if j >= v.len() {
                        break;
                    } else if !h.contains(&j) {
                        j += 1;
                    } else {
                        break;
                    }
                }

                if j < v.len()
                    && v[i] != v[j]
                    && v[i].to_ascii_lowercase() == v[j].to_ascii_lowercase()
                {
                    h.remove(&i);
                    h.remove(&j);
                    b = false;
                }
            }

            if &i == h.iter().next_back().unwrap() {
                if b {
                    break;
                } else {
                    b = true;
                }
            }
        }

        v.iter()
            .enumerate()
            .filter(|(i, _)| h.contains(i))
            .map(|(_, x)| x)
            .count() as i32
    }

    pub fn part2() {
        panic!()
    }
}
