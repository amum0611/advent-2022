fn main() {

    let mut grid: Vec<Vec<usize>> = Vec::new();
    let mut highest_scenic_scores: usize = usize::MIN;

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
                
                let mut visible_from_top: bool = true;
                let mut visible_from_bottom: bool = true;
                let mut visible_from_left: bool = true;
                let mut visible_from_right: bool = true;

                let mut visible_trees_from_top: usize = 0; 
                let mut visible_trees_from_bottom: usize = 0; 
                let mut visible_trees_from_left: usize = 0; 
                let mut visible_trees_from_right: usize = 0; 
                
                for top in (0..i).rev() {
                    let tree_under_consideration: usize = grid[top][j];
                    if visible_from_top && tree_under_consideration >= tree_height {
                        visible_from_top = false;
                        visible_trees_from_top += 1;
                    } else if visible_from_top {
                        visible_trees_from_top += 1;
                    }
                }

                for bottom in i+1..column_len {
                    let tree_under_consideration: usize = grid[bottom][j];
                    if visible_from_bottom && tree_under_consideration >= tree_height {
                        visible_from_bottom = false;
                        visible_trees_from_bottom += 1;
                    } else if visible_from_bottom {
                        visible_trees_from_bottom += 1;
                    }
                }

                for left in (0..j).rev(){
                    let tree_under_consideration: usize = grid[i][left];
                    if visible_from_left && tree_under_consideration >= tree_height {
                        visible_from_left = false;
                        visible_trees_from_left += 1;
                    } else if visible_from_left {
                        visible_trees_from_left += 1;
                    }
                }

                for right in j+1..row_len {
                    let tree_under_consideration: usize = grid[i][right];
                    if visible_from_right && tree_under_consideration >= tree_height {
                        visible_from_right = false;
                        visible_trees_from_right += 1;
                    } else if visible_from_right {
                        visible_trees_from_right += 1;
                    }
                }

                let scenic_score_of_a_tree: usize = visible_trees_from_top * visible_trees_from_bottom * visible_trees_from_left * visible_trees_from_right;
              
                if scenic_score_of_a_tree > highest_scenic_scores {
                    highest_scenic_scores = scenic_score_of_a_tree;
                }

                if visible_from_top || visible_from_bottom || visible_from_left || visible_from_right {
                    visible_trees += 1;
                }
            }
        }
    } 

    println!("Visible Trees (Part 1): {}", visible_trees);
    println!("Highest Scenic Score (Part 2): {}", highest_scenic_scores);
}

