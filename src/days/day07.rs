use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

fn solve_a(input: &str) {
    let root = parse(input);

    // problem a
    let mut stack_a = vec![Rc::clone(&root)];
    let mut sum_small_dirs = 0;
    while let Some(dir) = stack_a.pop() {
        for subdir in dir.children.borrow().values() {
            stack_a.push(Rc::clone(subdir));
        }
        let size = dir.get_size();
        if size <= 100000 {
            sum_small_dirs += size
        }
    }
    println!("sum_small_dirs={sum_small_dirs}");
}

fn solve_b(input: &str) {
    let root_b = parse(input);

    let used_space = root_b.get_size();
    let available_space = 70000000 - used_space;
    let required_extra = 30000000 - available_space;

    let mut stack_b = vec![Rc::clone(&root_b)];

    let mut min_deletable_size = i128::MAX;

    while let Some(dir) = stack_b.pop() {
        for subdir in dir.children.borrow().values() {
            stack_b.push(Rc::clone(subdir));
        }
        let size = dir.get_size();
        if size >= required_extra {
            min_deletable_size = min_deletable_size.min(size);
        }
    }
    println!("min_deletable_size={min_deletable_size}");
}

pub fn solve(input: &str) {
    solve_a(input);
    solve_b(input);
}

fn parse(input: &str) -> Rc<Node> {
    let root = Node::new(0, None);
    let mut cwd = Rc::clone(&root);
    for line in input.trim().split("\n") {
        let words: Vec<&str> = line.split(" ").collect();
        match (words[0], words[1]) {
            ("$", "ls") => {},
            ("$", "cd") => {
                match words[2] {
                    "/" => cwd = Rc::clone(&root),
                    ".." => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                    dirname => {
                        let newdir = cwd.children.borrow().get(dirname).unwrap().clone();
                        cwd = newdir;
                    },
                }
            },
            ("dir", name) => {
                let node = Node::new(0, Some(cwd.clone()));
                cwd.children.borrow_mut().insert(name.to_string(), node);
            }
            (size_input, _name) => {
                let size: i128 = size_input.parse().unwrap();
                *cwd.size.borrow_mut() += size;
            },
        }
    }
    return root
}

#[derive(Debug)]
struct Node {
    size: RefCell<i128>,
    parent: Option<Rc<Node>>,
    children: RefCell<HashMap<String, Rc<Node>>>
}

impl Node {
    fn new(size: i128, parent: Option<Rc<Node>>) -> Rc<Node> {
        return Rc::new(Node{size: RefCell::new(size), parent, children: RefCell::new(HashMap::new())})
    }
    fn get_size(&self) -> i128 {
        *self.size.borrow()
            + self.children.borrow().values().fold(
                0, |a, b| a + b.get_size()
            )
    }
}
