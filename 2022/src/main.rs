mod day01;

fn main() {
    day01::part1(read_input(1));
    day01::part2(read_input(1));
}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
}
