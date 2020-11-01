use crate::{content::SeedContent, writer::write_local_views};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{File, OpenOptions},
    io::Read,
    thread,
    time::Duration,
};
use structopt::StructOpt;
use syn::{Attribute, Error, Item, ItemEnum, ItemStruct, Lit, LitStr, Meta, MetaNameValue, Result};

mod constants;
mod content;
mod module;
mod view;
mod writer;
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

    // let mut files_to_create: Vec<String> = vec![]; // each of them should be
    // unique let mut local_views_to_create: Vec<String> = vec![]; // each of
    // them should be unique let mut guards_to_create: Vec<String> = vec![]; //
    // each of them should be unique let mut future_directory: Option<String> =
    // None;
    //
    // let mut found_routes: Option<String> = None;

    let parsed_file = syn::parse_file(&src)?;

    pb.set_message("Searching for routes");
    let enum_route = find_routes(&parsed_file);
    let model = find_model(&parsed_file);

    if model.is_none() {
        pb.finish_with_message("No Model detected, so nothing will be created");
        return Ok(());
    } else if let Some(routes) = enum_route {
        let seed_content = SeedContent::new(routes, Option::unwrap(model));

        pb.println("[+] finished parsing the file");

        pb.set_message(
            format!("creating local views on {}", &args.path.to_str().unwrap()).as_str(),
        );

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&args.path)
            .unwrap_or_else(|_| panic!("Unable to update file , {}", &args.path.to_str().unwrap()));

        write_local_views(seed_content.local_views().iter(), file, &pb);

        pb.set_message("Updating your files.");
        thread::sleep(Duration::from_secs(3));
        pb.println("[+] Files updated"); // todo add files names

        pb.set_message("Creating new files.");
        pb.println("[+] Files created"); // todo add files names
        thread::sleep(Duration::from_secs(3));

        pb.finish_with_message("Done");
    } else {
        pb.finish_with_message("No routes detected, so nothing will be created");
        return Ok(());
    }

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
        constants::{_FILE_WITHOUT_ROUTES_NOR_MODEL, _FILE_WITH_ROUTES_AND_MODEL},
        find_model, find_routes,
    };

    #[test]
    fn test_find_routes() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL).unwrap();
        let route = find_routes(&parsed_file);

        assert_eq!(route.is_some(), true);

        let parsed_file = syn::parse_file(_FILE_WITHOUT_ROUTES_NOR_MODEL).unwrap();
        let route = find_routes(&parsed_file);

        assert_eq!(route.is_some(), false)
    }

    #[test]
    fn test_find_model() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL).unwrap();
        let model = find_model(&parsed_file);

        assert_eq!(model.is_some(), true);

        let parsed_file = syn::parse_file(_FILE_WITHOUT_ROUTES_NOR_MODEL).unwrap();
        let model = find_model(&parsed_file);

        assert_eq!(model.is_some(), false)
    }
}
