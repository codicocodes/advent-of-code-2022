use std::cmp::max;

fn parse_calories_by_elf(input: &str) -> Vec<i32> {
    let split_inputs = str::trim(input).split("\n\n");
    let mut calories: Vec<i32> = vec![];
    for elf in split_inputs {
        let mut elf_calories = 0;
        for food in elf.split("\n") {
            let food_calories = food.parse::<i32>().unwrap();
            elf_calories += food_calories;
        }
        calories.push(elf_calories)
    }
    return calories;
}

fn calculate_max_calories(calories: Vec<i32>) -> i32 {
    let mut max_calories = 0;
    for elf_calories in calories {
        max_calories = max(max_calories, elf_calories);
    }
    return max_calories
}

pub fn solve(input: &str) -> i32 {
    let calories = parse_calories_by_elf(input);
    return calculate_max_calories(calories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_calories() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let calories = parse_calories_by_elf(input);
        assert_eq!(calculate_max_calories(calories), 24000);
    }
}
