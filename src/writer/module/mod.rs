use crate::content::SeedContent;
use indexmap::map::IndexMap;
use indicatif::ProgressBar;
use std::{
    fs,
    fs::{File, OpenOptions},
    io::Write,
};

// /// Specific options gotten from the command line
// pub struct WritingOptions {
//     no_write_routes: bool,
//     recursive: bool,
// }

#[derive(PartialEq, Clone)]
pub enum FileOperation {
    /// When a file has been created
    Create,
    /// When a file already exist and we will update
    Update,
    // Error,
}

/// Manage the writing of the content in seed
pub struct ModulesWriter {
    /// The seed content that contains our modules and parent directory if any
    content: SeedContent,
    /// The progressbar used to log the success or infos or errors
    pub pb: ProgressBar,
    /// The path of the current target file
    /// `cargo run -- -g ./examples/backbone_app/src/lib.rs` ->
    /// `./examples/backbone_app/src/lib.rs`
    root_file_path: String,
    /// The actual directory containing the target file from the CLI
    /// `cargo run -- -g ./examples/backbone_app/src/lib.rs` ->
    /// `./examples/backbone_app/src/`
    current_path: String,
    /// List of folder created by the Writer
    /// For now it should be one if #[modules_path = "pages"]
    folder_created: Vec<String>,
    /// List of files created or updated by their path
    files: IndexMap<String, (FileOperation, File)>,
}

impl ModulesWriter {
    pub fn new(
        content: SeedContent,
        pb: indicatif::ProgressBar,
        current_path: String,
        root_file_path: String,
    ) -> Self {
        ModulesWriter {
            content,
            pb,
            current_path,
            root_file_path,
            // options: None,
            folder_created: vec![],
            files: IndexMap::new(),
        }
    }

    /// Create a folder and update the state of the Writer with it
    /// log or or info
    fn create_folder(&mut self, path: &str) -> &mut Self {
        if let Ok(_) = fs::create_dir(path) {
            self.log_ok(format!("created folder {}", path).as_str());
            self.folder_created.push(path.to_string());
        } else {
            self.log_info(format!("will use folder {}", path).as_str());
        }
        self
    }

