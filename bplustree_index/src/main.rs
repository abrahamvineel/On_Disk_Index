use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use serde::{Deserialize, Serialize};

const PAGE_SIZE: usize = 4096;
const FILE_NAME: &str = "bplus_index.dat";
const MAX_NODES: i32 = 4;

#[derive(Serialize, Deserialize)]
struct Node {
    keys: Vec<i32>,
    children: Vec<usize>
}

#[derive(Debug)]
enum SerializationError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for SerializationError {
    fn from(error: std::io::Error) -> Self {
        SerializationError::IoError(error)
    }
}

impl Node {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializationError> {
            // Write key as u32
        for key in &self.keys {
            buffer.write_all(&key.to_be_bytes())?;
        }

            // Write value length (string length) as u16
        let value_len = self.children.len() as u16;
        buffer.write_all(&value_len.to_be_bytes())?;

            // Write value as bytes
        for child in &self.children {
            buffer.write_all(&child.to_be_bytes())?;
        }
        Ok(())
    }

    fn deserialize(buffer: &[u8]) -> Option<Node> {
        if buffer.len() < 8 { // Minimum size for key & length
            return None;
        }

        // Read key as u32
        let mut key_bytes = [0; 4];
        key_bytes.copy_from_slice(&buffer[..4]);
        // let keys = u32::from_be_bytes(key_bytes);

        let keys = vec![1, 2, 3, 4, 5];
        // Read value length as u16
        let mut value_len_bytes = [0; 2];
        value_len_bytes.copy_from_slice(&buffer[4..6]);
        let value_len = u16::from_be_bytes(value_len_bytes) as usize;

        // Check if buffer has enough data for value
        if buffer.len() < 8 + value_len {
            return None;
        }

        // Read value as string
        // let children = u32::from_be_bytes([]);

        let children = vec![10, 20, 30];
        Some(Node { keys, children })
    }
}

struct LeafNode {
    keys: Vec<i32>,
    values: Vec<usize>
}

struct BPlusTree {
    root: Option<Node>,
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
        let root = if buffer.len() != 0 {
            Some(Node::deserialize(&buffer).expect("TODO: handle deserialization error"))
        } else {
            // Create an empty root node (leaf)
            let root_node = Node {
                keys: vec![],
                children: vec![]
            };
            let offset = buffer.len();
            root_node.serialize(&mut buffer)?;
            self.file.seek(SeekFrom::Start(0))?;
            self.file.write_all(&buffer)?;
            self.root = Some(root_node);
            // Write the serialized root node
            None
        };

        // let root = if let Some(root_offset) = self.root {
        //     // Node::deserialize(&buffer[root_offset..])
        // } else {
        //     let root_node = Node { keys: vec![], children: vec![] };
        //     let offset = buffer.len();
        //     root_node.serialize(&mut buffer)?;
        //     self.root = Some(offset);
        //     Node::deserialize(&buffer[offset..]);
        //     offset;
        // };
        Ok(BPlusTree { root, file })
    }

    fn insert(&mut self) {
        // check for node size if > max_nodes then split else insert
        //need to check whether to write to disk after each insert or every 1000 inserts using WAL?
        //b+tree can be left bias or right bias need to pick one

    }
}

fn main() {
    let mut bplustree = BPlusTree {root: None, file: File::create(FILE_NAME)?};
    bplustree.new();
}
