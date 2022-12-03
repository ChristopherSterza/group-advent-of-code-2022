pub fn process_part1(input: &str) -> String {
    use std::cmp::max;

    let lines = input.lines();

    let mut max_calories: i32 = 0;
    let mut sum_calories: i32 = 0;

    lines.for_each(|line: &str| match line {
        "" => {
            max_calories = max(max_calories, sum_calories);
            sum_calories = 0;
        }
        _ => sum_calories += line.to_string().parse::<i32>().unwrap(),
    });

    return max_calories.to_string();
}

pub fn process_part2(input: &str) -> String {
    let lines = input.lines();

    let mut max_calories = vec![0, 0, 0];
    let mut sum_calories: i32 = 0;

    lines.for_each(|line: &str| match line {
        "" => {
            let mut to_check: Vec<i32> = vec![sum_calories];
            to_check.extend(max_calories.iter().copied());
            to_check.sort();
            max_calories = to_check[1..4].to_vec();
            sum_calories = 0;
        }
        _ => sum_calories += line.to_string().parse::<i32>().unwrap(),
    });

    // gross way to catch final elf
    let mut to_check: Vec<i32> = vec![sum_calories];
    to_check.extend(max_calories.iter().copied());
    to_check.sort();
    max_calories = to_check[1..4].to_vec();

    return max_calories.iter().sum::<i32>().to_string();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{process_part1, process_part2};

    #[test]
    fn part1_works() {
        let file = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part1(&file);

        assert_eq!(result, "24000");
    }

    #[test]
    fn part2_works() {
        let file = fs::read_to_string("./test_input.txt").unwrap();
        let result = process_part2(&file);

        assert_eq!(result, "45000");
    }
}
