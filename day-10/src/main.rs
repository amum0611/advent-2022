fn main() {
    
    let mut grid: Vec<Vec<&str>> = vec![vec![]; 7];

    let strength_interval = 40;
    let mut signal_strength = 20;
    let final_signal_strength = 220;
    let mut index: i32 = 1;
    let mut register_x: i32 = 1;
    let mut total_sig_strength = 0;
    let mut signals: Vec<&str> = Vec::new();
    let mut pixel: i32 = 0;

    let file_lines = include_str!("../data/dataset.txt").split("\n");
    for line in file_lines {
        if line != "noop" {
            let temp: Vec<&str> = line.split(" ").collect();
            signals.push(temp[0]);
            signals.push(temp[1]);
        } else {
            signals.push(line);
        }
    }

    for sig in signals {

        let x: usize = (index / 40).abs().try_into().unwrap();
        if (register_x - pixel).abs() <= 1 {
            grid[x].push("#");
        } else {
            grid[x].push(" ");
        }
        pixel += 1;
        pixel %= 40;

        if index == signal_strength && signal_strength <= final_signal_strength {
            println!("During the {}th cycle, register X has the value {}, so the signal strength is = {}", signal_strength, register_x, signal_strength * register_x);
            total_sig_strength = total_sig_strength + (signal_strength * register_x);
            signal_strength += strength_interval;
        }
        match sig {
            "noop" => {

            },
            "addx" => {

            },
            _ => {
                register_x += sig.parse::<i32>().unwrap();
            },
        }
        index += 1;
    }
    println!("\nPart 1: The sum of these six signal strengths: {}", total_sig_strength);

    println!("Part 2:");
    for g in grid {
        println!("{}", g.join(""));
    }
}
