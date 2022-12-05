use regex::Regex;

fn main() {
    part01();
    part02();
}

fn part01() {
    let mut stack = vec![
        vec!['W', 'D', 'G', 'B', 'H', 'R', 'V'],
        vec!['J', 'N', 'G', 'C', 'R', 'F'],
        vec!['L', 'S', 'F', 'H', 'D', 'N', 'J'],
        vec!['J', 'D', 'S', 'V'],
        vec!['S', 'H', 'D', 'R', 'Q', 'W', 'N', 'V'],
        vec!['P', 'G', 'H', 'C', 'M'],
        vec!['F', 'J', 'B', 'G', 'L', 'Z', 'H', 'C'],
        vec!['S', 'J', 'R'],
        vec!['L', 'G', 'S', 'R', 'B', 'N', 'V', 'M']
    ];

    let re = Regex::new("move | from | to ").unwrap();
    let file_lines = include_str!("../data/dataset.txt").split("\n");

    for line in file_lines {
        let fields: Vec<usize> = re.split(line).filter(|x| !x.is_empty()).map(|x| x.parse::<usize>().unwrap()).collect();

        let from_stack = fields[1] - 1;
        let to_stack = fields[2] - 1;
        for _i in 0..fields[0] {
            let value = stack[from_stack].pop();
            stack[to_stack].push(value.unwrap());
        }
    }

    print!("Crate ends up on top of each stack (Part 1): ");
    for st in stack.iter_mut() {
        print!("{}", st.pop().unwrap());
    }
}

fn part02() {
    let mut stack = vec![
        vec!['W', 'D', 'G', 'B', 'H', 'R', 'V'],
        vec!['J', 'N', 'G', 'C', 'R', 'F'],
        vec!['L', 'S', 'F', 'H', 'D', 'N', 'J'],
        vec!['J', 'D', 'S', 'V'],
        vec!['S', 'H', 'D', 'R', 'Q', 'W', 'N', 'V'],
        vec!['P', 'G', 'H', 'C', 'M'],
        vec!['F', 'J', 'B', 'G', 'L', 'Z', 'H', 'C'],
        vec!['S', 'J', 'R'],
        vec!['L', 'G', 'S', 'R', 'B', 'N', 'V', 'M']
    ];

    let re = Regex::new("move | from | to ").unwrap();
    let file_lines = include_str!("../data/dataset.txt").split("\n");

    for line in file_lines {
        let fields: Vec<usize> = re.split(line).filter(|x| !x.is_empty()).map(|x| x.parse::<usize>().unwrap()).collect();

        let from_stack = fields[1] - 1;
        let to_stack = fields[2] - 1;

        let mut temp: Vec<char> = Vec::new();

        for _i in 0..fields[0] {
            let value = stack[from_stack].pop();
            
            temp.push(value.unwrap());
        }

        loop {
            let value = temp.pop();
            if value.is_none() {
                break;
            }
            stack[to_stack].push(value.unwrap());
        }
    }

    print!("\nCrate ends up on top of each stack (Part 2): ");
    for st in stack.iter_mut() {
        print!("{}", st.pop().unwrap());
    }
    println!("");
}