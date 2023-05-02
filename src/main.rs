use std::{fs::{File, self}, io::{Write, Seek, SeekFrom, Read}, os::windows::prelude::FileExt, path::Path, env};

fn main() -> std::io::Result<()> {
    let foldername = "riff_output";
    if !Path::new(&foldername).exists() {
        fs::create_dir(&foldername).expect("couldn't create output directory");
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("bad arguments");
        return Ok(());
    }

    let filename = Path::new(&args[1]);
    let mut file = File::open(filename)?;
    
    let mut cursor = 0;
    let mut buffer = [0; 4];
    while file.seek_read(&mut buffer, cursor)? == 4 {
        if &buffer == b"RIFF" {
            file.seek(SeekFrom::Start(cursor)).unwrap();
            let parent = Path::new(&filename).parent().expect("could not find parent folder");
            let filename = format!("{}/{}-{cursor}.wav", parent.join(foldername).to_string_lossy(), filename.file_name().unwrap().to_string_lossy());

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