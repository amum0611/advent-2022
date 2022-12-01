use std::str::Split;

fn main() {

    let mut calories: Vec<u32> = read_dataset_by_calories();
    
    calories.sort();

    println!("Part 1 Answer: {}" , calories.iter().last().unwrap());
    println!("Part 2 Answer: {}" , calories.iter().rev().take(3).sum::<u32>());
}

fn read_dataset_by_calories() -> Vec<u32> {

    let mut calories = vec![0];
    let file_lines: Split<&str> = include_str!("../data/dataset.txt").split("\n\n");
    for line in file_lines {
        calories.push(line.split("\n").map(|each| each.parse::<u32>().unwrap()).sum());
    }

    calories
}
