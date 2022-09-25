use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

/// Search for a pattern in a file and display th elines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

// fn main() -> std::io::Result<> {
//     let args = Cli::parse();
//     println!("pattern: {:?}", args.pattern);
//     println!("path: {:?}", args.path);

//     let file = File::open(&args.path)?;
//     let reader = BufReader::new(file);
//     for line in reader.lines() {
//         if line?.contains(&args.pattern) {
//             println!("{}", line?);
//         }
//     }

//     Ok(());
// }

fn main() -> io::Result<()> {
    let args = Cli::parse();
    println!("pattern: {:?}", args.pattern);
    println!("path: {:?}", args.path);

    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let validated_line = line?;
        if validated_line.contains(&args.pattern) {
            println!("{:?}", validated_line);
        }
    }

    Ok(())
}
