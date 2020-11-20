//! # proto_seeder
//!
//! `proto_seeder` is a experimental cli that generate Seed code based on
//! routing from [seed_routing](https://github.com/arn-the-long-beard/seed-routing) with RoutingModules

use crate::{
    content::SeedContent,
    parser::{find_model, find_routes},
    writer::module::{manager::ContentManager, ModulesWriter},
};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{File, OpenOptions},
    io::Read,
};
use structopt::StructOpt;

#[rustfmt::skip]
mod constants;

mod content;
mod parser;
mod writer;

/// Generate code from the Routes enum from the given file
#[derive(StructOpt, Debug)]
struct Cli {
    /// Will parse the given file and generate views, guards and modules from
    /// the Routes enum in the given file
    #[structopt(short, long)]
    generate: bool,

    /// Test debug mode
    /// For now this is dummy , we need to add log level later
    #[structopt(short, long)]
    debug: bool,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> anyhow::Result<(),> {
    let args: Cli = Cli::from_args();

    if args.generate {}

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120,);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ],)
            .template("{spinner:.blue} {msg}",),
    );

    let mut file = File::open(&args.path,)
        .unwrap_or_else(|_| panic!("Unable to open file , {}", &args.path.to_str().unwrap()),);

    let mut src = String::new();
    file.read_to_string(&mut src,)
        .expect("Unable to read file",);
    let parsed_file = syn::parse_file(&src,)?;

    pb.set_message("Searching for routes",);
    let enum_route = find_routes(&parsed_file,);
    let model = find_model(&parsed_file,);
    let current_path = args
        .path
        .parent()
        .unwrap()
        .to_str()
        .expect("should have gotten the current path",);
    if model.is_none() {
        pb.finish_with_message("No Model detected, so nothing will be created",);
        return Ok((),);
    } else if let Some(routes,) = enum_route {
        let seed_content = SeedContent::new(
            routes,
            Option::unwrap(model,),
            current_path,
            args.path
                .to_str()
                .expect("should get string of target file",),
        );

        pb.println(
            format!(
                "-> found {} locals view to create",
                &seed_content.local_views().iter().len()
            )
            .as_str(),
        );

        pb.println(
            format!(
                "-> found {} guards to create",
                &seed_content.guards().iter().len()
            )
            .as_str(),
        );
        pb.println(
            format!(
                "-> found {} modules to create",
                &seed_content.modules().iter().len()
            )
            .as_str(),
        );

        pb.println("[+] finished parsing the file",);

        pb.set_message(
            format!("creating local views on {}", &args.path.to_str().unwrap()).as_str(),
        );

        pb.set_message("Updating your files.",);

        let  writer = ModulesWriter::new(
            seed_content,
            pb,
            current_path.to_string(),
            args.path
                .to_str()
                .expect("should get string of target file",)
                .to_string(),
        );

        let mut content_manager = ContentManager::new(writer,);
        content_manager
            .add_or_update_imports()
            .add_or_update_content()
            .add_or_update_local_content();

        content_manager
            .writer
            .log_info(format!("Created {} new files", content_manager.file_created).as_str(),);

        content_manager
            .writer
            .log_info(format!("Updated {} files", content_manager.file_updated).as_str(),);

        content_manager
            .writer
            .log_info(format!("Ignored {} files", content_manager.file_ignored).as_str(),);

        content_manager.writer.pb.finish_with_message("Done",);
    } else {
        pb.finish_with_message("No routes detected, so nothing will be created",);
        return Ok((),);
    }
    Ok((),)
}
