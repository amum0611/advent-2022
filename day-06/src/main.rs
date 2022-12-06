fn main() {

    start_of_packet_marker(4);
    start_of_packet_marker(14);
}

fn start_of_packet_marker(packet_size: usize) {
    let file_lines = include_str!("../data/dataset.txt").split("\n");
    let data_stream: Vec<char> = file_lines.last().unwrap().chars().collect();

    for i in 0..data_stream.len() - packet_size - 1 {
        let mut packet: Vec<char> = data_stream[i..i+packet_size].to_vec();
        packet.sort();
        packet.dedup();
        if packet.len() == packet_size {
            println!("First start-of-packet marker is detected {} @ {}", packet.into_iter().collect::<String>(), i + packet_size);
            break;
        }
    }
}
