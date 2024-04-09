use clap::{App, Arg};

pub fn run() {
    let matches = App::new("Rust File Sync Tool")
        .version("0.1.0")
        .author("Enow Scott")
        .about("Synchronize files between directories")
        .arg(
            Arg::with_name("source")
                .help("Source directory")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("destination")
                .help("Destination directory")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .help("Delete files not present in source directory"),
        )
        .get_matches();

    let source_dir = matches.value_of("source").unwrap();
    let destination_dir = matches.value_of("destination").unwrap();
    let delete_files = matches.is_present("delete");

    // Call synchronization logic
    sync::sync_directories(source_dir, destination_dir, delete_files);
}
