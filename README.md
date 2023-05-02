# riff extractor

useful for extracting WAVE files from a single file  
made in Rust!

## How do i use this?
Simply download the executable from releases and drag & drop files that you want unpacked ontop of the executable

*Note: Currently there is no way to choose where it puts the files  
so it creates a folder called `riff_output` in the working directory

## Currently supporting 
| Format  | Extension |
|:-------:|:---------:|
| WEBP    | .webp     |
| WAVE    | .wav      |
| AVI     | .avi      |

If it encounters a unknown format, the file extension is the RIFF Format

Feel free to add more with a pull request here:
```
[main.rs]
fn get_type(array: &[u8]) -> &str {
    return match array {
        b"WAVE" => "wav",
        b"WEBP" => "webp",
        b"AVI " => "avi",
        b"NEW " => "new", <--- like this
        _ => &std::str::from_utf8(array).unwrap(),
    }
}
```
