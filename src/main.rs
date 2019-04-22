pub mod day1 {
    use std::collections::HashSet;
    use std::fs;

    pub fn part1() -> i32 {
        let s = fs::read_to_string("./src/2018/day1.txt").unwrap();
        s.lines().map(|x| x.parse::<i32>().unwrap()).sum()
    }

    pub fn part2() -> i32 {
        let s = fs::read_to_string("./src/2018/day1.txt").unwrap();
        let mut fx = HashSet::new();
        let mut f = 0;
        fx.insert(f);
        loop {
            for l in s.lines() {
                let x = l.parse::<i32>().unwrap();
                f += x;
                if fx.contains(&f) {
                    return f;
                } else {
                    fx.insert(f);
                }
            }
        }
    }
}

pub mod day2 {
    use std::collections::HashMap;
    use std::fs;

    fn count(s: &str) -> HashMap<char, i32> {
        let mut m = HashMap::new();
        for c in s.chars() {
            m.insert(c, m.get(&c).unwrap_or(&0) + 1);
        }
        m
    }

    pub fn part1() -> i32 {
        let s = fs::read_to_string("./src/2018/day2.txt").unwrap();
        let mut x2 = 0;
        let mut x3 = 0;
        for l in s.lines() {
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
}

fn main() {
    // println!("{}", day1::part1());
}
