

fn main() {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    let start_position = (500, 500);
    let mut head_position = start_position;
    let mut tail_position = start_position;

    let file_lines = include_str!("../data/dataset.txt").split("\n");
    for line in file_lines {
        let temp: Vec<&str> = line.split(" ").collect();
        let command = temp[0];
        let steps = temp[1].parse::<usize>().unwrap();

        walk(&mut grid, &mut head_position, &mut tail_position, command, steps);
        // println!("Command: {} | Steps: {}", command, steps);
    }
    
    let mut visits = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i][j] > 0 {
                visits += 1;
            }
        }
    }

    println!("Part 1: # of positions the tail visited at least once: {}", visits);
}

fn walk(grid: &mut Vec<Vec<usize>>, head_position: &mut (usize, usize), tail_position: &mut (usize, usize), command: &str, steps: usize) {

    if steps == 0 {
        return;
    }

    let mut previous_head_position = (0, 0);
    previous_head_position.0 = head_position.0;
    previous_head_position.1 = head_position.1;
    
    match command {
        "R" => {
            head_position.0 += 1;
        }, 
        "L" => {
            head_position.0 -= 1;
        }, 
        "U" => {
            head_position.1 -= 1;
        }, 
        "D" => {
            head_position.1 += 1;
        }, 
        _ => {
            println!("Invalid Command: {}", command);
        },
    }

    if should_tail_move(head_position, tail_position) {
        tail_position.0 = previous_head_position.0;
        tail_position.1 = previous_head_position.1;
        grid[tail_position.0][tail_position.1] += 1;
    }
    walk(grid, head_position, tail_position, command, steps - 1);
}

fn should_tail_move (head_position: &mut (usize, usize), tail_position: &mut (usize, usize)) -> bool {
    
    //overlaps
    if head_position.0 == tail_position.0 && head_position.1 == tail_position.1 {
        return false;
    }

    //X varies
    if head_position.0 == tail_position.0 + 1 && head_position.1 == tail_position.1 {
        return false;
    }
    if head_position.0 == tail_position.0 - 1 && head_position.1 == tail_position.1 {
        return false;
    }

    //y varies
    if head_position.0 == tail_position.0 && head_position.1 == tail_position.1 + 1 {
        return false;
    }
    if head_position.0 == tail_position.0 && head_position.1 == tail_position.1 - 1 {
        return false;
    }

    //diagnoal
    if head_position.0 == tail_position.0 - 1 && head_position.1 == tail_position.1 - 1 {
        return false;
    }
    if head_position.0 == tail_position.0 - 1 && head_position.1 == tail_position.1 + 1 {
        return false;
    }
    if head_position.0 == tail_position.0 + 1 && head_position.1 == tail_position.1 - 1 {
        return false;
    }
    if head_position.0 == tail_position.0 + 1 && head_position.1 == tail_position.1 + 1 {
        return false;
    }

    return true;
}