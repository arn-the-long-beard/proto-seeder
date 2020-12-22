//! Manage writing and update of content on files
use crate::{
    content::{
        guard::SeedGuard,
        module::{
            import::{ImportModule, ParentModuleType},
            SeedModule,
        },
        view::SeedView,
    },
    writer::{checker::Checker, FileOperation, ModulesWriter},
};
use indexmap::map::IndexMap;
use std::io::{Read, Write};

/// Manage the SeedContent on modules
/// Uses the checker to know if need to create or ignore content
pub struct ContentManager {
    pub file_ignored: u32,
    pub file_created: u32,
    pub file_updated: u32,
    /// The module writer manage the creation, reading and update of files
    pub writer: ModulesWriter,
}

impl ContentManager {
    /// Use the writer to access file and update their content
    pub fn new(writer: ModulesWriter) -> ContentManager {
        ContentManager {
            file_ignored: 0,
            file_created: 0,
            file_updated: 0,
            writer,
        }
    }

    /// wrote on the file for given path and content with custom message
    /// Log ok or error
    /// State updated
    fn write_on_file_with_custom_message(
        &mut self,
        file_path: &str,
        file_content: &str,
        message: &str,
    ) -> &mut Self {
        if let Some((_, file)) = self.writer.files.get_mut(file_path) {
            if let Err(e) = file.write_all(file_content.as_ref()) {
                self.writer.log_error(
                    format!(
                        "error {:?} when writing {} at {} for {}",
                        e, file_content, file_path, message
                    )
                    .as_str(),
                );
            } else {
                self.writer
                    .log_ok(format!("updated {} for {}", file_path, message).as_str());
            }
        } else {
            self.writer.log_error(
                format!(
                    "file not found at {} when trying to {} ",
                    file_path, message
                )
                .as_str(),
            );
        }

        self
    }

    /// Write on the file for given path and content
    /// Log ok or error
    /// State updated
    /// //TODO could save error state
    fn write_on_file(&mut self, file_path: &str, file_content: &str) -> &mut Self {
        if let Some((_, file)) = self.writer.files.get_mut(file_path) {
            if let Err(e) = file.write_all(file_content.as_ref()) {
                self.writer.log_error(
                    format!(
                        "error {:?} when writing {} at {} ",
                        e, file_content, file_path
                    )
                    .as_str(),
                );
            } else {
                self.writer
                    .log_ok(format!("updated {} ", file_path).as_str());
            }
        } else {
            self.writer
                .log_error(format!("file not found at {}", file_path).as_str());
        }

        self
    }
    pub fn add_or_update_imports(&mut self) -> &mut Self {
        self.add_or_update_imports_from_module(self.writer.content.parent_module().clone())
    }

    /// Manage the imports for files
    ///
    ///
    ///
    fn add_or_update_imports_from_module(&mut self, import_module: ImportModule) -> &mut Self {
        let path = import_module.meta().filepath().to_string();

        match import_module.parent_type {
            ParentModuleType::TargetFile => {
                self.writer
                    .open_file_with_panic(self.writer.target_file_path.clone().as_str());
            }

            ParentModuleType::Folder => {
                self.writer
                    .create_folder(import_module.folder_path())
                    .create_or_update_file(path.clone());

                /// should be recursive but work only with lib.rs as import_module of the folder for now
                if !import_module.meta().clone().mod_import().is_empty() {
                    eprintln!("---------- detected module folder ");

                    eprintln!("---------- detected module folder ");
                    let mut root_module = ImportModule::new_target_file_module(
                        import_module.meta().import_file_location(),
                        import_module.meta().import_file_location(),
                    )
                    .unwrap();

                    root_module
                        .imports_names
                        .push(import_module.name().to_string());
                    root_module
                        .imports_content
                        .push(import_module.meta().mod_import().to_string());
                    self.add_or_update_imports_from_module(root_module);
                }
            }
        }
        let (op, file) = self.writer.files.get_mut(&path).unwrap();
        let mut imports = import_module.imports_to_write();
        match op {
            FileOperation::Update => {
                let mut src = String::new();
                let read = file.read_to_string(&mut src);
                if read.is_err() {
                    self.writer
                        .log_error(format!("Should read file for  {}", path).as_str());
                    self.writer
                        .log_error(format!("{:?}", read.unwrap_err()).as_str());
                }
                imports =
                    self.update_imports_to_write(imports.as_str(), &src, import_module.clone());

                if imports.is_empty() {
                    self.file_ignored += 1;
                } else {
                    self.file_updated += 1;
                }
            }
            FileOperation::Create => {
                self.file_created += 1;
            }
        }

        if !imports.is_empty() {
            self.write_on_file_with_custom_message(
                path.as_str(),
                imports.as_str(),
                format!("import import_module module => {}", imports).as_str(),
            );
        }

        self
    }

