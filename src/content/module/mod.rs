use crate::{content::SeedRoute, parser::view::get_view_attribute};
use indexmap::map::IndexMap;
mod constants;

use crate::content::module::{
    constants::*,
    import::{ImportModule, ParentModuleType},
    init::{
        get_init_for_init_struct_variant, get_init_for_tuple_variant, get_init_for_unit_variant,
    },
};
use convert_case::{Case, Casing};
use std::borrow::BorrowMut;
use syn::{Field, Fields, ItemEnum, Variant};

pub mod import;
mod init;

/// Seed module that represent sometimes page or a global module
/// TODO : in the future, should contain
/// - init
/// - update
/// - view
#[derive(Debug, Clone)]
pub struct SeedModule {
    view: String,
    init: String,
    update: String,
    model: String,
    msg: String,
    routes: String,
    origin_route: Option<SeedRoute>,
    meta: ModuleMeta,
}
#[derive(Debug, Clone)]
pub struct ModuleMeta {
    /// For example could be /pages/login.rs or /login.rs or mod.rs
    filepath: String,
    /// The string that represent the import such as `mod login;`
    /// Ex: If module is login ->  'pub mod login;' if sub directory -> 'mod
    /// login;'
    /// if empty, it means the module is not reexported. This is expected
    /// behavior if we import directly in the target file
    mod_import: String,
    /// Where the import should be written
    /// Ex: If parent directory, ->  'mod.rs' if  not ->  lib.rs
    /// if empty, it means the module is not reexported. This is expected
    /// behavior if we import directly in the target file
    import_file_location: String,
}

impl ModuleMeta {
    pub fn set_filepath(&mut self, filepath: String) {
        self.filepath = filepath;
    }
    pub fn set_mod_import(&mut self, mod_import: String) {
        self.mod_import = mod_import;
    }
    pub fn set_import_file_location(&mut self, import_file_location: String) {
        self.import_file_location = import_file_location;
    }
    pub fn filepath(&self) -> &str {
        &self.filepath
    }
    pub fn mod_import(&self) -> &str {
        &self.mod_import
    }
    pub fn import_file_location(&self) -> &str {
        &self.import_file_location
    }
}

impl SeedModule {
    pub fn view(&self) -> &str {
        &self.view
    }

    pub fn init(&self) -> &str {
        &self.init
    }

    pub fn update(&self) -> &str {
        &self.update
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn routes(&self) -> &str {
        &self.routes
    }

    pub fn origin_route(&self) -> &Option<SeedRoute> {
        &self.origin_route
    }

    pub fn meta(&self) -> &ModuleMeta {
        &self.meta
    }
}

impl Default for SeedModule {
    fn default() -> Self {
        SeedModule {
            view: "".to_string(),
            init: "".to_string(),
            update: "".to_string(),
            model: "".to_string(),
            msg: "".to_string(),
            routes: "".to_string(),
            origin_route: None,
            meta: ModuleMeta {
                filepath: "".to_string(),
                mod_import: "".to_string(),
                import_file_location: "".to_string(),
            },
        }
    }
}

impl SeedModule {
    /// Set the view, replace if any
    pub fn set_view(&mut self, view_function: String) -> &mut SeedModule {
        self.view = view_function;
        self
    }

    //// set the init , replace if any
    pub fn set_init(&mut self, init_function: String) -> &mut SeedModule {
        self.init = init_function;
        self
    }

    //// set the Updat,e replace if any
    pub fn set_update(&mut self, update_function: String) -> &mut SeedModule {
        self.update = update_function;
        self
    }

    pub fn set_model(&mut self, model: String) -> &mut SeedModule {
        self.model = model;
        self
    }

    pub fn set_msg(&mut self, msg: String) -> &mut SeedModule {
        self.msg = msg;
        self
    }

    pub fn set_routes(&mut self, routes: String) -> &mut SeedModule {
        self.routes = routes;
        self
    }

    pub fn set_origin_route(&mut self, origin_route: Option<SeedRoute>) -> &mut SeedModule {
        self.origin_route = origin_route;
        self
    }

