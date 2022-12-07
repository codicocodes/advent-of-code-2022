use std::collections::HashSet;

pub fn solve(input: &str) {
    let priority = calculate_total_priority(input);
    println!("{priority}");
}

#[derive(Debug)]
enum Casing {
    LowerCase,
    UpperCase,
}

impl Casing {
    fn offset(&self) -> u32 {
        match *self {
            Casing::LowerCase => 96,
            Casing::UpperCase => 38,
        }
    }
}

fn get_item_priority(item_code: char) -> u32 {
    let ascii_value = item_code as u32;
    match item_code {
        _ if item_code.is_lowercase() => ascii_value - Casing::LowerCase.offset(),
        _ if item_code.is_uppercase() => ascii_value - Casing::UpperCase.offset(),
        _ => { unimplemented!() }
    }
}

fn split_suitcase(suitcase: &str) -> (&str, &str) {
    let middle_idx = suitcase.len() / 2;
    return suitcase.split_at(middle_idx)
}

fn get_items_in_both_compartments(suitcase: &str) -> HashSet<char> {
    let (compartment1, compartment_2) = split_suitcase(suitcase);
    let comp_set_1: HashSet<char> = compartment1.chars().collect();
    let comp_set_2: HashSet<char> = compartment_2.chars().collect();
    let mut intersection_set: HashSet<char> = HashSet::from([]);
    for item in comp_set_1 {
        if comp_set_2.contains(&item) {
            intersection_set.insert(item);
        }
    }
    return intersection_set;
}

fn calculate_suitcase_priority(suitcase: &str) -> u32 {
    let duplicate_items = get_items_in_both_compartments(suitcase);
    let mut total_priority = 0;
    for item in duplicate_items {
        total_priority += get_item_priority(item)
    }
    return total_priority
}

fn calculate_total_priority(input: &str) -> u32 {
    let suitcases = str::trim(input).split("\n");
    let mut total_priority = 0;
    for suitcase in suitcases {
        total_priority += calculate_suitcase_priority(suitcase)
    }
    return total_priority
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_item_priority() {
        assert_eq!(get_item_priority('a'), 1);
        assert_eq!(get_item_priority('z'), 26);
        assert_eq!(get_item_priority('A'), 27);
        assert_eq!(get_item_priority('Z'), 52);
    }

    #[test]
    fn test_split_suitecase() {
        let compartments = split_suitcase("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(compartments.0, "vJrwpWtwJgWr");
        assert_eq!(compartments.1, "hcsFMMfFFhFp");
    }

    #[test]
    fn test_get_items_in_both_compartments() {
        assert_eq!(get_items_in_both_compartments("vJrwpWtwJgWrhcsFMMfFFhFp").contains(&'p'), true);
        assert_eq!(get_items_in_both_compartments("vJrwpWtwJgWrhcsFMMfFFhFp").contains(&'F'), false);
        assert_eq!(get_items_in_both_compartments("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").contains(&'L'), true);
        assert_eq!(get_items_in_both_compartments("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").contains(&'S'), false);
        assert_eq!(get_items_in_both_compartments("PmmdzqPrVvPwwTWBwg").contains(&'P'), true);
        assert_eq!(get_items_in_both_compartments("PmmdzqPrVvPwwTWBwg").contains(&'g'), false);
    }

    #[test]
    fn test_calculate_suitcase_priority() {
        assert_eq!(calculate_suitcase_priority("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
    }

    #[test]
    fn test_calculate_total_priority() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(calculate_total_priority(input), 157);
    }
}
