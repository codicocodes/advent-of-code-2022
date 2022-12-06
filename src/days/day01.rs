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

fn calculate_top3_calories(mut calories: Vec<i32>) -> i32 { calories.sort();
    calories.reverse();
    let mut aggregated_calories = 0;
    for idx in 0..3 {
        let elf_calories = calories[idx];
        aggregated_calories += elf_calories;
    }
    return aggregated_calories
}

pub fn solve_a(input: &str) {
    let calories = parse_calories_by_elf(input);
    let max_calories = calculate_max_calories(calories);
    println!("{max_calories}")
}


pub fn solve_b(input: &str) {
    let calories = parse_calories_by_elf(input);
    let top3_calories = calculate_top3_calories(calories);
    println!("{top3_calories}")
}

pub fn solve(input: &str) {
    solve_a(input);
    solve_b(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_max_calories() {
        let calories = parse_calories_by_elf(EXAMPLE_INPUT);
        assert_eq!(calculate_max_calories(calories), 24000);
    }

    #[test]
    fn test_top3_calories() {
        let calories = parse_calories_by_elf(EXAMPLE_INPUT);
        assert_eq!(calculate_top3_calories(calories), 45000);
    }
}
