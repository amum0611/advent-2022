use regex::Regex;

fn main() {

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

    println!("No. of assignment pairs with fully diplicate (part 1): {}", fully_contains_other);
}
 