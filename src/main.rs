use crate::{module::modules_path, view::variant_view_path_tuple};

use crate::view::variant_guard_path_tuple;
use convert_case::{Case, Casing};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::File, io::Read, thread, time::Duration};
use structopt::StructOpt;
use syn::{Attribute, Error, Item, ItemEnum, ItemStruct, Lit, LitStr, Meta, MetaNameValue, Result};

mod constants;
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

fn main() -> anyhow::Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ])
            .template("{spinner:.blue} {msg}"),
    );

    let args: Cli = Cli::from_args();
    let mut file = File::open(&args.path)
        .unwrap_or_else(|_| panic!("Unable to open file , {}", &args.path.to_str().unwrap()));

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let mut files_to_create: Vec<String> = vec![]; // each of them should be unique
    let mut local_views_to_create: Vec<String> = vec![]; // each of them should be unique
    let mut guards_to_create: Vec<String> = vec![]; // each of them should be unique
    let mut future_directory: Option<String> = None;

    let mut found_routes: Option<String> = None;

    let parsed_file = syn::parse_file(&src)?;

    pb.set_message("Searching for routes");
    let enum_route = find_routes(&parsed_file);

    let model = find_model(&parsed_file);

    if let Some(routes) = enum_route {
        eprintln!("got enum {} ", routes.ident);
        found_routes = Some(routes.ident.to_string());
        future_directory = modules_path(routes.attrs.iter());
        for v in routes.variants.iter() {
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
    } else {
        pb.finish_with_message("No routes detected, so nothing will be created");
        return Ok(());
    }

    println!("{} items", &parsed_file.items.len());

    println!("{} files will be created", files_to_create.len());

    if let Some(path) = future_directory {
        println!("under directory {}", path);
    }

    println!(
        "{} local views will be created",
        local_views_to_create.len()
    );
    println!("{} guards will be created", guards_to_create.len());

    pb.println("[+] finished parsing the file");

    pb.set_message("Updating your files.");
    thread::sleep(Duration::from_secs(3));
    pb.println("[+] Files updated"); // todo add files names

    pb.set_message("Creating new files.");
    pb.println("[+] Files created"); // todo add files names
    thread::sleep(Duration::from_secs(3));

    pb.finish_with_message("Done");

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

fn find_routes(file: &syn::File) -> Option<ItemEnum> {
    file.items.iter().find_map(|item| -> Option<ItemEnum> {
        if let Item::Enum(found_enum) = item {
            if found_enum.ident == "Routes" {
                return Some(found_enum.clone());
            }
        }
        None
    })
}

fn find_model(file: &syn::File) -> Option<ItemStruct> {
    file.items.iter().find_map(|item| -> Option<ItemStruct> {
        if let Item::Struct(model_struct) = item {
            if model_struct.ident == "Model" {
                return Some(model_struct.clone());
            }
        }
        None
    })
}

#[cfg(test)]
mod test {
    use crate::{
        constants::{FILE_WITHOUT_MODEL, FILE_WITHOUT_ROUTES, FILE_WITH_ROUTES},
        find_model, find_routes,
    };

    #[test]
    fn test_find_routes() {
        let parsed_file = syn::parse_file(FILE_WITH_ROUTES).unwrap();
        let route = find_routes(&parsed_file);

        assert_eq!(route.is_some(), true);

        let parsed_file = syn::parse_file(FILE_WITHOUT_ROUTES).unwrap();
        let route = find_routes(&parsed_file);

        assert_eq!(route.is_some(), false)
    }

    #[test]
    fn test_find_model() {
        let parsed_file = syn::parse_file(FILE_WITH_ROUTES).unwrap();
        let model = find_model(&parsed_file);

        assert_eq!(model.is_some(), true);

        let parsed_file = syn::parse_file(FILE_WITHOUT_MODEL).unwrap();
        let model = find_model(&parsed_file);

        assert_eq!(model.is_some(), false)
    }
}
