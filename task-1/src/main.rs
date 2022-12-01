use std::fs;
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Wrong file name!");
    let mut all_calories: Vec<i32> = Vec::new();

    for line in input.split("\n\n") {
        let foods: Vec<&str> = line.split("\n").collect();
        all_calories.push(foods.iter().map(|x| x.parse::<i32>().unwrap_or(0)).sum());
    }

    all_calories.sort();
    println!("Task 1: Highest calories carried: {}", all_calories.last().copied().unwrap());
    println!("Task 2: Three highest carried calories: {}", all_calories.iter().rev().take(3).sum::<i32>());
}
