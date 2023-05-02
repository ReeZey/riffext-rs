use std::{fs::{File, self}, io::{Write, Seek, SeekFrom, Read}, os::windows::prelude::FileExt, path::Path};

fn main() -> std::io::Result<()> {
    if !Path::new("output").exists() {
        fs::create_dir("output").expect("couldn't create output directory");
    }

    //set input file here :p
    let filename = "input_file.bin";
    let mut file = File::open(&filename)?;
    
    let mut cursor = 0;
    let mut buffer = [0; 4];
    while file.seek_read(&mut buffer, cursor)? == 4 {
        if &buffer == b"RIFF" {
            file.seek(SeekFrom::Start(cursor)).unwrap();
            let filename = format!("output/{filename}-{cursor}.wav");

            println!("found new riff at offset {}", cursor);

            let mut new_file = File::create(&filename).expect("could not save new file");
            file.read_exact(&mut buffer)?; //RIFF
            file.read_exact(&mut buffer)?; //ChunkData Length

            let total_size = i32::from_le_bytes(buffer);

            new_file.write(b"RIFF").expect("could not write to file");
            new_file.write(&total_size.to_le_bytes()).expect("could not write to file");

            
            let mut buffer = vec![0; total_size as usize];
            file.read_exact(&mut buffer).unwrap();
            
            new_file.write_all(&buffer).unwrap();
            cursor += total_size as u64;
        }
        cursor += 1;
    };

    Ok(())
}