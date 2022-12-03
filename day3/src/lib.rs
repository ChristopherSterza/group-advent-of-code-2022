use itertools::Itertools;

pub fn process_part1(input: &str) -> String {
    let packs = input.lines();

    let res: u32 = packs
        .map(|pack| {
            let half = &pack.chars().count() / 2;
            let (left, right) = pack.split_at(half);

            let score: u32 = left
                .chars()
                .unique()
                .map(|c| {
                    if !right.contains(c) {
                        return 0;
                    }

                    if c.is_ascii_lowercase() {
                        return c as u32 - 96;
                    } else if c.is_ascii_uppercase() {
                        return c as u32 - 64 + 26;
                    } else {
                        return 0;
                    }
                })
                .sum();

            return score;
        })
        .sum();

    return res.to_string();
}

pub fn process_part2(input: &str) -> String {
    let _lines = input.lines();
    let lines = _lines.map(|line| line.to_string());
    let groups = lines.into_iter().chunks(3);

    let ans: u32 = groups
        .into_iter()
        .map(|g| {
            let badge = g
                .reduce(|acc, item| acc.chars().unique().filter(|c| item.contains(*c)).collect())
                .unwrap()
                .chars()
                .nth(0)
                .unwrap();

            if badge.is_ascii_lowercase() {
                return badge as u32 - 96;
            } else if badge.is_ascii_uppercase() {
                return badge as u32 - 64 + 26;
            } else {
                return 0;
            }
        })
        .sum();

    return ans.to_string();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{process_part1, process_part2};

    #[test]
    fn part1_works() {
        let file = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part1(&file);

        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let file = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part2(&file);

        assert_eq!(result, "70");
    }
}
