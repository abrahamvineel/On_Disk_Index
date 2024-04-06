
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let name = String::from("apple");
    let subject = String::from("computer_science");
    const RANGE: u32 = 1_000_000;

    let mut file = File::create("data.txt").expect("Failed to create file");


    for i in 0..=RANGE {
        let data = name.clone() + &i.to_string() + "," + &subject.clone() + &i.to_string() + "\n";
        // println!("{}", data);
        file.write_all(data.as_bytes()).expect("Failed to write to file");
        // println!("{} {} {}", i, name.clone() + &i.to_string(), subject.clone() + &i.to_string());
    }

}
