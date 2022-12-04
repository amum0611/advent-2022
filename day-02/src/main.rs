fn main() {
    let mut part1_score: u32 = 0;
    let mut part2_score: u32 = 0;
    // println!("HEHE: {}", read_dataset());
    
    let file_lines = include_str!("../data/dataset.txt").split("\n");
    for line in file_lines {
        match line {
            "A X" => {
                part1_score += 4;
                part2_score += 3;
            },
            "A Y" => {
                part1_score += 8;
                part2_score += 4;
            },
            "A Z" => {
                part1_score += 3;
                part2_score += 8;
            },

            "B X" => {
                part1_score += 1;
                part2_score += 1;
            },
            "B Y" => {
                part1_score += 5;
                part2_score += 5;
            },
            "B Z" => {
                part1_score += 9;
                part2_score += 9;
            },

            "C X" => {
                part1_score += 7;
                part2_score += 2;
            },
            "C Y" => {
                part1_score += 2;
                part2_score += 6;
            },
            "C Z" => {
                part1_score += 6;
                part2_score += 7;
            },
            _ => {
                part1_score += 0;
                part2_score += 0;
            }
        }
    }

    println!("Total Score (Part 1): {}", part1_score);
    println!("Total Score (Part 2): {}", part2_score);
}