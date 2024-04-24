
use std::fs::{File, OpenOptions};
use std::path::Path;
const PAGE_SIZE: usize = 4096;
const FILE_NAME: &str = "bplus_index.dat";
const MAX_NODES: i32 = 4;
struct Node {
    keys: Vec<i32>,
    children: Vec<usize>
}

struct LeafNode {
    keys: Vec<i32>,
    values: Vec<usize>
}

struct BPlusTree {
    root: Option<usize>,
    file: File
}

impl BPlusTree {

    fn new() -> std::io::Result<Self> {
        let file = File::open(FILE_NAME).unwrap_or_else(|err| {
                eprintln!("File not found: {}", FILE_NAME);
                std::process::exit(1);
            });
        Ok(BPlusTree { root: None, file })
    }

    fn insert() {
        // check for node size if > max_nodes then split else insert
        //need to check whether to write to disk after each insert or every 1000 inserts using WAL?
    }
}

fn main() {


}
