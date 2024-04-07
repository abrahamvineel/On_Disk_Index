use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    println!("Value is {}", get_value("apple10"))
}

fn get_value(queried_key:&str) -> String{
    let file = File::open("C:\\Pro\\On_Disk_Index\\data_gen\\data.txt").unwrap();

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let res = match line {
            Ok(line) => String::from(line),
            Err(err) => String::from(err.to_string())
        };
        let row: Vec<&str> = res.split(",").collect();
        let (key, value) = (row[0], row[1]);
        if key == queried_key {
            return value.to_string()
        }
    }
    return String::new();
}
