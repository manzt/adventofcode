use anyhow::Result;

mod day01;
mod day02;

fn main() -> Result<()> {
    if let Some(arg) = std::env::args().nth(1) {
        match arg.as_str() {
            "day01" => {
                day01::part1(read_input(1));
                day01::part2(read_input(1));
            }
            "day02" => {
                day02::part1(read_input(2))?;
                day02::part2(read_input(2))?;
            }
            _ => panic!("unknown day"),
        }
    }
    Ok(())
}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
}
