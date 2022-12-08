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

struct Queue<T> {
  queue: Vec<T>,
}

impl<T> Queue<T> {
  fn new() -> Self {
    Queue { queue: Vec::new() }
  }
  fn enqueue(&mut self, item: T) {
      self.queue.push(item)
  }
  fn dequeue(&mut self) -> T {
      self.queue.remove(0)
  }
  fn length(&self) -> usize {
      self.queue.len()
  }
}

fn find_message(codes: &[u8], unique_length: usize) -> usize {
    let mut q: Queue<char> = Queue::new();
    let mut res = 0;
    for code in codes {
        let c = *code as char;
        if q.length() == unique_length {
            q.dequeue();
        }
        q.enqueue(c);
        res += 1;
        let hash_set: HashSet<char> = HashSet::from_iter(q.queue.clone());
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
