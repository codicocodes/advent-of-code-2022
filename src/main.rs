mod days;

fn main() {
    let file_path = "src/inputs/day03.txt";
    let input = std::fs::read_to_string(file_path).unwrap();
    days::day03::solve(&input);
}
