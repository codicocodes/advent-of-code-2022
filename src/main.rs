mod days;

fn main() {
    let file_path = "src/inputs/day04.txt";
    let input = std::fs::read_to_string(file_path).unwrap();
    days::day04::solve(&input);
}