    /// Add the TEA content ->
    /// - pub fn init
    /// - pub struct Model
    /// - pub enum Routes
    /// - put enum Msg
    /// - pub fn update
    /// - pub fn view
    fn insert_content(&mut self, path: &str, module: SeedModule) {
        const IMPORT_SEED: &str = r###"use seed::{prelude::*, *};"###;
        self.write_on_file(&path, format!("{}\n", IMPORT_SEED).as_str())
            .write_on_file_with_custom_message(&path, module.init(), "adding pub fn init()")
            .write_on_file_with_custom_message(&path, module.model(), "adding pub struct Model{}")
            .write_on_file_with_custom_message(&path, module.routes(), "adding pub enum Routes{} ")
            .write_on_file_with_custom_message(&path, module.msg(), "adding pub enum Msg{}")
            .write_on_file_with_custom_message(&path, module.update(), "adding pub fn update()")
            .write_on_file_with_custom_message(&path, module.view(), "adding pub fn view()");
        self.file_created += 1;
    }

    /// Use the Checker to verify if content already exist and then update if
    /// needed
    fn update_content_if_needed(&mut self, path: &str, src: &str, module: SeedModule) {
        let check = Checker::store_content_for_check(src);
        let mut number_update = 0;
        if check.init_exist() {
            self.writer.log_info("file already has init");
        } else {
            number_update += 1;
            self.write_on_file_with_custom_message(&path, module.init(), "adding pub fn init()");
        }

        if check.model_exist() {
            self.writer.log_info("file already has Model");
        } else {
            number_update += 1;

            self.write_on_file_with_custom_message(
                &path,
                module.model(),
                "adding pub struct Model{}",
            );
        }
        if check.routes_exist() {
            self.writer.log_info("file already has Routes");
        } else {
            number_update += 1;

            self.write_on_file_with_custom_message(
                &path,
                module.routes(),
                "adding pub enum Routes{} ",
            );
        }
        if check.message_exist() {
            self.writer.log_info("file already has Msg");
        } else {
            number_update += 1;

            self.write_on_file_with_custom_message(
                &path,
                module.routes(),
                "adding pub enum Msg{} ",
            );
        }
        if check.update_exist() {
            self.writer.log_info("file already has update");
        } else {
            number_update += 1;
            self.write_on_file_with_custom_message(
                &path,
                module.update(),
                "adding pub fn update() ",
            );
        }
        if check.view_exist() {
            self.writer.log_info("file already has view");
        } else {
            number_update += 1;

            self.write_on_file_with_custom_message(&path, module.view(), "adding pub fn view() ");
        }

        if number_update == 0 {
            self.file_ignored += 1;
        }
        if number_update != 0 {
            self.file_updated += 1;
        }
    }

    /// Replace the existing imports.
    /// This is so dirty code. I dislike it
    fn update_imports_to_write(
        &mut self,
        imports: &str,
        src: &str,
        parent_module: ImportModule,
    ) -> String {
        let mut new_imports: String = imports.to_string();
        let list = Checker::return_mod_if_exist(src, parent_module.clone());
        for l in list.iter() {
            if let Some(i) = parent_module.imports_names.iter().position(|n| n == l) {
                let code = parent_module.imports_content.get(i);
                if let Some(c) = code {
                    new_imports = new_imports.replace(c, "");
                    self.writer.log_info(
                        format!(
                            "No need to update imports on {} for {}",
                            parent_module.meta().filepath(),
                            l
                        )
                        .as_str(),
                    );
                }
            }
        }
        new_imports.trim().to_string()
    }

