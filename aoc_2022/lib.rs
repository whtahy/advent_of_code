shared::table_of_contents!();

pub mod day1 {
    use std::collections::BinaryHeap;

    shared::input!(1);
    shared::test!(72_602, 207_410); // examples: 24_000, 45_000

    type T = usize;

    fn inventory_sum(s: &str) -> T {
        s.lines().map(|ln| ln.parse::<T>().unwrap()).sum()
    }

    pub fn part1() -> String {
        INPUT
            .split("\r\n\r\n")
            .map(inventory_sum)
            .max()
            .unwrap()
            .to_string()
    }

    pub fn part2() -> String {
        INPUT
            .split("\r\n\r\n")
            .map(inventory_sum)
            .collect::<BinaryHeap<_>>()
            .iter()
            .take(3)
            .sum::<T>()
            .to_string()
    }
}

pub mod day2 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day3 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day4 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day5 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day6 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day7 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day8 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day9 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day10 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day11 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day12 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day13 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day14 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day15 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day16 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day17 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day18 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day19 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day20 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day21 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day22 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day23 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day24 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}

pub mod day25 {
    shared::input!();
    shared::test!(); // examples:

    pub fn part1() -> String {
        todo!()
    }

    pub fn part2() -> String {
        todo!()
    }
}