    /// open a filed to write and append code ot it
    /// if fail, then panic
    /// if succeed then file is indexed so we can use it later with the path as
    /// key
    fn open_file_with_panic(&mut self, path: &str) -> &mut Self {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|_| panic!("Unable to update file , {}", path));
        self.files
            .insert(path.to_string(), (FileOperation::Update, file));
        self
    }

    /// Standard open file to write and happen code ot it
    fn open_file(path: &str) -> std::io::Result<File> {
        OpenOptions::new().write(true).append(true).open(path)
    }

    /// Write on the file for given path and content
    /// Log ok or error
    /// State updated
    /// //TODO could save error state
    fn write_on_file(&mut self, file_path: &str, file_content: &str) -> &mut Self {
        if let Some((_, file)) = self.files.get_mut(file_path) {
            if let Err(e) = file.write_all(file_content.as_ref()) {
                self.log_error(
                    format!(
                        "error {:?} when writing {} at {} ",
                        e, file_content, file_path
                    )
                    .as_str(),
                );
            } else {
                self.log_ok(format!("updated {} ", file_path).as_str());
            }
        } else {
            self.log_error(format!("file not found at {}", file_path).as_str());
        }

        self
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
        if let Some((_, file)) = self.files.get_mut(file_path) {
            if let Err(e) = file.write_all(file_content.as_ref()) {
                self.log_error(
                    format!(
                        "error {:?} when writing {} at {} for {}",
                        e, file_content, file_path, message
                    )
                    .as_str(),
                );
            } else {
                self.log_ok(format!("updated {} for {}", file_path, message).as_str());
            }
        } else {
            self.log_error(
                format!(
                    "file not found at {} when trying to {} ",
                    file_path, message
                )
                .as_str(),
            );
        }

        self
    }

    /// Create a new file and index it in the state
    fn create_file(&mut self, path: String) -> &mut Self {
        match File::create(path.clone()) {
            Ok(file) => {
                self.files
                    .insert(path.clone(), (FileOperation::Create, file));
                self.log_ok(format!("created file at {} ", path).as_str());
            }
            Err(e) => {
                self.log_error(format!("error {:?} when creating file at {} ", e, path).as_str());
            }
        }
        self
    }
    /// Create a new file and index it in the state or open a new file to update
    fn create_or_update_file(&mut self, path: String) -> &mut Self {
        if let Ok(f) = ModulesWriter::open_file(path.as_str()) {
            self.files.insert(path.clone(), (FileOperation::Update, f));
            self.log_ok(format!("found file at {} ", path).as_str())
        } else {
            self.create_file(path);
        }
        self
    }
    /// create parent module if needed and then create sub modules with their
    /// content
    pub fn run(&mut self) -> &mut Self {
        let mut mod_file_path: Option<String> = None;

        if let Some(parent_module_path) = &self.content.directory().clone() {
            if parent_module_path.contains('/') {
                self.log_error("Does not support multiple directory for now");
                return self;
            }

            let root_path = &self.current_path; //my_app
            let root_file_path: String = String::from(&self.root_file_path); //my_app/lib.rs

            let new_folder_path: String = format!("{}/{}", &root_path, parent_module_path.clone()); //my_app/pages
            mod_file_path = Some(format!("{}/mod.rs", &new_folder_path)); //my_app/pages/mod.rs

            self.create_folder(&new_folder_path)
                .create_or_update_file(mod_file_path.clone().unwrap())
                .open_file_with_panic(&root_file_path)
                .write_on_file(
                    &root_file_path,
                    format!("mod {}; ", parent_module_path.clone()).as_str(),
                );
        }

        for (module_name, module) in self.content.modules().clone() {
            let mut file_path: Option<String> = None;
            if let Some(mod_file) = mod_file_path.clone() {
                file_path = Some(format!(
                    "{}/{}/{}.rs",
                    &self.current_path,
                    &self.content.directory().clone().unwrap(),
                    module_name
                ));
                self.write_on_file(
                    mod_file.clone().as_str(),
                    format!("pub mod {} ;\n", module_name).as_str(),
                );
            } else {
                file_path = Some(format!("{}/{}.rs", &self.current_path.clone(), module_name));
                let p = &self.root_file_path.clone().to_string();
                self.open_file_with_panic(&p.clone()).write_on_file(
                    p.clone().as_str(),
                    format!("pub mod {} ;\n", module_name).as_str(),
                );
            }

            if let Some(path) = file_path {
                self.create_or_update_file(path.clone());

                match self.files.get_mut(&path) {
                    None => {}
                    Some((operation, _)) => {
                        if operation.clone().eq(&FileOperation::Create) {
                            const IMPORT_SEED: &str = r###"use seed::{prelude::*, *};"###;
                            self.write_on_file(
                                path.clone().as_str(),
                                format!("{}\n", IMPORT_SEED).as_str(),
                            );
                        }
                        self.write_on_file_with_custom_message(
                            &path,
                            module.init(),
                            "adding pub fn init()",
                        )
                        .write_on_file_with_custom_message(
                            &path,
                            module.model(),
                            "adding pub struct Model{}",
                        )
                        .write_on_file_with_custom_message(
                            &path,
                            module.routes(),
                            "adding pub enum Routes{} ",
                        )
                        .write_on_file_with_custom_message(
                            &path,
                            module.msg(),
                            "adding pub enum Msg{}",
                        )
                        .write_on_file_with_custom_message(
                            &path,
                            module.update(),
                            "adding pub fn update()",
                        )
                        .write_on_file_with_custom_message(
                            &path,
                            module.view(),
                            "adding pub fn view()",
                        );
                    }
                }
            }
        }
        self
    }

    /// Log success in progress bar
    pub fn log_ok(&self, msg: &str) {
        self.pb.println(format!("[+] {}", msg).as_str());
    }

    /// Log info in progress bar
    pub fn log_info(&self, msg: &str) {
        self.pb.println(format!("[=>] {}", msg).as_str());
    }

    /// Log error in progress bar
    pub fn log_error(&self, msg: &str) {
        self.pb.println(format!("[!] {}", msg).as_str());
    }

    pub fn get_number_of_created_file(&self) -> u32 {
        let mut n = 0;

        for (_, (op, _)) in self.files.iter() {
            if op.eq(&FileOperation::Create) {
                n += 1;
            }
        }
        n
    }

    pub fn get_number_of_updated_file(&self) -> u32 {
        let mut n = 0;

        for (_, (op, _)) in self.files.iter() {
            if op.eq(&FileOperation::Update) {
                n += 1;
            }
        }
        n
    }
}
