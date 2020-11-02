use crate::{
    content::{module::SeedModule, SeedRoute},
    writer::write_space,
};
use indexmap::map::Iter;
use indicatif::ProgressBar;
use std::{
    fs,
    fs::{File, OpenOptions},
    io::Write,
};

pub fn write_modules(
    seed_modules: Iter<String, SeedModule>,
    modules_path: Option<String>,
    pb: &ProgressBar,
) {
    if let Some(p) = modules_path.clone() {
        if let Ok(_) = fs::create_dir(p.clone()) {
            pb.println(format!("[+] created folder {}", p).as_str());

            if let Err(e) = File::create(format!("./{}/mod.rs", p)) {
                pb.println(format!("[!] error {:?} when creating mod.rs at {} ", e, p).as_str());
            } else {
                pb.println(format!("[+] created at mod.rs at {} ", p).as_str());
            }
        } else {
            pb.println(format!("-> will use folder {}", p).as_str());
        }
    }

    for (module_name, module) in seed_modules {
        if let Some(parent_module_file) = modules_path.clone() {
            if let Ok(mut file) = OpenOptions::new()
                .write(true)
                .append(true)
                .open(format!("./{}/mod.rs", parent_module_file))
            {
                let res = file.write_all(format!("pub mod {};\n", module_name).as_ref());

                if let Err(e) = res {
                    pb.println(
                        format!(
                            "[!] error {:?} while updating parent module ./{}/mod.rs",
                            e, parent_module_file
                        )
                        .as_str(),
                    );
                }
            } else {
                pb.println(
                    format!(
                        "[!] error while updating parent module {}",
                        parent_module_file
                    )
                    .as_str(),
                );
            }
        }

        let mut file_to_create_or_update: Option<File> = None;
        // let create_folder: Option<String>;
        let file_path = if modules_path.clone().is_none() {
            format!("{}.rs)", module_name)
        } else {
            format!("./{}/{}.rs", modules_path.clone().unwrap(), module_name)
        };
        let existing_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path.clone());

        if existing_file.is_ok() {
            pb.println(format!("Found file to update  => {}", file_path).as_str());
            file_to_create_or_update = existing_file.ok();
        } else {
            pb.println(format!("Will create new file => {}", file_path).as_str());

            let create_file = File::create(file_path.clone());
            if let Err(e) = create_file {
                pb.println(
                    format!("[!] Error {:?} while creating file at {}", e, file_path).as_str(),
                );
            } else {
                file_to_create_or_update = create_file.ok();
            }
        }

        if let Some(mut file) = file_to_create_or_update {
            let res = file.write_all(format!("{}\n", module.init().to_string()).as_ref());
            write_space(&file);
            if let Err(e) = res {
            } else {
                pb.println(format!(
                    "[+] created init for route {} [ => ] in  {} ()",
                    module.origin_route().as_ref().unwrap().name,
                    file_path,
                ));
            }
        } else {
            pb.println(format!("Could not update the file {}", file_path.clone()).as_str());
        }
    }
}
