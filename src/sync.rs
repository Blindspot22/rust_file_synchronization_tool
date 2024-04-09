use std::fs;
use std::io;

pub fn sync_directories(source_dir: &str, destination_dir: &str, delete_files: bool) {
    // Implement synchronization logic here
    println!("Syncing files from '{}' to '{}'", source_dir, destination_dir);

    // Example: Copy all files from source to destination
    match fs::read_dir(source_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let source_path = entry.path();
                    let dest_path = format!("{}/{}", destination_dir, file_name.to_string_lossy());

                    if let Err(err) = fs::copy(&source_path, &dest_path) {
                        println!("Error copying file: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading source directory: {}", err);
        }
    }

    // Optionally, delete files not present in source directory
    if delete_files {
        delete_extra_files(source_dir, destination_dir);
    }
}

fn delete_extra_files(source_dir: &str, destination_dir: &str) {
    // Implement deletion of extra files logic here
    println!("Deleting extra files from '{}'", destination_dir);

    // Example: List all files in destination directory and delete those not present in source directory
    match fs::read_dir(destination_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let source_path = format!("{}/{}", source_dir, file_name.to_string_lossy());
                    let dest_path = entry.path();

                    if !fs::metadata(&source_path).is_ok() {
                        if let Err(err) = fs::remove_file(&dest_path) {
                            eprintln!("Error deleting file: {}", err);
                        }
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading destination directory: {}", err);
        }
    }
}
