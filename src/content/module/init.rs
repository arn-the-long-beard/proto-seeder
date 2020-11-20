//! Manage the build for fn init functions and the different arguments with it
//! by reading from the parser
use crate::content::SeedRoute;

use crate::content::module::constants::{
    _NESTED_INIT_TEMPLATE, _PAYLOAD_INIT_TEMPLATE, _SIMPLE_INIT_TEMPLATE,
};
use proc_macro2::Ident;
use syn::{punctuated::Iter, Field};

pub fn get_init_for_unit_variant(ident: Ident,) -> (String, SeedRoute,) {
    let template = _SIMPLE_INIT_TEMPLATE;
    (
        template.replace("TEMPLATE", ident.to_string().as_str(),),
        SeedRoute {
            name: ident.to_string(),
            nested: false,
            children: false,
            id_param: false,
            query: false,
            content_to_load: "".to_string(),
        },
    )
}

pub fn get_init_for_tuple_variant(ident: Ident, _: Iter<'_, Field,>,) -> (String, SeedRoute,) {
    // todo maybe passing type could be great
    // let first_field = fields.clone().next();
    // let mut scope_type = first_field
    //     .expect("Single tuple variant but no field found")
    //     .ty
    //     .to_token_stream()
    //     .to_string();
    //
    // scope_type.retain(|c| !c.is_whitespace());

    let template = _NESTED_INIT_TEMPLATE;
    (
        template.replace("TEMPLATE", ident.to_string().as_str(),),
        SeedRoute {
            name: ident.to_string(),
            nested: true,
            children: false,
            id_param: false,
            query: false,
            content_to_load: "".to_string(),
        },
    )
}

