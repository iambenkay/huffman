use crate::bitmanip;
use std::collections::HashMap;
use std::fmt::{Display};
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct HuffmanNode {
    freq: usize,
    byte: Option<u8>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HuffTreeErr,
    Message(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HuffTreeErr => write!(f, "Failed to create Huffman tree"),
            Self::Message(msg) => msg.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Message(value.to_string())
    }
}

impl From<bitmanip::Error> for Error {
    fn from(value: bitmanip::Error) -> Self {
        match value {
            bitmanip::Error::NotValid => Error::Message("Invalid bit string".into())
        }
    }
}

pub fn compress_data(mut reader: impl Read + Seek, mut writer: impl Write) -> Result<()> {
    let tree_root = create_huffman_tree(&mut reader)?;

    let mut huffman_code_table: HashMap<u8, String> = HashMap::new();

    assign_huffman_codes(tree_root, &mut huffman_code_table, "".to_string());

    let mut buffer: [u8; 65536] = [0; 65536];

    loop {
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        let mut compressed_segment = String::new();

        for byte in buffer {
            compressed_segment.push_str(&huffman_code_table[&byte]);
        }

        let compressed_bytes = bitmanip::bit_str_to_bytes(&compressed_segment)?;

        writer.write_all(&compressed_bytes)?;
    }

    Ok(())
}

fn create_huffman_tree(mut reader: impl Read + Seek) -> Result<Box<HuffmanNode>> {
    let freq_map = count_unique_bytes(&mut reader)?;

    let mut nodes: Vec<Box<HuffmanNode>> = freq_map
        .iter()
        .map(|(ch, freq)| new_node(Some(*ch), *freq))
        .collect();

    while nodes.len() > 1 {
        nodes.sort_by(|a, b| b.freq.cmp(&a.freq));
        let a = nodes.pop().ok_or(Error::HuffTreeErr)?;
        let b = nodes.pop().ok_or(Error::HuffTreeErr)?;

        let mut node = new_node(None, a.freq + b.freq);
        node.left = Some(a);
        node.right = Some(b);
        nodes.push(node);
    }

    let root = nodes.pop().ok_or(Error::HuffTreeErr)?;

    Ok(root)
}

fn assign_huffman_codes(node: Box<HuffmanNode>, h: &mut HashMap<u8, String>, s: String) {
    if let Some(ch) = node.byte {
        h.insert(ch, s);
    } else {
        if let Some(left) = node.left {
            assign_huffman_codes(left, h, s.clone() + "0");
        }
        if let Some(right) = node.right {
            assign_huffman_codes(right, h, s + "1");
        }
    }
}

fn count_unique_bytes(mut reader: impl Read + Seek) -> Result<HashMap<u8, usize>> {
    let mut freq_map: HashMap<u8, usize> = HashMap::new();

    let mut buffer: [u8; 65536] = [0; 65536];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        for byt in buffer.iter().take(bytes_read) {
            let count = freq_map.entry(*byt).or_insert(0);
            *count += 1;
        }
    }

    reader.seek(SeekFrom::Start(0))?;

    Ok(freq_map)
}

fn new_node(byte: Option<u8>, freq: usize) -> Box<HuffmanNode> {
    Box::new(HuffmanNode {
        byte,
        freq,
        left: None,
        right: None,
    })
}
