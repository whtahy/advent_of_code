pub mod day1 {
    use std::collections::HashSet;

    const INPUT: &str = include_str!("./2018/day1.txt");

    pub fn part1() -> i32 {
        INPUT.lines().map(|x| x.parse::<i32>().unwrap()).sum()
    }

    pub fn part2() -> i32 {
        let mut fx = HashSet::new();
        let mut f = 0;
        fx.insert(f);

        for l in INPUT.lines() {
            let x = l.parse::<i32>().unwrap();
            f += x;
            if fx.contains(&f) {
                return f;
            } else {
                fx.insert(f);
            }
        }

        fx.remove(&0);

        let offset = f;
        for i in 1.. {
            for v in &fx {
                let c = v + i * offset;
                if fx.contains(&c) {
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

    pub fn part1() -> i32 {
        fn count(s: &str) -> HashMap<char, i32> {
            let mut m = HashMap::new();
            for c in s.chars() {
                m.insert(c, m.get(&c).unwrap_or(&0) + 1);
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

    fn parse(s: &str) -> (&str, i32, &str) {
        let v = s.split(&[' ', ':', ']'][..]).collect::<Vec<_>>();
        (v[4], v[2].parse().unwrap(), v[5])
    }

    pub fn part1() -> i32 {
        let mut v = INPUT.lines().collect::<Vec<_>>();
        v.sort();

        let mut hm_t = HashMap::new();
        let mut t0_i = 0;
        let mut id_i = 0;
        for ln in v.iter() {
            let (s, t, i) = parse(ln);
            if s == "Guard" {
                id_i = i.split('#').collect::<Vec<_>>()[1].parse().unwrap();;
                hm_t.insert(id_i, 0);
            } else if s == "falls" {
                t0_i = t;
            } else if s == "wakes" {
                hm_t.insert(id_i, hm_t[&id_i] + t - t0_i);
            } else {
                panic!()
            }
        }
        let id_max = *hm_t
            .keys()
            .max_by(|x, y| hm_t.get(x).cmp(&hm_t.get(y)))
            .unwrap();

        let mut hm_m = HashMap::new();
        for l in v {
            let (s, t, i) = parse(l);
            if s == "Guard" {
                id_i = i.split('#').collect::<Vec<_>>()[1].parse().unwrap();
            } else if s == "falls" && id_i == id_max {
                t0_i = t;
            } else if s == "wakes" && id_i == id_max {
                for x in t0_i..t {
                    hm_m.insert(x, hm_m.get(&x).unwrap_or(&0) + 1);
                }
            }
        }

        let m_max = hm_m
            .keys()
            .max_by(|x, y| hm_m.get(x).cmp(&hm_m.get(y)))
            .unwrap();

        println!(
            "{}->{}={}",
            id_max,
            hm_t[&id_max],
            hm_t.values().max().unwrap()
        );

        println!("id={}, min={}", id_max, m_max);
        id_max * m_max
    }

    pub fn part2() {
        panic!()
    }
}

fn main() {
    // println!("{}", day1::part1());
    // println!("{}", day1::part2());
    // println!("{}", day2::part1());
    // println!("{}", day2::part2());
    // println!("{}", day3::part1());
    // println!("{}", day3::part2());

    println!("{}", day4::part1());
}
