fn main() {

    let mut total_priority: usize = 0;
    let mut total_badge_priority: usize = 0;
    // = Vec::new();

    let alphabet = String::from_utf8((b'a' ..= b'z').chain(b'A' ..= b'Z').collect()).unwrap();
    // let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let file_lines = include_str!("../data/dataset.txt").split("\n");
    for line in file_lines {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let common_item_priority = find_common_item_and_priority(&alphabet, first_half, second_half);
        total_priority += common_item_priority;
    }

    let mut group_vec: Vec<&str> = include_str!("../data/dataset.txt").split("\n").collect();

    for group_rucksack in group_vec.chunks(3).collect::<Vec<&[&str]>>() {
        println!("HEHE {:?}", group_rucksack[0]);
        total_badge_priority += find_badge_priority(&alphabet, group_rucksack[0], group_rucksack[1], group_rucksack[2]);
    }

    println!("Total Priority (Part 1): {}", total_priority);
    println!("Total Badge Priority (Part 2): {}", total_badge_priority);
}

fn find_alphabet_index(alphabet: &String, item: char) -> usize {
    let index = alphabet
        .chars()
        .position(|x| x == item)
        .unwrap();

    index
}

fn find_common_item_and_priority(alphabet: &String, first_half: &str, second_half: &str) -> usize {

    let mut first_half_occurrance_array: [u8; 52] = [0; 52];

    //length of both first_half and second_half are equal
    let half_string_len = first_half.len();

    //println!("{} and {} | {}", first_half, second_half, half_string_len);

    for index in 0..half_string_len {
        let char_vec: Vec<char> = first_half.chars().collect();
        let priority = find_alphabet_index(&alphabet, char_vec[index]);
        let occurrance = first_half_occurrance_array[priority];
        first_half_occurrance_array[priority] = occurrance + 1;
    }

    let mut second_half_occurrance_array: [u8; 52] = [0; 52];

    for index in 0..half_string_len {
        let char_vec: Vec<char> = second_half.chars().collect();
        let priority = find_alphabet_index(&alphabet, char_vec[index]);
        let occurrance = second_half_occurrance_array[priority];
        second_half_occurrance_array[priority] = occurrance + 1;
    }

    for index in 0..52 {
        if first_half_occurrance_array[index] > 0 && second_half_occurrance_array[index] > 0 {
            //let alphabet1: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
            // println!("{} | {} | {}", index + 1, first_half_occurrance_array[index], alphabet1[index]);
            return index + 1;
        }
    }

    return 0;
}

fn find_badge_priority(alphabet: &String, first_sack: &str, second_sack: &str, third_sack: &str) -> usize {
    
    let mut first_occurrance_array: [u8; 52] = [0; 52];

    for index in 0..first_sack.len() {
        let char_vec: Vec<char> = first_sack.chars().collect();
        let priority = find_alphabet_index(&alphabet, char_vec[index]);
        let occurrance = first_occurrance_array[priority];
        first_occurrance_array[priority] = occurrance + 1;
    }

    let mut second_occurrance_array: [u8; 52] = [0; 52];

    for index in 0..second_sack.len() {
        let char_vec: Vec<char> = second_sack.chars().collect();
        let priority = find_alphabet_index(&alphabet, char_vec[index]);
        let occurrance = second_occurrance_array[priority];
        second_occurrance_array[priority] = occurrance + 1;
    }

    let mut third_occurrance_array: [u8; 52] = [0; 52];

    for index in 0..third_sack.len() {
        let char_vec: Vec<char> = third_sack.chars().collect();
        let priority = find_alphabet_index(&alphabet, char_vec[index]);
        let occurrance = third_occurrance_array[priority];
        third_occurrance_array[priority] = occurrance + 1;
    }

    for index in 0..52 {
        if first_occurrance_array[index] > 0 && second_occurrance_array[index] > 0 && third_occurrance_array[index] > 0 {
            return index + 1;
        }
    }

    return 0;
}