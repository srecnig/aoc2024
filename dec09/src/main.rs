use std::fs;

use dec09::Disk;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    let normalized_input = input.replace('\n', "");

    let mut disk: Disk = Disk::new(&normalized_input);
    disk.compact();
    println!("pt1: {}", disk.checksum());

    let mut disk: Disk = Disk::new(&normalized_input);
    disk.compact_files();
    println!("pt1: {}", disk.checksum());
}
