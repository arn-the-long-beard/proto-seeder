pub const _SIMPLE_ROUTE: &str = r###"
#[derive(Debug, PartialEq, Clone, RoutingModules)]
#[modules_path = "pages"]
pub enum Routes {
    Login
    }
"###;

pub const _ROUTE_WITH_NESTED: &str = r###"
#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    Login(login::Routes)
    }
"###;

pub const _ROUTE_WITH_ID_PARAM: &str = r###"
#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    Login{ id : String }
    }
"###;

pub const _ROUTE_WITH_ID_PARAM_AND_QUERY: &str = r###"
#[derive(Debug, PartialEq, Clone, RoutingModules)]

pub enum Routes {
    Login{ id : String  , query : IndexMap<String, String> }
    }
"###;

pub const _ROUTE_WITH_ID_PARAM_AND_QUERY_AND_CHILDREN: &str = r###"
#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    Login{ id : String  , query : IndexMap<String, String> , children : login::routes }
    }
"###;

pub const _ROUTE_WITH_ID_PARAM_AND_CHILDREN: &str = r###"
#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    Login{ id : String  , children : login::routes }
    }
"###;

pub const _ROUTE_WITH_QUERY: &str = r###"
#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    Login{  query : IndexMap<String, String> }
    }
"###;

pub const _ROUTE_WITH_QUERY_AND_CHILDREN: &str = r###"
#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    Login{  query : IndexMap<String, String> , children : login::routes }
    }
"###;

pub const _ROUTE_WITH_CHILDREN: &str = r###"
#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    Login{  children : login::routes }
    }
"###;

// -------------- INIT CONST ------------

pub const _SIMPLE_INIT_TEMPLATE: &str = r###"
 pub fn init(url:Url, previous_state: Model, orders: &mut impl Orders<Msg,> ) {
       log!("init for TEMPLATE");
       }
"###;

pub const _NESTED_INIT_TEMPLATE: &str = r###"
 pub fn init(url:Url, previous_state: Model, nested: Routes, orders: &mut impl Orders<Msg,> ) {
       log!("init for TEMPLATE");
       }
"###;

pub const _PAYLOAD_INIT_TEMPLATE: &str = r###"
 pub fn init(url:Url, previous_state: Model, PAYLOAD, orders: &mut impl Orders<Msg,> ) {
       log!("init for TEMPLATE");
       }
"###;

// -------------- INIT CONST for Tests------------
pub const _SIMPLE_INIT: &str = r###"
 pub fn init(url:Url, previous_state: Model, orders: &mut impl Orders<Msg,> ) {
       log!("init for Login");
       }
"###;

pub const _INIT_WITH_NESTED: &str = r###"
 pub fn init(url:Url, previous_state: Model, nested: Routes, orders: &mut impl Orders<Msg,> ) {
       log!("init for Login");
       }
"###;

pub const _INIT_WITH_ID_PARAM: &str = r###"
 pub fn init(url:Url, previous_state: Model, id: &str, orders: &mut impl Orders<Msg,> ) {
       log!("init for Login");
       }
"###;

pub const _INIT_WITH_ID_PARAM_AND_QUERY: &str = r###"
 pub fn init(url:Url, previous_state: Model, id: &str, query: &IndexMap<String, String>, orders: &mut impl Orders<Msg,> ) {
       log!("init for Login");
       }
"###;

pub const _INIT_WITH_ID_PARAM_AND_QUERY_AND_CHILDREN: &str = r###"
 pub fn init(url:Url, previous_state: Model, id: &str, query: &IndexMap<String, String>, children: &Routes, orders: &mut impl Orders<Msg,> ) {
       log!("init for Login");
       }
"###;

pub const _INIT_WITH_ID_PARAM_AND_CHILDREN: &str = r###"
 pub fn init(url:Url, previous_state: Model, id: &str, children: &Routes, orders: &mut impl Orders<Msg,> ) {
       log!("init for Login");
       }
"###;

pub const _INIT_WITH_QUERY: &str = r###"
 pub fn init(url:Url, previous_state: Model, query: &IndexMap<String, String>, orders: &mut impl Orders<Msg,> ) {
       log!("init for Login");
       }
"###;

pub const _INIT_WITH_QUERY_AND_CHILDREN: &str = r###"
 pub fn init(url:Url, previous_state: Model, query: &IndexMap<String, String>, children: &Routes, orders: &mut impl Orders<Msg,> ) {
       log!("init for Login");
       }
"###;

pub const _INIT_WITH_CHILDREN: &str = r###"
 pub fn init(url:Url, previous_state: Model, children: &Routes, orders: &mut impl Orders<Msg,> ) {
       log!("init for Login");
       }
"###;
