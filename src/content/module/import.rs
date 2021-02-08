//! Manage module imports to write for Seed content and files to write with and
//! without Folder.

use crate::content::module::ModuleMeta;

const ERROR_WHEN_PATH_EMPTY: &str = "Cannot have empty path for directory";
const ERROR_WHEN_MULTIPLE_FOLDERS_NOT_SUPPORTED: &str =
    "Does not support multiple directory for now";
/// Specific module used only for imports
/// Useful for subdirectories,
/// see examples with `#[modules_path = "pages"]`.
#[derive(Debug, Clone)]
pub struct ImportModule {
    meta: ModuleMeta,
    folder_path: String,
    name: String,
    /// List of imports.
    pub imports_content: Vec<String,>,
    pub imports_names: Vec<String,>,
    pub parent_type: ParentModuleType,
}
#[derive(Debug, Clone, PartialEq)]
pub enum ParentModuleType {
    TargetFile,
    Folder,
}

impl ImportModule {
    pub fn meta(&self,) -> &ModuleMeta {
        &self.meta
    }

    pub fn name(&self,) -> &str {
        &self.name
    }

    pub fn imports_content(&self,) -> &Vec<String,> {
        &self.imports_content
    }

    pub fn folder_path(&self,) -> &str {
        &self.folder_path
    }

    pub fn imports_to_write(&self,) -> String {
        let mut content = "".to_string();
        for imp in &self.imports_content {
            content = format!("{}{}\n", content, imp)
        }
        content
    }

    /// Create a new import module.
    pub fn new_folder_module(
        directory: &str,
        current_path: &str,
        target_file_path: &str,
    ) -> Result<ImportModule, String,> {
        if directory.contains('/',) {
            return Result::Err(ERROR_WHEN_MULTIPLE_FOLDERS_NOT_SUPPORTED.to_string(),);
        }
        if directory.is_empty() {
            return Result::Err(ERROR_WHEN_PATH_EMPTY.to_string(),);
        }
        Ok(ImportModule {
            name: directory.to_string(),
            folder_path: format!("{}/{}", current_path, directory),
            meta: ModuleMeta {
                filepath: format!("{}/{}/mod.rs", current_path, directory),
                mod_import: format!("mod {};", directory),
                import_file_location: target_file_path.to_string(),
            },
            imports_content: vec![],
            imports_names: vec![],
            parent_type: ParentModuleType::Folder,
        },)
    }

    /// Mark target file as module for imports.
    pub fn new_target_file_module(
        current_path: &str,
        target_file_path: &str,
    ) -> Result<ImportModule, String,> {
        Ok(ImportModule {
            // todo need to fix later
            name: "".parse().unwrap(),
            folder_path: current_path.to_string(),
            meta: ModuleMeta {
                filepath: target_file_path.to_string(),
                mod_import: "".to_string(),
                import_file_location: "".to_string(),
            },
            imports_content: vec![],
            imports_names: vec![],
            parent_type: ParentModuleType::TargetFile,
        },)
    }
}
#[cfg(test)]
mod test {
    use crate::content::module::import::{
        ImportModule, ERROR_WHEN_MULTIPLE_FOLDERS_NOT_SUPPORTED, ERROR_WHEN_PATH_EMPTY,
    };

    #[test]
    fn get_pages() {
        let import_result =
            ImportModule::new_folder_module("pages", "./my_app/src", "./my_app/src/lib.rs",);

        let import = import_result.unwrap();

        assert_eq!(import.meta().filepath(), "./my_app/src/pages/mod.rs");
        assert_eq!(import.meta().import_file_location(), "./my_app/src/lib.rs");
        assert_eq!(import.meta().mod_import(), "mod pages;");
        assert_eq!(import.folder_path, "./my_app/src/pages");
    }

    #[test]
    fn get_error_if_empty_path() {
        let import_result =
            ImportModule::new_folder_module("", "./my_app/src", "./my_app/src/lib.rs",);

        assert_eq!(import_result.is_err(), true);
        assert_eq!(import_result.unwrap_err(), ERROR_WHEN_PATH_EMPTY)
    }
    #[test]
    fn get_error_if_multiple_folders() {
        let import_result =
            ImportModule::new_folder_module("pages/sub", "./my_app/src", "./my_app/src/lib.rs",);

        assert_eq!(import_result.is_err(), true);
        assert_eq!(
            import_result.unwrap_err(),
            ERROR_WHEN_MULTIPLE_FOLDERS_NOT_SUPPORTED
        )
    }
}
