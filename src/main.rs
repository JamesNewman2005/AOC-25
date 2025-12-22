use std::fs;

mod days;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // DAY 1 -----------------------------------------------------------
    let input: String = fs::read_to_string("input_files/day01.txt")?;

    let p1_result: i32 = days::day01::solve(&input);
    let p2_result: i32 = days::day01::solve_p2(&input);

    println!("--------------- DAY 1 ---------------");
    println!("Answer For Part 1: {}", p1_result);
    println!("Answer For Part 2: {}", p2_result);     

    Ok(())
}
