use std::fs::{File};
use std::io::{BufReader, BufWriter};
use std::io;
use huffman::huffman;

fn main() {
    let mut reader = file_reader("data/S.csv").unwrap();
    let mut writer = file_writer("data/S.csv.huff").unwrap();

    huffman::compress_data(&mut reader, &mut writer).unwrap();
}

fn file_reader(filename: &str) -> io::Result<BufReader<File>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn file_writer(filename: &str) -> io::Result<BufWriter<File>> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    Ok(writer)
}
