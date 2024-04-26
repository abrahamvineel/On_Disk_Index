use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

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

    fn new(&mut self) -> std::io::Result<Self> {
        let file = File::open(FILE_NAME).unwrap_or_else(|err| {
                eprintln!("File not found: {}", FILE_NAME);
                std::process::exit(1);
            });
        let mut buffer = vec![0; PAGE_SIZE];
        self.file.seek(SeekFrom::Start(0))?;
        self.file.read_exact(&mut buffer)?;
        let root = if let Some(root_offset) = self.root {
            // Node::deserialize(&buffer[root_offset..])
        } else {
            let root_node = Node { keys: vec![], children: vec![] };
            let offset = buffer.len();
            root_node.serialize(&mut buffer)?;
            self.root = Some(offset);
            Node::deserialize(&buffer[offset..])
            offset
        };
        Ok(BPlusTree { root, file })
    }

    fn insert(&mut self) {
        // check for node size if > max_nodes then split else insert
        //need to check whether to write to disk after each insert or every 1000 inserts using WAL?
        //b+tree can be left bias or right bias need to pick one

    }
}

fn main() {
    let mut bptree = BPlusTree::new();


}
