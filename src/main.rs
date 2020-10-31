use structopt::StructOpt;
use structopt::clap::Arg;
use std::fs::File;
use std::io::{BufReader, BufRead,  Lines};
use anyhow::{Context, Result, Error};
use std::io;
use std::path::Path;
use std::fmt::Display;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug) ]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args  : Cli  = Cli::from_args();


    match  read_lines(&args.path) {
        Ok(lines) =>
            {
                for line in lines {
                    println!("{}",line.unwrap());
                }
                Ok(())
            }
        Err(e) => { Err(Error::from(e))}
    }
}
        // Consumes the iterator, returns an (Optional) String


fn read_lines(filename: &std::path::PathBuf) -> Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename).with_context(|| format!("could not read file `{}`", &filename.to_str().unwrap()))?;
    Ok(io::BufReader::new(file).lines())
}