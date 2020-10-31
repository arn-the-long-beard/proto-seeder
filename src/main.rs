use anyhow::{Context, Error, Result};
use std::{
    fmt::Display,
    fs::File,
    io,
    io::{BufRead, BufReader, Lines, Read},
    path::Path,
};
use structopt::{clap::Arg, StructOpt};
use syn::Item;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args: Cli = Cli::from_args();
    // match read_lines(&args.path) {
    //     Ok(lines) => {
    //         for line in lines {
    //             println!("{}", line.unwrap());
    //         }
    //         Ok(())
    //     }
    //     Err(e) => Err(Error::from(e)),
    // }

    let mut file = File::open(&args.path).expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let ast = syn::parse_file(&src)?;
    for a in ast.items.iter() {
        if let Item::Enum(found_enum) = a {
            eprintln!("got enum {} ", found_enum.ident);
            if found_enum.ident == "Routes" {
                for v in found_enum.variants.iter() {
                    println!("routes => {}", v.ident);
                    if v.fields.len() > 0 {
                        for f in v.fields.iter() {
                            if let Some(i) = f.ident.as_ref() {
                                println!("specific_fields {}", i);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{} items", ast.items.len());

    Ok(())
}

fn read_lines(filename: &std::path::PathBuf) -> Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)
        .with_context(|| format!("could not read file `{}`", &filename.to_str().unwrap()))?;
    Ok(io::BufReader::new(file).lines())
}
