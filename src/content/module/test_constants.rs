//! Contains string for unit test and Template for writing content.

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


// -------------- INIT CONST for Tests------------
pub const _SIMPLE_INIT: &str = r###"
 pub fn init(url:Url, previous_state: &mut Model, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for Login");
       Model {}
       }
"###;

pub const _INIT_WITH_NESTED: &str = r###"
 pub fn init(url:Url, previous_state: &mut Model, nested: &Routes, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for Login");
       Model {}
       }
"###;

pub const _INIT_WITH_ID_PARAM: &str = r###"
 pub fn init(url:Url, previous_state: &mut Model, id: &str, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for Login");
       Model {}
       }
"###;

pub const _INIT_WITH_ID_PARAM_AND_QUERY: &str = r###"
 pub fn init(url:Url, previous_state: &mut Model, id: &str, query: &IndexMap<String, String>, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for Login");
       Model {}
       }
"###;

pub const _INIT_WITH_ID_PARAM_AND_QUERY_AND_CHILDREN: &str = r###"
 pub fn init(url:Url, previous_state: &mut Model, id: &str, query: &IndexMap<String, String>, children: &Routes, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for Login");
       Model {}
       }
"###;

pub const _INIT_WITH_ID_PARAM_AND_CHILDREN: &str = r###"
 pub fn init(url:Url, previous_state: &mut Model, id: &str, children: &Routes, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for Login");
       Model {}
       }
"###;

pub const _INIT_WITH_QUERY: &str = r###"
 pub fn init(url:Url, previous_state: &mut Model, query: &IndexMap<String, String>, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for Login");
       Model {}
       }
"###;

pub const _INIT_WITH_QUERY_AND_CHILDREN: &str = r###"
 pub fn init(url:Url, previous_state: &mut Model, query: &IndexMap<String, String>, children: &Routes, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for Login");
       Model {}
       }
"###;

pub const _INIT_WITH_CHILDREN: &str = r###"
 pub fn init(url:Url, previous_state: &mut Model, children: &Routes, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for Login");
       Model {}
       }
"###;






