// GOAL: detect a start-of-packet marker in the datastream
// the start of a packet is indicated by a sequence of four characters that are all different.
// it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

use std::collections::HashSet;

pub fn solve(input: &str) {
    let res_a = solve_a(input);
    println!("a={res_a}");
    let res_b = solve_b(input);
    println!("b={res_b}");
}

fn find_message(codes: &[u8], unique_length: usize) -> usize {
    let mut q: Vec<char> = vec![];
    let mut res = 0;
    for code in codes {
        let c = *code as char;
        if q.len() == unique_length {
            q.remove(0);
        }
        q.push(c);
        res += 1;
        let hash_set: HashSet<char> = HashSet::from_iter(q.clone());
        if hash_set.len() == unique_length {
            return res
        }
    }
    unreachable!()
}

fn solve_a(input: &str) -> usize {
    let codes = input.as_bytes();
    return find_message(codes, 4)
}

fn solve_b(input: &str) -> usize {
    let codes = input.as_bytes();
    return find_message(codes, 14);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_a("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_b("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_b("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
