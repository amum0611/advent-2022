fn main() {

    let sum = solve(1, 1);
    println!("Part 1: Sum = {:?}", sum);

    let sum = solve(811589153, 10);
    println!("Part 2: Sum = {:?}", sum);
}

fn solve(decryption_key: i64, mix: usize) -> i64 {
    let mut original: Vec<i64> = include_str!("../data/dataset.txt").split("\n").map(|each| each.parse::<i64>().unwrap()).collect();
    let mut target: Vec<i64> = (0..original.len() as i64).collect();

    for _ in 0..mix {
        for index in 0..(original.len() as i64) {
            let jump_value = target.iter().position(|&x| x == index).unwrap();
            target.remove(jump_value);
            target.insert((((jump_value as i64) + original[index as usize] * decryption_key).rem_euclid(target.len() as i64)) as usize, index);
        }    
    }

    // println!("{:?}", original);
    // println!("{:?}", target);

    let zero_p = original.iter().position(|&x| x == 0).unwrap();
    let element_in_zero_p = target.iter().position(|&x| x == zero_p as i64).unwrap();
    let sum: i64 = [1000, 2000, 3000].iter()
        .map(|index| original[target[(element_in_zero_p + index) as usize % target.len()] as usize] * decryption_key).sum();
    sum
}