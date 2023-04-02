extern crate flate2;

use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};
use std::time::Instant;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;


fn compress(input_path: &str, output_path: &str) {
    let mut input = BufReader::new(File::open(input_path).unwrap());
    let output = File::create(output_path).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}

fn decompress(input_path: &str, output_path: &str) {
    let input = File::open(input_path).unwrap();
    let mut decoder = GzDecoder::new(input);
    let output = File::create(output_path).unwrap();
    let mut output = BufWriter::new(output);
    let start = Instant::now();
    copy(&mut decoder, &mut output).unwrap();
    println!("Elapsed: {:?}", start.elapsed());
}

fn main() {
    if args().len() != 4 {
        eprintln!("Usage: '-comp/-decomp' 'source' 'target'");
        return;
    }
    let operation = args().nth(1).unwrap();
    let input_path = args().nth(2).unwrap();
    let output_path = args().nth(3).unwrap();

    match operation.as_str() {
        "-comp" => compress(&input_path, &output_path),
        "-decomp" => decompress(&input_path, &output_path),
        _ => {
            eprintln!("Invalid operation. Use '-comp' or '-decomp'.");
        }
    }
}
