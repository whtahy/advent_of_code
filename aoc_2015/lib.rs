shared::table_of_contents!();

pub mod day1 {
    shared::input!(1);
    shared::test!(74, 1_795);

    type T = i8;

    fn parse(ch: char) -> T {
        match ch {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        }
    }

    pub fn part1() -> String {
        INPUT.chars().map(parse).sum::<T>().to_string()
    }

    pub fn part2() -> String {
        INPUT
            .chars()
            .map(parse)
            .scan(0, |floor, x| {
                *floor += x;
                Some(*floor)
            })
            .take_while(|&floor| floor != -1)
            .chain([0])
            .count()
            .to_string()
    }
}

pub mod day2 {
    shared::input!(2);
    shared::test!(1_586_300, 3_737_498);

    type T = usize;
    type Dimensions = Vec<T>;

    fn parse(ln: &str) -> Dimensions {
        ln.split('x').flat_map(str::parse).cycle().take(4).collect()
    }

    pub fn part1() -> String {
        let calc = |dims: Dimensions| {
            let iter = dims.windows(2).map(|v| v[0] * v[1]);
            2 * iter.clone().sum::<T>() + iter.min().unwrap()
        };
        INPUT.lines().map(parse).map(calc).sum::<T>().to_string()
    }

    pub fn part2() -> String {
        let calc = |mut dims: Dimensions| -> T {
            dims[..3].sort_unstable();
            let wrap = 2 * (dims[0] + dims[1]);
            wrap + dims[..3].iter().product::<T>()
        };
        INPUT.lines().map(parse).map(calc).sum::<T>().to_string()
    }
}

pub mod day3 {
    shared::input!(3);
    shared::test!(2_572, 2_631);

    use std::collections::HashSet;

    type T = isize;
    type Coord = (T, T);

    pub fn part1() -> String {
        INPUT
            .chars()
            .scan((0, 0), travel)
            .chain([(0, 0)])
            .collect::<HashSet<_>>()
            .len()
            .to_string()
    }

    pub fn part2() -> String {
        let santa = INPUT.chars().step_by(2).scan((0, 0), travel);
        let robo_santa = INPUT.chars().skip(1).step_by(2).scan((0, 0), travel);
        santa
            .chain(robo_santa)
            .chain([(0, 0)])
            .collect::<HashSet<_>>()
            .len()
            .to_string()
    }

    fn travel(loc: &mut Coord, dir: char) -> Option<Coord> {
        match dir {
            '>' => loc.0 += 1,
            '<' => loc.0 -= 1,
            '^' => loc.1 += 1,
            'v' => loc.1 -= 1,
            _ => unreachable!(),
        };
        Some(*loc)
    }
}

pub mod day4 {
    shared::input!(4);
    shared::test!(282_749, 9_962_624);

    use md5::{Digest, Md5};
    use std::sync::{Arc, RwLock};

    pub fn part1() -> String {
        mine(15).to_string()
    }

    pub fn part2() -> String {
        mine(0).to_string()
    }

    fn mine(n: u8) -> usize {
        let ans = Arc::new(RwLock::new(usize::MAX));
        let n_threads = std::thread::available_parallelism().unwrap().get();
        let search = |t: usize| {
            let ans = ans.clone();
            let md5 = Md5::new_with_prefix(INPUT.trim());
            std::thread::spawn(move || {
                for i in (1..).skip(t - 1).step_by(n_threads) {
                    let hash = md5
                        .clone()
                        .chain_update(i.to_string().into_bytes())
                        .finalize();
                    if hash[0] == 0 && hash[1] == 0 && hash[2] <= n {
                        return *ans.write().unwrap() = i;
                    } else if (i - t) % (n_threads * 256) == 0
                        && i > *ans.read().unwrap()
                    {
                        return;
                    }
                }
            })
        };
        for handle in
            (1..=n_threads).map(search).collect::<Vec<_>>().into_iter()
        {
            handle.join().ok();
        }
        Arc::try_unwrap(ans).unwrap().into_inner().unwrap()
    }
}

pub mod day5 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day6 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day7 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day8 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day9 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day10 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day11 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day12 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day13 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day14 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day15 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day16 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day17 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day18 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day19 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day20 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day21 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day22 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day23 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day24 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day25 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}
