use crate::{content::SeedRoute, parser::view::get_view_attribute};
use indexmap::map::IndexMap;
mod constants;

use crate::content::module::constants::{
    _NESTED_INIT_TEMPLATE, _PAYLOAD_INIT_TEMPLATE, _SIMPLE_INIT_TEMPLATE,
};
use convert_case::{Case, Casing};
use proc_macro2::Ident;
use syn::{punctuated::Iter, Field, Fields, ItemEnum, Variant};

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
}

impl SeedModule {}

impl SeedModule {
    pub fn new() -> SeedModule {
        SeedModule {
            view: "".to_string(),
            init: "".to_string(),
            update: "".to_string(),
            model: "".to_string(),
            msg: "".to_string(),
            routes: "".to_string(),
            origin_route: None,
        }
    }

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
}
pub fn get_modules(routes_enum: ItemEnum) -> IndexMap<String, SeedModule> {
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
            let mut module = SeedModule::new();

            let (init, route) = match fields {
                Fields::Unit => get_init_for_unit_variant(ident.clone()),
                Fields::Unnamed(fields) => {
                    get_init_for_tuple_variant(ident.clone(), fields.unnamed.iter())
                }
                Fields::Named(fields) => {
                    get_init_for_init_struct_variant(ident.clone(), fields.named.iter())
                }
            };

            module.set_init(init).set_origin_route(Some(route.clone()));

            map.insert(v.ident.clone().to_string().to_case(Case::Snake), module);
        }
    }
    map
}

fn get_init_for_unit_variant(ident: Ident) -> (String, SeedRoute) {
    let template = _SIMPLE_INIT_TEMPLATE;
    (
        template.replace("TEMPLATE", ident.to_string().as_str()),
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

fn get_init_for_tuple_variant(ident: Ident, fields: Iter<'_, Field>) -> (String, SeedRoute) {
    //todo maybe passing type could be great
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
        template.replace("TEMPLATE", ident.to_string().as_str()),
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

fn get_init_for_init_struct_variant(ident: Ident, fields: Iter<'_, Field>) -> (String, SeedRoute) {
    let fields_to_extract = fields.clone();

    let query_parameters = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "query");

    let id_param = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "id");

    let children = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "children");

    let payload = inject_variant_payload_in_function_call((id_param, query_parameters, children));

    let template = _PAYLOAD_INIT_TEMPLATE;
    (
        template
            .replace("PAYLOAD", payload.as_str())
            .replace("TEMPLATE", ident.to_string().as_str()),
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
    structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>),
) -> String {
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            "id: &str, query: &IndexMap<String, String>, children: &Routes".to_string()
        }

        (id, query, _) if id.is_some() && query.is_some() => {
            "id: &str, query: &IndexMap<String, String>".to_string()
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            "query: &IndexMap<String, String>, children: &Routes".to_string()
        }
        (id, query, children) if id.is_some() && children.is_some() && query.is_none() => {
            "id: &str, children: &Routes".to_string()
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            "id: &str".to_string()
        }
        (id, query, children) if query.is_some() && id.is_none() && children.is_none() => {
            "query: &IndexMap<String, String>".to_string()
        }
        (id, query, children) if query.is_none() && id.is_none() & children.is_some() => {
            "children: &Routes".to_string()
        }

        (id, query, children) if query.is_none() && id.is_none() & children.is_none() => {
            "".to_string()
        }
        (_, _, _) => "".to_string(),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        content::{
            module::{constants::*, get_modules, SeedModule},
            view::get_view_function,
            SeedContent, SeedRoute,
        },
        find_model, find_routes,
    };
    use indexmap::map::IndexMap;
    use syn::ItemEnum;

    fn get_routes(enum_string: &str) -> ItemEnum {
        let parsed_file = syn::parse_file(enum_string).unwrap();
        let routes = find_routes(&parsed_file).expect("should have got the route");
        routes
    }

    #[test]
    fn write_init() {
        let map: IndexMap<String, SeedModule> = get_modules(get_routes(_SIMPLE_ROUTE));
        let result: &SeedModule = map.get("login").unwrap();
        let should_have = _SIMPLE_INIT;
        assert_eq!(result.init(), should_have);
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
        let map: IndexMap<String, SeedModule> = get_modules(get_routes(_ROUTE_WITH_NESTED));
        let result: &SeedModule = map.get("login").unwrap();
        let should_have = _INIT_WITH_NESTED;
        assert_eq!(result.init(), should_have);
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
        let map: IndexMap<String, SeedModule> = get_modules(get_routes(_ROUTE_WITH_ID_PARAM));
        let result: &SeedModule = map.get("login").unwrap();
        let should_have = _INIT_WITH_ID_PARAM;
        assert_eq!(result.init(), should_have);
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
        let map: IndexMap<String, SeedModule> =
            get_modules(get_routes(_ROUTE_WITH_ID_PARAM_AND_QUERY));
        let result: &SeedModule = map.get("login").unwrap();
        let should_have = _INIT_WITH_ID_PARAM_AND_QUERY;
        assert_eq!(result.init(), should_have);
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
        let map: IndexMap<String, SeedModule> =
            get_modules(get_routes(_ROUTE_WITH_ID_PARAM_AND_QUERY_AND_CHILDREN));
        let result: &SeedModule = map.get("login").unwrap();
        let should_have = _INIT_WITH_ID_PARAM_AND_QUERY_AND_CHILDREN;
        assert_eq!(result.init(), should_have);
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
        let map: IndexMap<String, SeedModule> =
            get_modules(get_routes(_ROUTE_WITH_ID_PARAM_AND_CHILDREN));
        let result: &SeedModule = map.get("login").unwrap();
        let should_have = _INIT_WITH_ID_PARAM_AND_CHILDREN;
        assert_eq!(result.init(), should_have);
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
        let map: IndexMap<String, SeedModule> = get_modules(get_routes(_ROUTE_WITH_QUERY));
        let result: &SeedModule = map.get("login").unwrap();
        let should_have = _INIT_WITH_QUERY;
        assert_eq!(result.init(), should_have);
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
        let map: IndexMap<String, SeedModule> =
            get_modules(get_routes(_ROUTE_WITH_QUERY_AND_CHILDREN));
        let result: &SeedModule = map.get("login").unwrap();
        let should_have = _INIT_WITH_QUERY_AND_CHILDREN;
        assert_eq!(result.init(), should_have);
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
        let map: IndexMap<String, SeedModule> = get_modules(get_routes(_ROUTE_WITH_CHILDREN));
        let result: &SeedModule = map.get("login").unwrap();
        let should_have = _INIT_WITH_CHILDREN;
        assert_eq!(result.init(), should_have);
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
