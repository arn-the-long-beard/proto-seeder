use crate::{
    content::module::{
        import::{ImportModule, ParentModuleType},
        SeedModule,
    },
    writer,
    writer::module::{checker::Checker, FileOperation, ModulesWriter},
};
use std::{
    fs,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

/// Do manage the SeedContent on modules
/// Do use the checker to know if need to create or merge content
pub struct ContentManager {
    number_file_updates: u32,
    number_file_created: u32,
    pub writer: ModulesWriter,
}

impl ContentManager {
    pub fn new(writer: ModulesWriter) -> ContentManager {
        ContentManager {
            number_file_updates: 0,
            number_file_created: 0,
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
        let parent = self.writer.content.parent_module().clone();
        let path = parent.meta().filepath().to_string();

        match parent.parent_type {
            ParentModuleType::TargetFile => {
                self.writer
                    .open_file_with_panic(self.writer.target_file_path.clone().as_str());
            }

            ParentModuleType::Folder => {
                self.writer
                    .create_folder(parent.folder_path())
                    .create_or_update_file(path.clone());
            }
        }
        let (op, file) = self.writer.files.get_mut(&path).unwrap();
        let mut imports = parent.imports_to_write();
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
                imports = self.update_imports_to_write(imports.as_str(), &src, parent);
            }
            FileOperation::Create => {}
        }

        self.write_on_file_with_custom_message(
            path.as_str(),
            imports.as_str(),
            format!("import parent module => {}", imports).as_str(),
        );

        self
    }

    fn insert_content(&mut self, path: &str, module: SeedModule) {
        const IMPORT_SEED: &str = r###"use seed::{prelude::*, *};"###;
        self.write_on_file(&path, format!("{}\n", IMPORT_SEED).as_str())
            .write_on_file_with_custom_message(
                &path,
                module.init(),
                "adding
pub fn init()",
            )
            .write_on_file_with_custom_message(&path, module.model(), "adding pub struct Model{}")
            .write_on_file_with_custom_message(
                &path,
                module.routes(),
                "adding pub enum
Routes{} ",
            )
            .write_on_file_with_custom_message(&path, module.msg(), "adding pub enum Msg{}")
            .write_on_file_with_custom_message(
                &path,
                module.update(),
                "adding pub fn
update()",
            )
            .write_on_file_with_custom_message(&path, module.view(), "adding pub fn view()");
    }

    fn update_content_if_needed(&mut self, path: &str, src: &str, module: SeedModule) {
        let check = Checker::store_content_for_check(src);

        if check.init_exist() {
            self.writer.log_info("file already has init");
        } else {
            self.write_on_file_with_custom_message(&path, module.init(), "adding pub fn init()");
        }

        if check.model_exist() {
            self.writer.log_info("file already has Model");
        } else {
            self.write_on_file_with_custom_message(
                &path,
                module.model(),
                "adding pub struct Model{}",
            );
        }
        if check.routes_exist() {
            self.writer.log_info("file already has Routes");
        } else {
            self.write_on_file_with_custom_message(
                &path,
                module.routes(),
                "adding pub enum Routes{} ",
            );
        }
        if check.message_exist() {
            self.writer.log_info("file already has Msg");
        } else {
            self.write_on_file_with_custom_message(
                &path,
                module.routes(),
                "adding pub enum Msg{} ",
            );
        }
        if check.update_exist() {
            self.writer.log_info("file already has update");
        } else {
            self.write_on_file_with_custom_message(
                &path,
                module.update(),
                "adding pub fn update() ",
            );
        }
        if check.view_exist() {
            self.writer.log_info("file already has view");
        } else {
            self.write_on_file_with_custom_message(&path, module.view(), "adding pub fn view() ");
        }
    }

    /// Replace the existing imports
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
        new_imports
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
}
