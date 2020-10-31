use crate::{module::modules_path, view::variant_view_path_tuple};

use crate::view::variant_guard_path_tuple;
use convert_case::{Case, Casing};
use std::{
    fmt::Display,
    fs::File,
    io,
    io::{BufRead, BufReader, Lines, Read},
    path::Path,
};
use structopt::StructOpt;
use syn::{Attribute, Error, Item, Lit, LitStr, Meta, MetaNameValue, Result};

mod module;
mod view;
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

    let mut file = File::open(&args.path)
        .unwrap_or_else(|_| panic!("Unable to open file , {}", &args.path.to_str().unwrap()));

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let mut files_to_create: Vec<String> = vec![]; // each of them should be unique
    let mut local_views_to_create: Vec<String> = vec![]; // each of them should be unique
    let mut guards_to_create: Vec<String> = vec![]; // each of them should be unique
    let mut future_directory: Option<String> = None;

    let ast = syn::parse_file(&src)?;
    for a in ast.items.iter() {
        if let Item::Enum(found_enum) = a {
            eprintln!("got enum {} ", found_enum.ident);

            if found_enum.ident == "Routes" {
                future_directory = modules_path(found_enum.attrs.iter());
                for v in found_enum.variants.iter() {
                    println!("routes => {}", v.ident);

                    for f in v.fields.iter() {
                        if let Some(i) = f.ident.as_ref() {
                            println!(" specific_fields {}", &i);
                        }
                        // todo add nested tuple, struct and url payload
                        // extractor so we re inject them into the files we
                        // create no update
                    }

                    for a in v.attrs.iter() {
                        println!("attributes {}", a.path.get_ident().unwrap());
                    }
                    if let Some(view) = variant_view_path_tuple(v.ident.clone(), v.attrs.iter()) {
                        println!("The route will load local view");
                        local_views_to_create.push(format!("pub fn {}", view.1))
                    // new parser
                    // here , reuse
                    // code from macro
                    // derive ?
                    } else {
                        println!("Got a module");
                        files_to_create.push(v.ident.to_string().to_case(Case::Snake));
                    }

                    if let Some(guard) = variant_guard_path_tuple(v.ident.clone(), v.attrs.iter()) {
                        println!("Got a guard");
                        guards_to_create.push(format!("pub fn {}", guard.1));
                    }
                }
            }
        }
    }

    println!("{} items", ast.items.len());

    println!("{} files will be created", files_to_create.len());

    if let Some(path) = future_directory {
        println!("under directory {}", path);
    }

    println!(
        "{} local views will be created",
        local_views_to_create.len()
    );
    println!("{} guards will be created", guards_to_create.len());
    Ok(())

    // todo add counting maybe ?
}

pub fn get_string_from_attribute(attribute_name: &str, attr: &Attribute) -> Result<Option<LitStr>> {
    if !attr.path.is_ident(attribute_name) {
        return Ok(None); // not our attribute
    }
    match attr.parse_meta()? {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(name),
            ..
        }) => Some(Some(name)),
        _ => None,
    }
    .ok_or_else(|| Error::new_spanned(attr, &format!("expected #[{} = \"...\"]", attribute_name)))
}
