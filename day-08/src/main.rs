fn main() {

    let mut grid: Vec<Vec<usize>> = Vec::new();
    let file_lines = include_str!("../data/dataset.txt").split("\n");

    let mut row_len: usize = 0;
    let mut column_len: usize = 0;
    let mut visible_trees: usize = 0;

    for line in file_lines {
        let row: Vec<usize> = line.chars().map(|each| each.to_string().parse::<usize>().unwrap()).collect();
        row_len = row.len();
        grid.push(row);
    }

    column_len = grid.len();
    for i in 0..column_len {
        for j in 0..row_len {
            if i == 0 || i == column_len - 1 || j == 0 || j == row_len - 1 {
                visible_trees += 1;
            } else {
                let tree_height = grid[i][j];
                let mut visible: bool = true;

                println!("tree_height: [{}] i = [{}] & j = [{}] | {}", tree_height, i, j, visible);
                
                for top in (0..i).rev() {
                    println!("top: {} | {} >= {} | {}", top, grid[top][j], tree_height, visible);
                    if grid[top][j] >= tree_height {
                        visible = false;
                        break;
                    }
                }
                if visible {
                    visible_trees += 1;
                    println!("Visible from top");
                    println!("");
                    continue;
                }

                visible = true;
                for bottom in i+1..column_len {
                    println!("bottom: {} | {} >= {} | {}", bottom, grid[bottom][j], tree_height, visible);
                    if grid[bottom][j] >= tree_height {
                        visible = false;
                        break;
                    }
                }
                if visible {
                    visible_trees += 1;
                    println!("Visible from bottom");
                    println!("");
                    continue;
                }

                visible = true;
                for left in (0..j).rev(){
                    println!("left: {} | {} >= {} | {}", left, grid[i][left], tree_height, visible);
                    if grid[i][left] >= tree_height {
                        visible = false;
                        break;
                    }
                }
                if visible {
                    visible_trees += 1;
                    println!("Visible from left");
                    println!("");
                    continue;
                }

                visible = true;
                for right in j+1..row_len {
                    println!("right: {} | {} >= {} | {}", right, grid[i][right], tree_height, visible);
                    if grid[i][right] >= tree_height {
                        visible = false;
                        break;
                    }
                }
                if visible {
                    visible_trees += 1;
                    println!("Visible from right");
                    println!("");
                    continue;
                }
                println!("Tree is not visible");
                println!("=============================");
                println!("");
            }
        }
    } 

    println!("Visible Trees: {}", visible_trees);
}

