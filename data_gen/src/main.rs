
use std::fs::File;
use std::io::Write;

fn main() {
    const RANGE: u32 = 1_000_000;
    // const RANGE: u32 = 1_0;
    let name = String::from("apple");
    let subject = String::from("computer_science");
    let mut file = File::create("data.txt").expect("Failed to create file");

    for i in 0..=RANGE {
        let data = format!("{}{},{}{}\n", name, i, subject, i);
        file.write_all(data.as_bytes()).expect("Failed to write to file");
    }
}
