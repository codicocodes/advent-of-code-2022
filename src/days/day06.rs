// GOAL: detect a start-of-packet marker in the datastream
// the start of a packet is indicated by a sequence of four characters that are all different.
// it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

use std::collections::HashSet;

pub fn solve(input: &str) {
    let res_a = solve_a(input);
    println!("{res_a}");
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

fn solve_a(input: &str) -> i32 {
    let codes = input.as_bytes();
    let mut q: Queue<char> = Queue::new();
    let mut res = 0;
    for code in codes {
        let c = *code as char;
        if q.length() == 4 {
            q.dequeue();
        }
        q.enqueue(c);
        res += 1;
        let hash_set: HashSet<char> = HashSet::from_iter(q.queue.clone());
        if hash_set.len() == 4 {
            return res
        }
    }
    return -1
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
}
