use crate::{
    content::{get_scoped_field, SeedRoute},
    parser::view::get_guard_attribute,
};
use indexmap::map::IndexMap;
use syn::{export::ToTokens, ItemEnum, ItemStruct};

pub fn get_guards(
    routes_enum: &ItemEnum,
    model: ItemStruct,
) -> IndexMap<String, (String, Vec<SeedRoute>)> {
    let mut map: IndexMap<String, (String, Vec<SeedRoute>)> = IndexMap::new();

    for v in routes_enum.variants.iter() {
        if let Some((model_scope, guard, _)) = get_guard_attribute(v.ident.clone(), v.attrs.iter())
        {
            let function_content = get_guard_function(model_scope.as_str(), guard.as_str(), &model);

            if let Some(g) = map.get_mut(&guard) {
                g.1.push(SeedRoute {
                    name: v.ident.clone().to_string(),
                    content_to_load: function_content,
                    nested: false,
                    children: false,
                    id_param: false,
                    query: false,
                })
            } else {
                map.insert(
                    guard.clone(),
                    (
                        function_content.clone(),
                        vec![SeedRoute {
                            name: v.ident.clone().to_string(),
                            content_to_load: function_content,
                            nested: false,
                            children: false,
                            id_param: false,
                            query: false,
                        }],
                    ),
                );
            }
        }
    }
    map
}

/// todo add Model extractor to match the scope
pub fn get_guard_function(model_scope: &str, guard: &str, model: &ItemStruct) -> String {
    if model_scope.is_empty() {
        format!(
            "fn {}(model : &Model) -> Option<bool> {{log!(\"Write condition\")}}",
            guard,
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
                "fn {}({} : &{}) -> Option<bool> {{log!(\"Write condition\")}}",
                guard,
                ident.to_string(),
                scope_type,
            )
        } else {
            println!(
                "scope {} not found on Model {} so we inject Model instead",
                model_scope, model.ident
            );
            format!(
                "fn {}(model : &Model) -> Option<bool> {{log!(\"Write condition\")}}",
                guard,
            )
        }
    }
}
#[cfg(test)]
mod test {
    use crate::{
        constants::_FILE_WITH_ROUTES_AND_MODEL,
        content::{SeedContent, SeedRoute},
        find_model, find_routes,
    };
    use indexmap::map::IndexMap;

    const GUARD: &str = r###"fn guard(model : &Model) -> Option<bool> {log!("Write condition")}"###;

    const ADMIN_GUARD: &str = r###"fn admin_guard(logged_user : &Option<LoggedData>) -> Option<bool> {log!("Write condition")}"###;

    #[test]
    fn test_get_guards() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL).unwrap();
        let model = find_model(&parsed_file);
        let routes_enum = find_routes(&parsed_file);

        let content = SeedContent::new(routes_enum.unwrap(), model.unwrap(), "", "");

        let mut should_have: IndexMap<String, (String, SeedRoute)> = IndexMap::new();

        let guard = content.guards.get("guard").unwrap();
        assert_eq!(guard.1.len(), 2);

        assert_eq!(
            guard.1[0],
            SeedRoute {
                name: "Settings".to_string(),
                nested: false,
                children: false,
                id_param: false,
                query: false,
                content_to_load: GUARD.to_string()
            }
        );
        assert_eq!(
            guard.1[1],
            SeedRoute {
                name: "Dashboard".to_string(),
                nested: false,
                children: false,
                id_param: false,
                query: false,
                content_to_load: GUARD.to_string()
            }
        );
        let admin_guard = content.guards.get("admin_guard").unwrap();
        assert_eq!(
            admin_guard.1[0],
            SeedRoute {
                name: "Admin".to_string(),
                nested: false,
                children: false,
                id_param: false,
                query: false,
                content_to_load: ADMIN_GUARD.to_string()
            }
        );

        assert_eq!(admin_guard.1.len(), 1);
    }
}