    /// Will check if content already exist and create or merge
    pub fn add_or_update_content(&mut self) -> &mut Self {
        let map = self.writer.content.modules().clone();
        let iter = map.iter();
        for (_, module) in iter {
            let path = module.meta().filepath().to_string();
            self.writer.create_or_update_file(String::from(&path));
            let (op, file) = self.writer.files.get_mut(&path).unwrap();
            match op {
                FileOperation::Update => {
                    let mut src = String::new();

                    let read = file.read_to_string(&mut src);

                    if read.is_err() {
                        self.writer
                            .log_error(format!("Should read file for  {}", &path).as_str());
                        self.writer
                            .log_error(format!("{:?}", read.unwrap_err()).as_str());
                    }
                    self.update_content_if_needed(&path, src.as_str(), module.clone());
                }
                FileOperation::Create => {
                    self.insert_content(&path, module.clone());
                }
            }
        }
        self
    }

    /// For writing guard and local view on the target file
    /// Could be extended for custom content maybe on any modules
    pub fn add_or_update_local_content(&mut self) -> &mut Self {
        let path = self.writer.target_file_path.to_string();
        let views = self.writer.content.local_views().clone();
        let guards = self.writer.content.guards().clone();
        let view_updates = self.write_local_views(&path, &views);
        let guard_updates = self.write_local_guards(&path, &guards);
        let updates: u32 = view_updates + guard_updates;
        if updates == 0 {
            self.file_ignored += 1;
        }
        self
    }

    /// Write local views on the targeted path
    fn write_local_views(&mut self, path: &str, views: &IndexMap<String, SeedView>) -> u32 {
        let mut updates_number = 0;
        for (view_name, view) in views {
            let mut src = String::new();
            self.writer.create_or_update_file(path.to_string());
            let mut file = &self.writer.files.get_mut(path).unwrap().1;
            let read = file.read_to_string(&mut src);

            if read.is_err() {
                self.writer
                    .log_error(format!("Should read file for  {}", &path).as_str());
                self.writer
                    .log_error(format!("{:?}", read.unwrap_err()).as_str());
            }

            let check = Checker::check_local_function_exist(view_name, src.as_str());
            if check {
                self.writer.log_info(
                    format!(
                        "No need to create view for route {} [ => ] as fn {} ()",
                        view.route.name, view_name
                    )
                    .as_str(),
                );
            } else {
                self.write_on_file_with_custom_message(
                    path,
                    view.content.as_str(),
                    format!(
                        "writing local view {} for route {}",
                        view.name, view.route.name
                    )
                    .as_str(),
                );
                self.write_on_file_with_custom_message(path, "\n", "Added indentation");

                updates_number += 1;
            }
        }
        if updates_number != 0 {
            self.file_updated += 1;
        }

        updates_number
    }

    /// Write local guard and redirect on the targeted path
    fn write_local_guards(&mut self, path: &str, guards: &IndexMap<String, SeedGuard>) -> u32 {
        let mut updates_number = 0;
        for (guard_name, guard) in guards {
            let mut src = String::new();
            self.writer.create_or_update_file(path.to_string());
            let mut file = &self.writer.files.get_mut(path).unwrap().1;
            let read = file.read_to_string(&mut src);

            if read.is_err() {
                self.writer
                    .log_error(format!("Should read file for  {}", &path).as_str());
                self.writer
                    .log_error(format!("{:?}", read.unwrap_err()).as_str());
            }

            let check = Checker::check_local_function_exist(guard_name, src.as_str());
            if check {
                self.writer.log_info(
                    format!("No need to create guard [ => ] as fn {} ()", guard_name,).as_str(),
                );
            } else {
                self.write_on_file_with_custom_message(
                    path,
                    guard.content.as_str(),
                    format!("writing local guard as {}", guard_name).as_str(),
                );
                self.write_on_file_with_custom_message(path, "\n", "Added indentation");

                let check_redirect =
                    Checker::check_local_function_exist(&guard.redirect.name, src.as_str());
                if check_redirect {
                    self.writer.log_info(
                        format!(
                            "No need to create redirect {} for [ => ] {} ()",
                            &guard.redirect.name, guard_name,
                        )
                        .as_str(),
                    );
                } else {
                    self.write_on_file_with_custom_message(
                        path,
                        &guard.redirect.content.as_str(),
                        format!("writing redirect for guard as {}", &guard.redirect.name).as_str(),
                    );
                    self.write_on_file_with_custom_message(path, "\n", "Added indentation");
                }

                updates_number += 1;
            }
        }
        if updates_number != 0 {
            self.file_updated += 1;
        }
        updates_number
    }
}
