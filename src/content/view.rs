use crate::{
    content::{get_scoped_field, SeedRoute},
    parser::view::get_view_attribute,
};
use indexmap::map::IndexMap;
use syn::{export::ToTokens, ItemEnum, ItemStruct};

#[derive(PartialEq, Debug, Clone)]
pub struct SeedView {
    /// The name of the view
    pub(crate) name: String,
    /// Its content with function and body
    pub(crate) content: String,
    /// The route that will load it
    pub(crate) route: SeedRoute,
}

pub fn get_local_views(routes_enum: &ItemEnum, model: ItemStruct) -> IndexMap<String, SeedView> {
    let mut map: IndexMap<String, SeedView> = IndexMap::new();

    for v in routes_enum.variants.iter() {
        if let Some((model_scope, view)) = get_view_attribute(v.ident.clone(), v.attrs.iter()) {
            let function_content = get_view_function(model_scope.as_str(), view.as_str(), &model);
            map.insert(
                view,
                SeedView {
                    name: v.ident.clone().to_string(),
                    content: function_content.clone(),
                    route: SeedRoute {
                        name: v.ident.clone().to_string(),
                        content_to_load: function_content.clone(),
                        nested: false,
                        children: false,
                        id_param: false,
                        query: false,
                    },
                },
            );
        }
    }
    map
}

/// todo add Model extractor to match the scope
pub fn get_view_function(model_scope: &str, view: &str, model: &ItemStruct) -> String {
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

#[cfg(test)]
mod test {
    use crate::{
        constants::_FILE_WITH_ROUTES_AND_MODEL,
        content::{view::get_view_function, SeedContent},
        find_model, find_routes,
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
}
