pub fn process_part1(input: &str) -> String {
    let lines = input.lines();

    let res: u32 = lines
        .map(|line| {
            let (_range1, _range2) = line.split_once(',').unwrap();
            let (start1_str, end1_str) = _range1.split_once('-').unwrap();
            let (start2_str, end2_str) = _range2.split_once('-').unwrap();

            let (start1, end1): (u32, u32) =
                (start1_str.parse().unwrap(), end1_str.parse().unwrap());
            let (start2, end2): (u32, u32) =
                (start2_str.parse().unwrap(), end2_str.parse().unwrap());

            let range1: Vec<u32> = (start1..end1 + 1).collect();
            let range2: Vec<u32> = (start2..end2 + 1).collect();

            return if range1.iter().all(|n| range2.contains(&n))
                || range2.iter().all(|n| range1.contains(&n))
            {
                1
            } else {
                0
            };
        })
        .sum();

    return res.to_string();
}

pub fn process_part2(input: &str) -> String {
    let lines = input.lines();

    let res: u32 = lines
        .map(|line| {
            let (_range1, _range2) = line.split_once(',').unwrap();
            let (start1_str, end1_str) = _range1.split_once('-').unwrap();
            let (start2_str, end2_str) = _range2.split_once('-').unwrap();

            let (start1, end1): (u32, u32) =
                (start1_str.parse().unwrap(), end1_str.parse().unwrap());
            let (start2, end2): (u32, u32) =
                (start2_str.parse().unwrap(), end2_str.parse().unwrap());

            let range1: Vec<u32> = (start1..end1 + 1).collect();
            let range2: Vec<u32> = (start2..end2 + 1).collect();

            return if range1.contains(&start2) || range2.contains(&start1) {
                1
            } else {
                0
            };
        })
        .sum();

    return res.to_string();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{process_part1, process_part2};

    #[test]
    fn part1_works() {
        let file = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part1(&file);

        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let file = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part2(&file);

        assert_eq!(result, "4");
    }
}
