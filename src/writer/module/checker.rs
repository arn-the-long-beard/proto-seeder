use crate::{
    content::module::{import::ImportModule, SeedModule},
    parser::{find_function, find_message, find_mod, find_model, find_routes},
};
use std::{fs::File, io::Read};

pub struct Checker {
    source_file: syn::File,
    will_duplicate: Vec<String>,
}

impl Checker {
    pub fn return_mod_if_exist(src: &str, parent_module: ImportModule) -> Vec<String> {
        let mut list: Vec<String> = Vec::new();

        let parsed_file = syn::parse_file(&src).expect("Should read content for file ");

        for imp in parent_module.imports_names.iter() {
            if find_mod(&parsed_file, imp).is_some() {
                list.push(imp.to_string())
            }
        }

        list
    }

    pub fn store_content_for_check(src: &str) -> Checker {
        let parsed_file = syn::parse_file(&src).expect(
            "Should read content
for file ",
        );

        let mut list: Vec<String> = Vec::new();

        if let Some(init) = find_function(&parsed_file, "init") {
            list.push(init.sig.ident.to_string());
        }

        if let Some(m) = find_model(&parsed_file) {
            list.push(m.ident.to_string());
        }

        if let Some(r) = find_routes(&parsed_file) {
            list.push(r.ident.to_string());
        }

        if let Some(msg) = find_message(&parsed_file) {
            list.push(msg.ident.to_string());
        }
        if let Some(update) = find_function(&parsed_file, "update") {
            list.push(update.sig.ident.to_string());
        }

        if let Some(view) = find_function(&parsed_file, "view") {
            list.push(view.sig.ident.to_string());
        }

        Checker {
            source_file: parsed_file,
            will_duplicate: list,
        }
    }

    fn check_duplicate(&self, content_name: &str) -> bool {
        self.will_duplicate
            .iter()
            .clone()
            .any(|c| *c == content_name)
    }

    /// todo all of this should be dynamic in the future
    pub fn init_exist(&self) -> bool {
        self.check_duplicate("init")
    }
    pub fn model_exist(&self) -> bool {
        self.check_duplicate("Model")
    }
    pub fn routes_exist(&self) -> bool {
        self.check_duplicate("Routes")
    }
    pub fn message_exist(&self) -> bool {
        self.check_duplicate("Msg")
    }
    pub fn update_exist(&self) -> bool {
        self.check_duplicate("update")
    }
    pub fn view_exist(&self) -> bool {
        self.check_duplicate("view")
    }
}

#[cfg(test)]
mod test {
    use crate::{constants::*, writer::module::checker::Checker};
    #[test]
    fn test_if_exist_update() {
        let check = Checker::store_content_for_check(_FILE_WITHOUT_UPDATE);
        assert_eq!(check.update_exist(), true);
    }
    #[test]
    fn test_if_exist_model() {
        let check = Checker::store_content_for_check(_FILE_WITH_ROUTES_AND_MODEL);

        assert_eq!(check.init_exist(), true);
        assert_eq!(check.routes_exist(), true);
        assert_eq!(check.model_exist(), true);
        assert_eq!(check.update_exist(), false);
    }
}
