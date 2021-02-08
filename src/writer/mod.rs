//! This module checks the existing files and write or update their content.

use crate::content::SeedContent;
use indexmap::map::IndexMap;
use indicatif::ProgressBar;
use std::{
    fs,
    fs::{File, OpenOptions},
};

pub mod checker;
pub mod manager;

#[derive(PartialEq, Clone)]
pub enum FileOperation {
    /// When a file has been created.
    Create,
    /// When a file already exist and we will update.
    Update,
}

/// Manage the opening, creation and update of files.
pub struct ModulesWriter {
    /// The seed content that contains our modules and parent directory if.
    content: SeedContent,
    /// The progressbar used to log the success or infos or errors.
    pub pb: ProgressBar,
    /// The path of the current target file
    /// `cargo run -- -g ./examples/backbone_app/src/lib.rs` ->
    /// `./examples/backbone_app/src/lib.rs`.
    target_file_path: String,
    /// The actual directory containing the target file from the CLI
    /// `cargo run -- -g ./examples/backbone_app/src/lib.rs` ->
    /// `./examples/backbone_app/src/`.
    current_path: String,
    /// List of folder created by the Writer
    /// For now it should be one if #[modules_path = "pages"].
    folder_created: Vec<String,>,
    /// List of files created or updated by their path.
    files: IndexMap<String, (FileOperation, File,),>,
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
            target_file_path: root_file_path,
            // options: None,
            folder_created: vec![],
            files: IndexMap::new(),
        }
    }

    /// Create a folder and update the state of the Writer with it
    /// log or or info.
    fn create_folder(&mut self, path: &str,) -> &mut Self {
        if fs::create_dir(path,).is_ok() {
            self.log_ok(format!("created folder {}", path).as_str(),);
            self.folder_created.push(path.to_string(),);
        } else {
            self.log_info(format!("will use folder {}", path).as_str(),);
        }
        self
    }

    /// Open a filed to write and append code.
    /// If fail, then panic.
    /// If succeed then file is indexed so we can use it later with the path
    /// key.
    fn open_file_with_panic(&mut self, path: &str,) -> &mut Self {
        let file = OpenOptions::new()
            .write(true,)
            .read(true,)
            .append(true,)
            .open(path,)
            .unwrap_or_else(|_| panic!("Unable to update file , {}", path),);
        self.files
            .insert(path.to_string(), (FileOperation::Update, file,),);
        self
    }

    /// Standard open file to write and happen code ot it.
    fn open_file(path: &str,) -> std::io::Result<File,> {
        OpenOptions::new()
            .read(true,)
            .write(true,)
            .append(true,)
            .open(path,)
    }

    /// Create a new file and index it in the state.
    fn create_file(&mut self, path: String,) -> &mut Self {
        match File::create(path.clone(),) {
            Ok(file,) => {
                self.files
                    .insert(path.clone(), (FileOperation::Create, file,),);
                self.log_ok(format!("created file at {} ", path).as_str(),);
            },
            Err(e,) => {
                self.log_error(
                    format!(
                        "error {:?} when creating file at {}
",
                        e, path
                    )
                    .as_str(),
                );
            },
        }
        self
    }

    /// Create a new file and index it in the state or open a new file to.
    fn create_or_update_file(&mut self, path: String,) -> &mut Self {
        if let Ok(f,) = ModulesWriter::open_file(path.as_str(),) {
            self.files
                .insert(path.clone(), (FileOperation::Update, f,),);
            self.log_ok(format!("found file to update at {} ", path).as_str(),);
        } else {
            self.create_file(path,);
        }
        self
    }

    /// Log success in progress bar.
    pub fn log_ok(&mut self, msg: &str,) {
        self.pb.println(format!("[+] {}", msg).as_str(),);
    }

    /// Log info in progress bar.
    pub fn log_info(&mut self, msg: &str,) {
        self.pb.println(format!("[=>] {}", msg).as_str(),);
    }

    /// Log error in progress bar.
    pub fn log_error(&mut self, msg: &str,) {
        self.pb.println(format!("[!] {}", msg).as_str(),);
    }
}
