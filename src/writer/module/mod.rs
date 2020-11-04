use crate::{
    content::{
        module::{
            import::{ImportModule, ParentModuleType},
            SeedModule,
        },
        SeedContent,
    },
    parser::*,
    writer::module::checker::Checker,
};
use indexmap::map::{IndexMap, Iter};
use indicatif::ProgressBar;
use std::{
    fs,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

mod checker;
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
    /// The seed content that contains our modules and parent directory if
    content: SeedContent,
    /// The progressbar used to log the success or infos or errors
    pub pb: ProgressBar,
    /// The path of the current target file
    /// `cargo run -- -g ./examples/backbone_app/src/lib.rs` ->
    /// `./examples/backbone_app/src/lib.rs`
    target_file_path: String,
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
            target_file_path: root_file_path,
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
    /// if succeed then file is indexed so we can use it later with the path
    /// key
    fn open_file_with_panic(&mut self, path: &str) -> &mut Self {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|_| panic!("Unable to update file , {}", path));
        self.files
            .insert(path.to_string(), (FileOperation::Update, file));
        self
    }

    /// Standard open file to write and happen code ot it
    fn open_file(path: &str) -> std::io::Result<File> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(path)
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
                self.log_error(
                    format!(
                        "error {:?} when creating file at {}
",
                        e, path
                    )
                    .as_str(),
                );
            }
        }
        self
    }
    /// Create a new file and index it in the state or open a new file to
    fn create_or_update_file(&mut self, path: String) -> &mut Self {
        if let Ok(f) = ModulesWriter::open_file(path.as_str()) {
            self.files.insert(path.clone(), (FileOperation::Update, f));
            self.log_ok(format!("found file to update at {} ", path).as_str());
        } else {
            self.create_file(path);
        }
        self
    }

    pub fn add_or_update_imports(&mut self) -> &mut Self {
        let parent = self.content.parent_module().clone();
        let path = parent.meta().filepath().to_string();

        match parent.parent_type {
            ParentModuleType::TargetFile => {
                self.open_file_with_panic(self.target_file_path.clone().as_str());
            }

            ParentModuleType::Folder => {
                self.create_folder(parent.folder_path())
                    .create_or_update_file(path.clone());
            }
        }
        let (op, file) = self.files.get_mut(&path).unwrap();
        let mut imports = parent.imports_to_write();
        match op {
            FileOperation::Update => {
                let mut src = String::new();
                let read = file.read_to_string(&mut src);
                if read.is_err() {
                    self.log_error(format!("Should read file for  {}", path).as_str());
                    self.log_error(format!("{:?}", read.unwrap_err()).as_str());
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
    /// Will check if content already exist and create or merge
    pub fn add_or_update_content(&mut self) -> &mut Self {
        let map = self.content.modules().clone();
        let iter = map.iter();
        for (_, module) in iter {
            let path = module.meta().filepath().to_string();
            self.create_or_update_file(String::from(&path));
            let (op, file) = self.files.get_mut(&path).unwrap();
            match op {
                FileOperation::Update => {
                    let mut src = String::new();

                    let read = file.read_to_string(&mut src);

                    if read.is_err() {
                        self.log_error(format!("Should read file for  {}", &path).as_str());
                        self.log_error(format!("{:?}", read.unwrap_err()).as_str());
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
                    self.log_info(
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
            self.log_info("file already has init");
        } else {
            self.write_on_file_with_custom_message(&path, module.init(), "adding pub fn init()");
        }

        if check.model_exist() {
            self.log_info("file already has Model");
        } else {
            self.write_on_file_with_custom_message(
                &path,
                module.model(),
                "adding pub struct Model{}",
            );
        }
        if check.routes_exist() {
            self.log_info("file already has Routes");
        } else {
            self.write_on_file_with_custom_message(
                &path,
                module.routes(),
                "adding pub enum Routes{} ",
            );
        }
        if check.message_exist() {
            self.log_info("file already has Msg");
        } else {
            self.write_on_file_with_custom_message(
                &path,
                module.routes(),
                "adding pub enum Msg{} ",
            );
        }
        if check.update_exist() {
            self.log_info("file already has update");
        } else {
            self.write_on_file_with_custom_message(
                &path,
                module.update(),
                "adding pub fn update() ",
            );
        }
        if check.view_exist() {
            self.log_info("file already has view");
        } else {
            self.write_on_file_with_custom_message(&path, module.view(), "adding pub fn view() ");
        }
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
