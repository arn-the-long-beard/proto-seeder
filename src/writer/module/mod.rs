use crate::{content::module::SeedModule, writer::write_space};
use indexmap::map::Iter;
use indicatif::ProgressBar;
use std::{
    fs,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

pub fn write_modules(
    seed_modules: Iter<String, SeedModule>,
    modules_path: Option<String>,
    pb: &ProgressBar,
    current_path: &Path,
) {
    if let Some(p) = modules_path.clone() {
        let current_path_as_string = current_path.to_str().unwrap().to_string();
        let p_string = format!("{}/{}", &current_path_as_string, p);
        if let Ok(_) = fs::create_dir(p_string.clone()) {
            pb.println(format!("[+] created folder {}", p_string).as_str());

            if let Err(e) = File::create(format!("{}/mod.rs", p_string)) {
                pb.println(
                    format!("[!] error {:?} when creating mod.rs at {} ", e, p_string).as_str(),
                );
            } else {
                pb.println(format!("[+] created at mod.rs at {} ", p_string).as_str());
            }
        } else {
            pb.println(format!("-> will use folder {}", p_string).as_str());
        }
    }

    for (module_name, module) in seed_modules {
        let mut is_new_file: bool = true;
        if let Some(parent_module_file) = modules_path.clone() {
            if let Ok(mut file) = OpenOptions::new().write(true).append(true).open(format!(
                "./{}/{}/mod.rs",
                current_path.to_str().unwrap(),
                parent_module_file
            )) {
                let res = file.write_all(format!("pub mod {};\n", module_name).as_ref());

                if let Err(e) = res {
                    pb.println(
                        format!(
                            "[!] error {:?} while updating parent module {}/{}/mod.rs",
                            e,
                            current_path.to_str().unwrap(),
                            parent_module_file
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
            format!("{}/{}.rs)", current_path.to_str().unwrap(), module_name,)
        } else {
            format!(
                "{}/{}/{}.rs",
                current_path.to_str().unwrap(),
                modules_path.clone().unwrap(),
                module_name
            )
        };
        let existing_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path.clone());

        if existing_file.is_ok() {
            pb.println(format!("Found file to update  => {}", file_path).as_str());
            file_to_create_or_update = existing_file.ok();
            is_new_file = false;
        } else {
            pb.println(format!("Will create new file => {}", file_path).as_str());

            let create_file = File::create(file_path.clone());
            if let Err(e) = create_file {
                pb.println(
                    format!("[!] Error {:?} while creating file at {}", e, file_path).as_str(),
                );
            } else {
                file_to_create_or_update = create_file.ok();
                is_new_file = true;
            }
        }

        if let Some(mut file) = file_to_create_or_update {
            if is_new_file {
                const IMPORT_SEED: &str = r###"use seed::{prelude::*, *};"###;
                let res_import = file.write_all(format!("{}\n", IMPORT_SEED).as_ref());

                if let Err(e) = res_import {
                    pb.println(format!(
                        "[!] could not write Seed import  for module {} [ => ] in  {}  because \
                         {:?}",
                        module.origin_route().as_ref().unwrap().name,
                        file_path,
                        e
                    ));
                }
            }
            let res = file.write_all(format!("{}\n", module.init().to_string()).as_ref());

            write_space(&file);
            if let Err(e) = res {
            } else {
                pb.println(format!(
                    "[+] created init for module {} [ => ] in  {} ()",
                    module.origin_route().as_ref().unwrap().name,
                    file_path,
                ));
            }

            let res = file.write_all(format!("{}\n", module.model().to_string()).as_ref());
            write_space(&file);
            if let Err(e) = res {
            } else {
                pb.println(format!(
                    "[+] created Model for module {} [ => ] in  {} ()",
                    module.origin_route().as_ref().unwrap().name,
                    file_path,
                ));
            }

            let res = file.write_all(format!("{}\n", module.routes().to_string()).as_ref());
            write_space(&file);
            if let Err(e) = res {
            } else {
                pb.println(format!(
                    "[+] created pub enum Routes  for module {} [ => ] in  {} ()",
                    module.origin_route().as_ref().unwrap().name,
                    file_path,
                ));
            }

            let res = file.write_all(format!("{}\n", module.msg().to_string()).as_ref());
            write_space(&file);
            if let Err(e) = res {
            } else {
                pb.println(format!(
                    "[+] created Message for module {} [ => ] in  {} ()",
                    module.origin_route().as_ref().unwrap().name,
                    file_path,
                ));
            }

            let res = file.write_all(format!("{}\n", module.view().to_string()).as_ref());
            write_space(&file);
            if let Err(e) = res {
            } else {
                pb.println(format!(
                    "[+] created pub fn view() for module {} [ => ] in  {} ()",
                    module.origin_route().as_ref().unwrap().name,
                    file_path,
                ));
            }
        } else {
            pb.println(format!("Could not update the file {}", file_path.clone()).as_str());
        }
    }
}