    pub fn set_meta(&mut self, meta: ModuleMeta) -> &mut SeedModule {
        self.meta = meta;
        self
    }
}
pub fn get_modules(
    routes_enum: ItemEnum,
    modules_path: Option<String>,
    root_path_file: &str,
    target_file_path: &str,
) -> (IndexMap<String, SeedModule>, ImportModule) {
    let mut parent_module = if modules_path.is_some() {
        ImportModule::new_folder_module(
            modules_path.unwrap().as_str(),
            root_path_file,
            target_file_path,
        )
        .ok()
    } else {
        ImportModule::new_target_file_module(root_path_file, target_file_path).ok()
    };

    let mut map: IndexMap<String, SeedModule> = IndexMap::new();
    for v in routes_enum.variants.iter() {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = v;

        if let Some(_) = get_view_attribute(v.ident.clone(), v.attrs.iter()) {
        } else {
            let mut module = SeedModule::default();

            let name = v.ident.clone().to_string().to_case(Case::Snake);

            // find the parent module and improts
            let (module_meta, updated_parent) = generate_module_meta(
                parent_module.unwrap(),
                name.as_str(),
                root_path_file,
                target_file_path,
            );

            parent_module = Some(updated_parent);
            // prepare init content

            let (init, route) = match fields {
                Fields::Unit => get_init_for_unit_variant(ident.clone()),
                Fields::Unnamed(fields) => {
                    get_init_for_tuple_variant(ident.clone(), fields.unnamed.iter())
                }
                Fields::Named(fields) => {
                    get_init_for_init_struct_variant(ident.clone(), fields.named.iter())
                }
            };
            // prepare view content
            let view = match fields {
                Fields::Unit => _VIEW_TEMPLATE,
                Fields::Unnamed(_) => _VIEW_TEMPLATE_WITH_ROUTES,
                Fields::Named(fields) => {
                    match fields
                        .named
                        .iter()
                        .clone()
                        .find(|f| f.ident.as_ref().unwrap() == "children")
                    {
                        None => _VIEW_TEMPLATE,
                        Some(_) => _VIEW_TEMPLATE_WITH_ROUTES, // todo could extract type there
                    }
                }
            };

            module
                .set_meta(module_meta)
                .set_origin_route(Some(route.clone()))
                .set_init(format!("{} {}", _INIT_COMMENT, init))
                .set_model(format!("{} {}", _MODEL_COMMENT, _MODEL_TEMPLATE))
                .set_msg(format!("{} {}", _MESSAGE_COMMENT, _MESSAGE_TEMPLATE))
                .set_update(format!("{} {}", _UPDATE_COMMENT, _UPDATE_TEMPLATE))
                .set_view(format!("{} {}", _VIEW_COMMENT, view))
                .set_routes(format!("{} {}", _ROUTES_COMMENT, _ROUTES_TEMPLATE));

            map.insert(name, module);
        }
    }
    (map, parent_module.unwrap())
}

fn generate_module_meta(
    parent_module: ImportModule,
    name: &str,
    root_path_file: &str,
    target_file_path: &str,
) -> (ModuleMeta, ImportModule) {
    let mut parent = parent_module.clone();
    match parent_module.parent_type {
        ParentModuleType::TargetFile => {
            let import = format!("mod {};", name.clone());
            parent.imports_content.push(import.clone());
            parent.imports_names.push(name.to_string());
            (
                ModuleMeta {
                    filepath: format!("{}/{}.rs", root_path_file, name),
                    mod_import: import,
                    import_file_location: target_file_path.to_string(),
                },
                parent,
            )
        }
        ParentModuleType::Folder => {
            let import = format!("pub mod {};", name);
            parent.imports_content.push(import.clone());
            parent.imports_names.push(name.to_string());
            (
                ModuleMeta {
                    filepath: format!("{}/{}.rs", parent.folder_path(), name),
                    mod_import: import,
                    import_file_location: parent.meta().filepath.to_string(),
                },
                parent,
            )
        }
    }
}
#[cfg(test)]
mod test {
    use crate::content::module::{
        generate_module_meta,
        import::{ImportModule, ParentModuleType},
    };

    #[test]
    fn test_import_with_parent() {
        let import_result =
            ImportModule::new_folder_module("pages", "./my_app/src", "./my_app/src/lib.rs");

        let mut import_module = import_result.unwrap();

        let (module_meta, updated_parent) = generate_module_meta(
            import_module,
            "login",
            "./my_app/src/",
            "./my_app/src/lib.rs",
        );

        let (module_meta2, updated_parent_2) = generate_module_meta(
            updated_parent.clone(),
            "register",
            "./my_app/src/",
            "./my_app/src/lib.rs",
        );

        assert_eq!(module_meta.filepath, "./my_app/src/pages/login.rs");
        assert_eq!(module_meta.mod_import, "pub mod login;");
        assert_eq!(
            module_meta.import_file_location,
            "./my_app/src/pages/mod.rs"
        );

        let imports = updated_parent.clone().imports_content;
        let login = imports
            .iter()
            .clone()
            .find(|import| import.to_string().eq("pub mod login;"));

        assert_eq!(login.is_some(), true);

        let imports_content = "pub mod login;\npub mod register;\n";

        assert_eq!(updated_parent_2.imports_to_write(), imports_content);
    }

    #[test]
    fn test_import_with_target_file() {
        let target_file_module =
            ImportModule::new_target_file_module("./my_app/src", "./my_app/src/lib.rs");
        let (module_meta, import) = generate_module_meta(
            target_file_module.unwrap(),
            "login",
            "./my_app/src",
            "./my_app/src/lib.rs",
        );

        let import_module = import;

        assert_eq!(import_module.parent_type, ParentModuleType::TargetFile);
        assert_eq!(import_module.folder_path(), "./my_app/src");
        assert_eq!(import_module.meta().filepath, "./my_app/src/lib.rs");
        assert_eq!(import_module.meta().mod_import, "");
        assert_eq!(import_module.meta().import_file_location, "");

        let login = import_module
            .imports_content
            .iter()
            .clone()
            .find(|import| import.to_string().eq("mod login;"));

        eprintln!("{:?}", import_module.imports_content);
        assert_eq!(login.is_some(), true);
        assert_eq!(module_meta.filepath, "./my_app/src/login.rs");
        assert_eq!(module_meta.mod_import, "mod login;");
        assert_eq!(module_meta.import_file_location, "./my_app/src/lib.rs");
    }
}
