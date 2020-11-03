use crate::{
    content::SeedContent,
    parser::{find_model, find_routes},
    writer::{guard::write_guards, module::ModulesWriter, view::write_local_views},
};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{File, OpenOptions},
    io::Read,
};
use structopt::StructOpt;

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

fn main() -> anyhow::Result<()> {
    let args: Cli = Cli::from_args();

    if args.generate {}

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

    let mut file = File::open(&args.path)
        .unwrap_or_else(|_| panic!("Unable to open file , {}", &args.path.to_str().unwrap()));

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");
    let parsed_file = syn::parse_file(&src)?;

    pb.set_message("Searching for routes");
    let enum_route = find_routes(&parsed_file);
    let model = find_model(&parsed_file);

    if model.is_none() {
        pb.finish_with_message("No Model detected, so nothing will be created");
        return Ok(());
    } else if let Some(routes) = enum_route {
        let seed_content = SeedContent::new(routes, Option::unwrap(model));

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

        pb.println("[+] finished parsing the file");

        pb.set_message(
            format!("creating local views on {}", &args.path.to_str().unwrap()).as_str(),
        );

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&args.path)
            .unwrap_or_else(|_| panic!("Unable to update file , {}", &args.path.to_str().unwrap()));

        let current_path = args.path.parent().unwrap();

        write_local_views(seed_content.local_views().iter(), &file, &pb);

        write_guards(seed_content.guards().iter(), &file, &pb);

        pb.set_message("Updating your files.");

        let mut writer = ModulesWriter::new(
            seed_content,
            pb,
            current_path
                .to_str()
                .expect("should get string of current path")
                .to_string(),
            args.path
                .to_str()
                .expect("should get string of target file")
                .to_string(),
        );

        writer.run();
        writer
            .pb
            .println(format!("Created {} new files", writer.get_number_of_created_file()).as_str());
        writer
            .pb
            .println(format!("Updated {} files", writer.get_number_of_updated_file()).as_str());
        writer.pb.finish_with_message("Done");
    } else {
        pb.finish_with_message("No routes detected, so nothing will be created");
        return Ok(());
    }
    Ok(())
}
