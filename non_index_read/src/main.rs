use std::fs::File;
use std::io;
use std::io::{BufRead, Read};
use std::thread;
use std::time::{Instant, Duration};

fn main() {
    // let start_time = Instant::now();

    // println!("Value is {}", get_value("apple100000000"));
    // println!("Elapsed time: {:?}", Instant::now() - start_time);

    let mut handles = vec![];
    for i in 1..=10 {
        let handle = thread::spawn(move || {
            let v = 100000000 - i;
            let val = format!("{}{}", "apple", v);
            let start_time = Instant::now();
            println!("Value is {}", get_value(&val));
            println!("Elapsed time: {:?}", Instant::now() - start_time);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn get_value(queried_key:&str) -> String{
    let file = File::open("../data_gen/data.txt").unwrap();
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

fn get_value_with_100mil(queried_key:&str) -> String{
    let mut file = File::open("../data_gen/data100mil.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    // let reader = io::BufReader::new(file);

    for line in buffer.split("\n") {
        // println!("{}", line);
        // let res = match line {
        //     Ok(line) => String::from(line),
        //     Err(err) => String::from(err.to_string())
        // };
        let row: Vec<&str> = line.split(",").collect();
        let (key, value) = (row[0], row[1]);
        if key == queried_key {
            return value.to_string()
        }
    }
    return String::new();
}
