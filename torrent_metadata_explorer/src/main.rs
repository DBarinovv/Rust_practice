use std::io;
use std::path::{Path, PathBuf};

fn main() {
    loop {
        let path = prompt_for_path().unwrap_or_else(|e| {
            println!("Error reading input: {}", e);
            PathBuf::new()  // return an empty path on error
        });

        match check_metadata(&path) {
            Ok(()) => {
                println!("Metadata is okay!");
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn prompt_for_path() -> Result<PathBuf, io::Error> {
    let mut input_path = String::new();
    println!("Please enter the file path:");
    
    io::stdin().read_line(&mut input_path)?;
    Ok(PathBuf::from(input_path.trim()))
}

fn check_metadata(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err("The file does not exist.".to_string());
    }

    let path = path.canonicalize().expect("Can't make global var");
    
    let metadata = path.metadata().map_err(|e| e.to_string())?;
    
    println!("{:?}", metadata.file_type());
    println!("{:?}", metadata.permissions());
    println!("{:?}", metadata.created());
    Ok(())
}
