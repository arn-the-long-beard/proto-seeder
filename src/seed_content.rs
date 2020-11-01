use crate::view::variant_view_path_tuple;
use indexmap::map::IndexMap;
use syn::{export::ToTokens, Field, ItemEnum, ItemStruct};

/// Routing Seed concept extracted from the parser
#[derive(Debug)]
pub struct SeedContent {
    local_views: IndexMap<String, (String, SeedRoute)>,
    guards: IndexMap<String, (String, SeedRoute)>,
    directory: Option<String>,
    modules: IndexMap<String, (SeedModule, SeedRoute)>,
}

impl SeedContent {
    pub fn new(routes_enum: ItemEnum, model: ItemStruct) -> Self {
        SeedContent {
            local_views: get_local_view(routes_enum, model),
            guards: IndexMap::new(),
            directory: None,
            modules: IndexMap::new(),
        }
    }
}

fn get_local_view(
    routes_enum: ItemEnum,
    model: ItemStruct,
) -> IndexMap<String, (String, SeedRoute)> {
    let mut map: IndexMap<String, (String, SeedRoute)> = IndexMap::new();

    for v in routes_enum.variants.iter() {
        if let Some((model_scope, view)) = variant_view_path_tuple(v.ident.clone(), v.attrs.iter())
        {
            let function_content = get_view_function(model_scope.as_str(), view.as_str(), &model);
            map.insert(
                view,
                (
                    function_content.clone(),
                    SeedRoute {
                        name: v.ident.clone().to_string(),
                        content_to_load: function_content,
                        nested: false,
                        children: false,
                        id_param: false,
                        query: false,
                    },
                ),
            );
        }
    }
    map
}

/// todo add Model extractor to match the scope
fn get_view_function(model_scope: &str, view: &str, model: &ItemStruct) -> String {
    if model_scope.is_empty() {
        format!(
            "fn {}(model : &Model) -> Node<Msg>{{div![\"{}\"]}}",
            view, view
        )
    } else {
        let scope = model
            .fields
            .iter()
            .find(|field| get_scoped_field(model_scope.to_string(), field));
        // fix it with Model

        if let Some(s) = scope {
            let scope_type = &mut s.ty.to_token_stream().to_string();
            scope_type.retain(|c| !c.is_whitespace());

            let ident = &s.ident.as_ref().expect("Should have get property name");
            format!(
                "fn {}({} : &{}) -> Node<Msg>{{div![\"{}\"]}}",
                view,
                ident.to_string(),
                scope_type,
                view
            )
        } else {
            println!(
                "scope {} not found on Model {} so we inject Model instead",
                model_scope, model.ident
            );
            format!(
                "fn {}(model : &Model) -> Node<Msg>{{div![\"{}\"]}}",
                view, view
            )
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

fn get_guards(item_enum: ItemEnum) -> IndexMap<String, String> {
    IndexMap::new()
}

fn get_modules(item_enum: ItemEnum) -> IndexMap<String, (SeedModule, SeedRoute)> {
    IndexMap::new()
}
/// Seed module that represent sometimes page or a global module
/// TODO : in the future, should contain
/// - init
/// - update
/// - view
#[derive(Debug)]
struct SeedModule {
    view: String,
    init: String,
}
#[derive(Debug, PartialEq)]
struct SeedRoute {
    name: String,
    nested: bool,
    children: bool,
    id_param: bool,
    query: bool,
    content_to_load: String,
}

#[cfg(test)]
mod test {
    use crate::{
        constants::_FILE_WITH_ROUTES_AND_MODEL,
        find_model, find_routes,
        seed_content::{get_view_function, SeedContent, SeedRoute},
    };
    use indexmap::map::IndexMap;

    const FORBIDDEN_VIEW: &str =
        r###"fn forbidden(logged_user : &Option<LoggedData>) -> Node<Msg>{div!["forbidden"]}"###;

    const NOT_FOUND: &str = r###"fn not_found(model : &Model) -> Node<Msg>{div!["not_found"]}"###;
    const HOME: &str = r###"fn home(theme : &Theme) -> Node<Msg>{div!["home"]}"###;
    #[test]
    fn test_get_view_function_when_scope_is_good() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL).unwrap();
        let model = find_model(&parsed_file);

        let result = get_view_function("logged_user", "forbidden", &model.unwrap());

        let should_have = FORBIDDEN_VIEW;

        assert_eq!(result, should_have)
    }

    #[test]
    fn test_get_view_function_when_scope_is_wrong() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL).unwrap();
        let model = find_model(&parsed_file);

        let result = get_view_function("my_wrong_scope_on_model", "forbidden", &model.unwrap());

        let should_have = r###"fn forbidden(model : &Model) -> Node<Msg>{div!["forbidden"]}"###;

        assert_eq!(result, should_have)
    }

    #[test]
    fn test_get_view_function_when_no_scope() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL).unwrap();
        let model = find_model(&parsed_file);

        let result = get_view_function("", "not_found", &model.unwrap());

        let should_have = NOT_FOUND;

        assert_eq!(result, should_have);
    }

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
}
