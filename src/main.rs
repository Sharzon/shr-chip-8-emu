use std::env;
use std::io;
// Why must we import trait?
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        println!("Please, give a file name");
        return;
    }
    read_file(&args[0]);
}

fn read_file (filename: &String) -> io::Result<()> {
    let mut file = std::fs::File::open(filename)?;
    let mut list_of_chunks = Vec::new();
    let chunk_size = 2;

    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        // What is adaptor?
        let n = file.by_ref().take(chunk_size as u64).read_to_end(&mut chunk)?;
        if n == 0 { break; }
        list_of_chunks.push(chunk);
        if n < chunk_size { break; }
    }

    for chunk in list_of_chunks.into_iter() {
        println!("{:#06x}", chunk_to_u32(chunk));
    }

    Ok(())
}

fn chunk_to_u32 (chunk: Vec<u8>) -> u32 {
    let mut res: u32 = 0;
    for byte in chunk.into_iter() {
        res = (res << 8) + (byte as u32);
    }

    res
}