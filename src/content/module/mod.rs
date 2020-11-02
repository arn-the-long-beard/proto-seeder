use crate::{content::SeedRoute, parser::view::get_view_attribute};
use indexmap::map::IndexMap;
mod constants;

use crate::content::module::{
    constants::{
        _INIT_COMMENT, _MESSAGE_COMMENT, _MESSAGE_TEMPLATE, _MODEL_COMMENT, _MODEL_TEMPLATE,
    },
    init::{
        get_init_for_init_struct_variant, get_init_for_tuple_variant, get_init_for_unit_variant,
    },
};
use convert_case::{Case, Casing};
use syn::{Fields, ItemEnum, Variant};

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

            module
                .set_init(format!("{} {}", _INIT_COMMENT, init))
                .set_origin_route(Some(route.clone()))
                .set_model(format!("{} {}", _MODEL_COMMENT, _MODEL_TEMPLATE))
                .set_msg(format!("{} {}", _MESSAGE_COMMENT, _MESSAGE_TEMPLATE));

            map.insert(v.ident.clone().to_string().to_case(Case::Snake), module);
        }
    }
    map
}
