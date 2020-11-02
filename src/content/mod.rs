use crate::{
    content::{
        guard::get_guards,
        module::{get_modules, SeedModule},
        view::get_local_views,
    },
    parser::{module::modules_path, *},
};
use indexmap::map::IndexMap;
use syn::{Field, ItemEnum, ItemStruct};

mod guard;
pub mod module;
mod view;

/// Routing Seed concept extracted from the parser
/// This struct conains all thew views, guard, modules ( init ,views, update,
/// Mdg, Model, Routes) for TEA
#[derive(Debug)]
pub struct SeedContent {
    local_views: IndexMap<String, (String, SeedRoute)>,
    guards: IndexMap<String, (String, Vec<SeedRoute>)>,
    directory: Option<String>,
    modules: IndexMap<String, SeedModule>,
}

impl SeedContent {
    pub fn local_views(&self) -> &IndexMap<String, (String, SeedRoute)> {
        &self.local_views
    }
    pub fn guards(&self) -> &IndexMap<String, (String, Vec<SeedRoute>)> {
        &self.guards
    }
    pub fn directory(&self) -> &Option<String> {
        &self.directory
    }
    pub fn modules(&self) -> &IndexMap<String, SeedModule> {
        &self.modules
    }
}

impl SeedContent {
    pub fn new(routes_enum: ItemEnum, model: ItemStruct) -> Self {
        SeedContent {
            local_views: get_local_views(&routes_enum, model.clone()),
            guards: get_guards(&routes_enum, model.clone()),
            directory: modules_path(&routes_enum.attrs.iter()),
            modules: get_modules(routes_enum),
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
        constants::_FILE_WITH_ROUTES_AND_MODEL,
        content::{SeedContent, SeedRoute},
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

        let content = SeedContent::new(routes_enum.unwrap(), model.unwrap());

        let mut should_have: IndexMap<String, (String, SeedRoute)> = IndexMap::new();

        should_have.insert(
            "forbidden".to_string(),
            (
                FORBIDDEN_VIEW.to_string(),
                SeedRoute {
                    name: "Forbidden".to_string(),
                    nested: false,
                    children: false,
                    id_param: false,
                    query: false,
                    content_to_load: FORBIDDEN_VIEW.to_string(),
                },
            ),
        );
        should_have.insert(
            "not_found".to_string(),
            (
                NOT_FOUND.to_string(),
                SeedRoute {
                    name: "NotFound".to_string(),
                    nested: false,
                    children: false,
                    id_param: false,
                    query: false,
                    content_to_load: NOT_FOUND.to_string(),
                },
            ),
        );

        should_have.insert(
            "home".to_string(),
            (
                HOME.to_string(),
                SeedRoute {
                    name: "Home".to_string(),
                    nested: false,
                    children: false,
                    id_param: false,
                    query: false,
                    content_to_load: HOME.to_string(),
                },
            ),
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

        let content = SeedContent::new(routes_enum.unwrap(), model.unwrap());
        assert_eq!(content.directory.unwrap(), "pages");
    }
}