pub fn get_init_for_init_struct_variant(
    ident: Ident,
    fields: Iter<'_, Field,>,
) -> (String, SeedRoute,) {
    let fields_to_extract = fields.clone();

    let query_parameters = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "query",);

    let id_param = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "id",);

    let children = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "children",);

    let payload = inject_variant_payload_in_function_call((id_param, query_parameters, children,),);

    let template = _PAYLOAD_INIT_TEMPLATE;
    (
        template
            .replace("PAYLOAD", payload.as_str(),)
            .replace("TEMPLATE", ident.to_string().as_str(),),
        SeedRoute {
            name: ident.to_string(),
            nested: false,
            children: children.is_some(),
            id_param: id_param.is_some(),
            query: query_parameters.is_some(),
            content_to_load: "".to_string(),
        },
    )
}
/// write the payload from the variant into the init
/// todo maybe could in the future extract the type from the fields instead
pub fn inject_variant_payload_in_function_call(
    structs_tuple: (Option<&Field,>, Option<&Field,>, Option<&Field,>,),
) -> String {
    match structs_tuple {
        (id, query, children,) if id.is_some() && query.is_some() && children.is_some() => {
            "id: &str, query: &IndexMap<String, String>, children: &Routes".to_string()
        },

        (id, query, _,) if id.is_some() && query.is_some() => {
            "id: &str, query: &IndexMap<String, String>".to_string()
        },
        (id, query, children,) if id.is_none() && query.is_some() && children.is_some() => {
            "query: &IndexMap<String, String>, children: &Routes".to_string()
        },
        (id, query, children,) if id.is_some() && children.is_some() && query.is_none() => {
            "id: &str, children: &Routes".to_string()
        },
        (id, query, children,) if id.is_some() && query.is_none() && children.is_none() => {
            "id: &str".to_string()
        },
        (id, query, children,) if query.is_some() && id.is_none() && children.is_none() => {
            "query: &IndexMap<String, String>".to_string()
        },
        (id, query, children,) if query.is_none() && id.is_none() & children.is_some() => {
            "children: &Routes".to_string()
        },

        (id, query, children,) if query.is_none() && id.is_none() & children.is_none() => {
            "".to_string()
        },
        (_, _, _,) => "".to_string(),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        content::{
            module::{constants::*, get_modules, SeedModule},
            SeedRoute,
        },
        find_routes,
    };
    use indexmap::map::IndexMap;
    use syn::ItemEnum;

    fn get_routes(enum_string: &str,) -> ItemEnum {
        let parsed_file = syn::parse_file(enum_string,).unwrap();
        find_routes(&parsed_file,).expect("should have got the route",)
    }

    fn get_result(expected_template: &str,) -> String {
        format!("{} {}", _INIT_COMMENT, expected_template)
    }

    #[test]
    fn write_init() {
        let map: IndexMap<String, SeedModule,> =
            get_modules(get_routes(_SIMPLE_ROUTE,), None, "", "",).0;
        let result: &SeedModule = map.get("login",).unwrap();
        let should_have = _SIMPLE_INIT;
        assert_eq!(result.init(), get_result(should_have));
        assert_eq!(
            result.origin_route().as_ref().unwrap().clone(),
            SeedRoute {
                name: "Login".to_string(),
                nested: false,
                children: false,
                id_param: false,
                query: false,
                content_to_load: "".to_string()
            }
        );
    }

    #[test]
    fn write_init_with_nested() {
        let map: IndexMap<String, SeedModule,> =
            get_modules(get_routes(_ROUTE_WITH_NESTED,), None, "", "",).0;
        let result: &SeedModule = map.get("login",).unwrap();
        let should_have = _INIT_WITH_NESTED;
        assert_eq!(result.init(), get_result(should_have));
        assert_eq!(
            result.origin_route().as_ref().unwrap().clone(),
            SeedRoute {
                name: "Login".to_string(),
                nested: true,
                children: false,
                id_param: false,
                query: false,
                content_to_load: "".to_string()
            }
        );
    }

    #[test]
    fn write_init_with_id_param() {
        let map: IndexMap<String, SeedModule,> =
            get_modules(get_routes(_ROUTE_WITH_ID_PARAM,), None, "", "",).0;
        let result: &SeedModule = map.get("login",).unwrap();
        let should_have = _INIT_WITH_ID_PARAM;
        assert_eq!(result.init(), get_result(should_have));
        assert_eq!(
            result.origin_route().as_ref().unwrap().clone(),
            SeedRoute {
                name: "Login".to_string(),
                nested: false,
                children: false,
                id_param: true,
                query: false,
                content_to_load: "".to_string()
            }
        );
    }
    #[test]
    fn write_init_with_id_param_and_query() {
        let map: IndexMap<String, SeedModule,> =
            get_modules(get_routes(_ROUTE_WITH_ID_PARAM_AND_QUERY,), None, "", "",).0;
        let result: &SeedModule = map.get("login",).unwrap();
        let should_have = _INIT_WITH_ID_PARAM_AND_QUERY;
        assert_eq!(result.init(), get_result(should_have));
        assert_eq!(
            result.origin_route().as_ref().unwrap().clone(),
            SeedRoute {
                name: "Login".to_string(),
                nested: false,
                children: false,
                id_param: true,
                query: true,
                content_to_load: "".to_string()
            }
        );
    }
    #[test]
    fn write_init_with_id_param_and_query_and_children() {
        let map: IndexMap<String, SeedModule,> = get_modules(
            get_routes(_ROUTE_WITH_ID_PARAM_AND_QUERY_AND_CHILDREN,),
            None,
            "",
            "",
        )
        .0;

        let result: &SeedModule = map.get("login",).unwrap();
        let should_have = _INIT_WITH_ID_PARAM_AND_QUERY_AND_CHILDREN;
        assert_eq!(result.init(), get_result(should_have));
        assert_eq!(
            result.origin_route().as_ref().unwrap().clone(),
            SeedRoute {
                name: "Login".to_string(),
                nested: false,
                children: true,
                id_param: true,
                query: true,
                content_to_load: "".to_string()
            }
        );
    }
    #[test]
    fn write_init_with_id_param_and_children() {
        let map: IndexMap<String, SeedModule,> =
            get_modules(get_routes(_ROUTE_WITH_ID_PARAM_AND_CHILDREN,), None, "", "",).0;
        let result: &SeedModule = map.get("login",).unwrap();
        let should_have = _INIT_WITH_ID_PARAM_AND_CHILDREN;
        assert_eq!(result.init(), get_result(should_have));
        assert_eq!(
            result.origin_route().as_ref().unwrap().clone(),
            SeedRoute {
                name: "Login".to_string(),
                nested: false,
                children: true,
                id_param: true,
                query: false,
                content_to_load: "".to_string()
            }
        );
    }
    #[test]
    fn write_init_with_query() {
        let map: IndexMap<String, SeedModule,> =
            get_modules(get_routes(_ROUTE_WITH_QUERY,), None, "", "",).0;
        let result: &SeedModule = map.get("login",).unwrap();
        let should_have = _INIT_WITH_QUERY;
        assert_eq!(result.init(), get_result(should_have));
        assert_eq!(
            result.origin_route().as_ref().unwrap().clone(),
            SeedRoute {
                name: "Login".to_string(),
                nested: false,
                children: false,
                id_param: false,
                query: true,
                content_to_load: "".to_string()
            }
        );
    }
    #[test]
    fn write_init_with_query_and_children() {
        let map: IndexMap<String, SeedModule,> =
            get_modules(get_routes(_ROUTE_WITH_QUERY_AND_CHILDREN,), None, "", "",).0;
        let result: &SeedModule = map.get("login",).unwrap();
        let should_have = _INIT_WITH_QUERY_AND_CHILDREN;
        assert_eq!(result.init(), get_result(should_have));
        assert_eq!(
            result.origin_route().as_ref().unwrap().clone(),
            SeedRoute {
                name: "Login".to_string(),
                nested: false,
                children: true,
                id_param: false,
                query: true,
                content_to_load: "".to_string()
            }
        );
    }

    #[test]
    fn write_init_with_children() {
        let map: IndexMap<String, SeedModule,> =
            get_modules(get_routes(_ROUTE_WITH_CHILDREN,), None, "", "",).0;
        let result: &SeedModule = map.get("login",).unwrap();
        let should_have = _INIT_WITH_CHILDREN;
        assert_eq!(result.init(), get_result(should_have));
        assert_eq!(
            result.origin_route().as_ref().unwrap().clone(),
            SeedRoute {
                name: "Login".to_string(),
                nested: false,
                children: true,
                id_param: false,
                query: false,
                content_to_load: "".to_string()
            }
        );
    }
}
