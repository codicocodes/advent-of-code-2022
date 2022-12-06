mod days;

fn main() {
    let file_path = "src/inputs/day01.txt";
    let input = std::fs::read_to_string(file_path).unwrap();
    let result = days::day01::solve(&input);
    println!("{result}");
}
