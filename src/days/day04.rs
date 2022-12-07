pub fn solve(input: &str) {
    let count = compare_all_assignment_pairs(input);
    println!("solve_a={count}");
    let count = compare_all_assignment_pair_overlap(input);
    println!("solve_b={count}");
}

#[derive(Debug, Clone, Copy)]
struct AssignedSections {
    first: i32,
    last: i32,
}

impl AssignedSections {
    fn contains(&self, sections: AssignedSections) -> bool {
        let contains_first = sections.first >= self.first;
        let contains_last = sections.last <= self.last;
        return contains_first && contains_last;
    }

    fn overlaps(&self, sections: AssignedSections) -> bool {
        let first_after_first_and_first_before_last = sections.first >= self.first && sections.first <= self.last;
        let last_after_last_and_first_before_last = sections.last <= self.last && sections.last >= self.first;
        let contains = self.contains(sections);
        let contained = sections.contains(*self);
        return contains || contained || first_after_first_and_first_before_last || last_after_last_and_first_before_last
    }
}

fn new_section_from_string(section_input: &str) -> AssignedSections {
    let (first, last) = section_input.split_once("-").unwrap();
    return AssignedSections{
        first: first.parse().unwrap(),
        last: last.parse().unwrap(),
    }
}

fn compare_assignment_pair(section_input: &str) -> bool {
    let (first, second) = section_input.split_once(",").unwrap();
    let first_section = new_section_from_string(first);
    let second_section = new_section_from_string(second);
    let first_contains_second = first_section.contains(second_section);
    let second_contains_first = second_section.contains(first_section);
    return  first_contains_second || second_contains_first;
}

fn compare_assignment_pair_overlap(section_input: &str) -> bool {
    let (first, second) = section_input.split_once(",").unwrap();
    let first_section = new_section_from_string(first);
    let second_section = new_section_from_string(second);
    return first_section.overlaps(second_section);
}

fn compare_all_assignment_pair_overlap(pairs_input: &str) -> i32 {
    let pairs = pairs_input.trim().split("\n");
    let mut count = 0;
    for pair in pairs {
        let overlaps = compare_assignment_pair_overlap(pair);
        count += overlaps as i32;
    }
    return count;
}

fn compare_all_assignment_pairs(pairs_input: &str) -> i32 {
    let pairs = pairs_input.trim().split("\n");
    let mut count = 0;
    for pair in pairs {
        count += compare_assignment_pair(pair) as i32
    }
    return count;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assigned_sections_contains_true() {
        let contained = AssignedSections{first: 2, last: 8}.contains(
            AssignedSections{
                first: 3,
                last: 7,
            }
        );
        assert_eq!(contained, true);

        let contained = AssignedSections{first: 2, last: 8}.contains(
            AssignedSections{
                first: 8,
                last: 8,
            }
        );
        assert_eq!(contained, true);
    }

    #[test]
    fn test_assigned_sections_overlaps_true() {
        let section1 = new_section_from_string("2-4");
        let section2 = new_section_from_string("4-5");
        assert_eq!(section1.overlaps(section2), true);
        assert_eq!(section2.overlaps(section1), true);

        let section1 = new_section_from_string("2-4");
        let section2 = new_section_from_string("1-5");
        assert_eq!(section1.overlaps(section2), true);
        assert_eq!(section2.overlaps(section1), true);
    }

    #[test]
    fn test_assigned_sections_overlaps_false() {
        let section1 = new_section_from_string("2-4");
        let section2 = new_section_from_string("6-8");
        assert_eq!(section1.overlaps(section2), false);
        assert_eq!(section2.overlaps(section1), false);
    }

    #[test]
    fn test_assigned_sections_contains_false() {
        let contained = AssignedSections{first: 2, last: 6}.contains(
            AssignedSections{
                first: 3,
                last: 7,
            }
        );
        assert_eq!(contained, false)
    }

    #[test]
    fn test_new_section_from_string() {
        let sections = new_section_from_string("2-4");
        assert_eq!(sections.first, 2);
        assert_eq!(sections.last, 4);
    }

    #[test]
    fn test_compare_assignment_pair() {
        assert_eq!(compare_assignment_pair("2-4,6-8"), false);
        assert_eq!(compare_assignment_pair("2-8,3-7"), true);
        assert_eq!(compare_assignment_pair("6-6,4-6"), true);
        assert_eq!(compare_assignment_pair("2-6,4-8"), false);
    }

    #[test]
    fn test_compare_assignment_pair_overlap() {
        assert_eq!(compare_assignment_pair_overlap("2-4,6-8"), false);
        assert_eq!(compare_assignment_pair_overlap("2-8,3-7"), true);
        assert_eq!(compare_assignment_pair_overlap("6-6,4-6"), true);
        assert_eq!(compare_assignment_pair_overlap("2-6,4-8"), true);
        assert_eq!(compare_assignment_pair_overlap("4-8,2-6"), true);
        assert_eq!(compare_assignment_pair_overlap("4-8,2-10"), true);
    }

    #[test]
    fn test_compare_all_assignment_pair() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(compare_all_assignment_pairs(input), 2);
    }

    #[test]
    fn test_compare_all_assignment_pair_overlap() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(compare_all_assignment_pair_overlap(input), 4);
    }
}
