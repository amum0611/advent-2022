use regex::Regex;

fn main() {
    part_01();
    part_02();
}
 
fn part_01() {
    let mut fully_contains_other: u16 = 0;
    let re = Regex::new("-|,").unwrap();

    let file_lines = include_str!("../data/dataset.txt").split("\n");

    for line in file_lines {
        let fields: Vec<u32> = re.split(line).map(|each| each.parse::<u32>().unwrap()).collect();
        if fields[0] <= fields[2] && fields[1] >= fields[3] {
            fully_contains_other += 1;
        } else if fields[0] >= fields[2] && fields[1] <= fields[3] {
            fully_contains_other += 1;
        }
    }

    println!("No. of assignment pairs with fully duplicate (part 1): {}", fully_contains_other); 
}

fn part_02() {
    let mut doesnot_contains_other: u16 = 0;
    let re = Regex::new("-|,").unwrap();

    let file_lines = include_str!("../data/dataset.txt").split("\n");
    let mut number_of_lines: u16 = 0;

    for line in file_lines {
        number_of_lines += 1;
        let fields: Vec<u32> = re.split(line).map(|each| each.parse::<u32>().unwrap()).collect();
        if fields[2] > fields[1] || fields[3] < fields[0] {
            doesnot_contains_other += 1;
        } 
    }

    println!("Does not contain: {}", doesnot_contains_other);
    println!("No. of assignment pairs with partial duplicates (part 2): {}", number_of_lines - doesnot_contains_other); 
}