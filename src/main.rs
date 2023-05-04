use std::{fs::{File, self}, io::{Write, Seek, SeekFrom, Read}, path::Path, env};

fn main() -> std::io::Result<()> {
    let foldername = "riff_output";
    if !Path::new(&foldername).exists() {
        fs::create_dir(&foldername).expect("couldn't create output directory");
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("bad arguments");
        return Ok(());
    }

    let filename = Path::new(&args[1]);
    let mut file = File::open(filename)?;

    let mut find_r = [0; 1];
    let mut find_iff = [0; 3];

    let mut buffer = [0; 4];

    let mut spinner = 0;

    while file.read_exact(&mut find_r).is_ok() {
        if &find_r != b"R" { continue; }
        print!("\rlooking for riff {} ", spinny_boi(&spinner));
        let current = SeekFrom::Current(0);
        file.read_exact(&mut find_iff)?;
        if &find_iff != b"IFF" {
            file.seek(current)?;
            spinner += 1;
            if spinner > 3 { spinner = 0 }

            continue;
        }

        print!("\r");

        let current_offset = file.seek(SeekFrom::Current(0));
        let cursor = current_offset.unwrap() - 4;

        let parent = Path::new(&filename).parent().expect("could not find parent folder");
        println!("found new riff at offset {:3}", cursor);

        file.read_exact(&mut buffer)?; //ChunkData Length
        let total_size = i32::from_le_bytes(buffer) - 4;
        file.read_exact(&mut buffer)?; //Format
        let file_type = get_type(&buffer);

        let new_filename = format!("{}/{}-{cursor}.{file_type}", parent.join(foldername).to_string_lossy(), filename.file_name().unwrap().to_string_lossy());
        let mut new_file = File::create(&new_filename).expect("could not save new file");

        new_file.write(b"RIFF").expect("could not write to file");
        new_file.write(&total_size.to_le_bytes()).expect("could not write to file");
        new_file.write_all(&buffer).expect("could not write to file");
        
        let mut buffer = vec![0; total_size as usize];
        file.read_exact(&mut buffer).unwrap();

        new_file.write_all(&buffer).unwrap();
    }

    Ok(())
}

fn get_type(array: &[u8]) -> &str {
    return match array {
        b"WAVE" => "wav",
        b"WEBP" => "webp",
        b"AVI " => "avi",
        _ => &std::str::from_utf8(array).unwrap(),
    }
}

fn spinny_boi(num: &u8) -> &str {
    return match num {
        0 => "/",
        1 => "|",
        2 => "\\",
        3 => "-",
        _ => "how?"
    }
}