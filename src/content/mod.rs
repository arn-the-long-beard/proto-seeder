use crate::{
    content::{
        guard::get_guards,
        module::{get_modules, import::ImportModule, ModuleMeta, SeedModule},
        view::{get_local_views, SeedView},
    },
    parser::{module::modules_path, *},
};
use indexmap::map::IndexMap;
use syn::{Field, ItemEnum, ItemStruct};

pub mod guard;
pub mod module;
pub mod view;

/// Routing Seed concept extracted from the parser
/// This struct conains all thew views, guard, modules ( init ,views, update,
/// Mdg, Model, Routes) for TEA
#[derive(Debug)]
pub struct SeedContent {
    ///Views on the target file from the cli generated by the routing
    local_views: IndexMap<String, SeedView>,
    /// Guards on the target file
    guards: IndexMap<String, (String, Vec<SeedRoute>)>,
    /// Directory specify by `#[modules_path = "pages"]`
    /// It is considered as a module with imports
    /// If not directory, then it will be target file
    parent_module: ImportModule,
    /// TEA files with full content
    modules: IndexMap<String, SeedModule>,
}

impl SeedContent {
    pub fn local_views(&self) -> &IndexMap<String, SeedView> {
        &self.local_views
    }

    pub fn guards(&self) -> &IndexMap<String, (String, Vec<SeedRoute>)> {
        &self.guards
    }

    pub fn parent_module(&self) -> &ImportModule {
        &self.parent_module
    }

    pub fn modules(&self) -> &IndexMap<String, SeedModule> {
        &self.modules
    }
}

impl SeedContent {
    pub fn new(
        routes_enum: ItemEnum,
        model: ItemStruct,
        root_path_file: &str,
        target_file_path: &str,
    ) -> Self {
        let parent_module = modules_path(&routes_enum.attrs.iter());

        let (modules, import_module) = get_modules(
            routes_enum.clone(),
            parent_module,
            root_path_file,
            target_file_path,
        );
        SeedContent {
            local_views: get_local_views(&routes_enum, model.clone()),
            guards: get_guards(&routes_enum, model),
            parent_module: import_module,
            modules,
        }
    }
}

fn get_scoped_field(scope: String, field: &&Field) -> bool {
    if let Some(ident) = &field.ident {
        *ident == scope
    } else {
        false
    }
}
/// todo maybe put field if guarded or not ?
#[derive(Debug, PartialEq, Clone)]
pub struct SeedRoute {
    pub name: String,
    pub nested: bool,
    pub children: bool,
    pub id_param: bool,
    pub query: bool,
    pub content_to_load: String,
}

#[cfg(test)]
mod test {
    use crate::{
        constants::{_FILE_WITH_ROUTES_AND_MODEL, _FILE_WITH_ROUTES_AND_MODEL_WITHOUT_PAGES},
        content::{view::SeedView, SeedContent, SeedRoute},
        find_model, find_routes,
    };
    use indexmap::map::IndexMap;

    const FORBIDDEN_VIEW: &str =
        r###"fn forbidden(logged_user : &Option<LoggedData>) -> Node<Msg>{div!["forbidden"]}"###;

    const NOT_FOUND: &str = r###"fn not_found(model : &Model) -> Node<Msg>{div!["not_found"]}"###;
    const HOME: &str = r###"fn home(theme : &Theme) -> Node<Msg>{div!["home"]}"###;

