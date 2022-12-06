use std::cmp::max;

pub fn solve(input: &str) -> i32 {
    let split_inputs = str::trim(input).split("\n\n");
    let mut max_calories = 0;
    for elf in split_inputs {
        let mut elf_calories = 0;
        for food in elf.split("\n") {
            let food_calories = food.parse::<i32>().unwrap();
            elf_calories += food_calories;
        }
        max_calories = max(max_calories, elf_calories);
    }
    return max_calories;
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

        assert_eq!(solve(input), 24000);
    }
}
