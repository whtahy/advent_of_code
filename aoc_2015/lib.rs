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
            .count()
            .checked_add(1)
            .unwrap()
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
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day4 {
    shared::input!();
    shared::test!();

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
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