    #[test]
    fn test_get_local_views() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL).unwrap();
        let model = find_model(&parsed_file);
        let routes_enum = find_routes(&parsed_file);

        let content = SeedContent::new(routes_enum.unwrap(), model.unwrap(), "", "");

        let mut should_have: IndexMap<String, SeedView> = IndexMap::new();

        should_have.insert(
            "forbidden".to_string(),
            SeedView {
                name: "Forbidden".to_string(),
                content: FORBIDDEN_VIEW.to_string(),
                route: SeedRoute {
                    name: "Forbidden".to_string(),
                    nested: false,
                    children: false,
                    id_param: false,
                    query: false,
                    content_to_load: FORBIDDEN_VIEW.to_string(),
                },
            },
        );
        should_have.insert(
            "not_found".to_string(),
            SeedView {
                name: "NotFound".to_string(),
                content: NOT_FOUND.to_string(),
                route: SeedRoute {
                    name: "NotFound".to_string(),
                    nested: false,
                    children: false,
                    id_param: false,
                    query: false,
                    content_to_load: NOT_FOUND.to_string(),
                },
            },
        );

        should_have.insert(
            "home".to_string(),
            SeedView {
                name: "Home".to_string(),
                content: HOME.to_string(),
                route: SeedRoute {
                    name: "Home".to_string(),
                    nested: false,
                    children: false,
                    id_param: false,
                    query: false,
                    content_to_load: HOME.to_string(),
                },
            },
        );

        assert_eq!(
            content.local_views.get("home").unwrap(),
            should_have.get("home").unwrap()
        );

        assert_eq!(
            content.local_views.get("forbidden").unwrap(),
            should_have.get("forbidden").unwrap()
        );

        assert_eq!(
            content.local_views.get("not_found").unwrap(),
            should_have.get("not_found").unwrap()
        );
    }

    #[test]
    fn get_pages() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL).unwrap();
        let model = find_model(&parsed_file);
        let routes_enum = find_routes(&parsed_file);

        let content = SeedContent::new(
            routes_enum.unwrap(),
            model.unwrap(),
            "./my_app/src",
            "./my_app/src/lib.rs",
        );

        let directory_module = content.parent_module;

        assert_eq!(
            directory_module.meta().filepath(),
            "./my_app/src/pages/mod.rs"
        );
        assert_eq!(
            directory_module.meta().import_file_location(),
            "./my_app/src/lib.rs"
        );
        assert_eq!(directory_module.meta().mod_import(), "mod pages;");
    }

    #[test]
    fn test_modules_with_parent() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL).unwrap();
        let model = find_model(&parsed_file);
        let routes_enum = find_routes(&parsed_file);
        let content = SeedContent::new(
            routes_enum.unwrap(),
            model.unwrap(),
            "./src/my_app",
            "./src/my_app/lib.rs",
        );

        let dir = content.parent_module;

        let login = dir
            .imports_content
            .iter()
            .clone()
            .find(|import| import.to_string().eq("pub mod login;"));
        let dashboard = dir
            .imports_content
            .iter()
            .clone()
            .find(|import| import.to_string().eq("pub mod dashboard;"));
        let admin = dir
            .imports_content
            .iter()
            .clone()
            .find(|import| import.to_string().eq("pub mod admin;"));

        assert_eq!(login.is_some(), true);
        assert_eq!(dashboard.is_some(), true);
        assert_eq!(admin.is_some(), true);

        assert_eq!(dir.meta().filepath(), "./src/my_app/pages/mod.rs");
        assert_eq!(dir.meta().import_file_location(), "./src/my_app/lib.rs");
        assert_eq!(dir.meta().mod_import(), "mod pages;");
    }

    #[test]
    fn test_modules_with_target_file() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL_WITHOUT_PAGES).unwrap();
        let model = find_model(&parsed_file);
        let routes_enum = find_routes(&parsed_file);
        let content = SeedContent::new(
            routes_enum.unwrap(),
            model.unwrap(),
            "./src/my_app",
            "./src/my_app/lib.rs",
        );

        let dir = content.parent_module;

        let login = dir
            .imports_content
            .iter()
            .clone()
            .find(|import| import.to_string().eq("mod login;"));
        let dashboard = dir
            .imports_content
            .iter()
            .clone()
            .find(|import| import.to_string().eq("mod dashboard;"));
        let admin = dir
            .imports_content
            .iter()
            .clone()
            .find(|import| import.to_string().eq("mod admin;"));

        assert_eq!(login.is_some(), true);
        assert_eq!(dashboard.is_some(), true);
        assert_eq!(admin.is_some(), true);

        assert_eq!(dir.meta().filepath(), "./src/my_app/lib.rs");
        assert_eq!(dir.meta().import_file_location(), "");
        assert_eq!(dir.meta().mod_import(), "");
    }
}
