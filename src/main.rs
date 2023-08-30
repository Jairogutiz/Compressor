use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;


fn main() {
    // Check if the number of command line arguments is not equal to 3
    if args().len() != 3 {
        // Print usage instructions
        eprintln!("Usage: 'source' 'target'");
        return;
    }

    // Open the source file for reading
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    // Create the target file for writing
    let output = File::create(args().nth(2).unwrap()).unwrap();
    // Create a GzEncoder to compress the data and write it to the target file
    let mut encoder = GzEncoder::new(output, Compression::Default);
    // Start measuring the elapsed time
    let start = Instant::now();
    // Copy the data from the source file to the encoder
    copy(&mut input, &mut encoder);
    // Finish the compression and get the compressed data
    let output = encoder.finish().unwrap();
    
    // Print the length of the source file
    println!("Source len: {:?}", input.get_ref().metadata().unwrap().len());
    // Print the length of the target file
    println!("Target len: {:?}", output.metadata().unwrap().len());
    // Print the elapsed time
    println!("Elapsed: {:?}", start.elapsed());
    
}
