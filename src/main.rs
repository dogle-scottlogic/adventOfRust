mod day_one;
fn main() {
    let answer = day_one::solve_part_one();
    print!("Part One: {}\n\n", answer);

    let answer_two = day_one::solve_part_two();
    print!("Part Two: {}", answer_two)
}
