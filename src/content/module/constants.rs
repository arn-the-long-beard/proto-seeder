//! Contains string for unit test and Template for writting content

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
// ------ ------
//     Init
// ------ ------
pub const _INIT_COMMENT: &str = r###"
// ------ ------
//     Init
// ------ ------

"###;

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

// ------ ------
//     Model
// ------ ------

pub const _MODEL_COMMENT: &str = r###"
// ------ ------
//     Model
// ------ ------

"###;

pub const _MODEL_TEMPLATE: &str = r###"
 pub struct Model {}
"###;

// ------ ------
//     Message
// ------ ------

pub const _MESSAGE_COMMENT: &str = r###"
// ------ ------
//     Message
// ------ ------

"###;

pub const _MESSAGE_TEMPLATE: &str = r###"
 pub enum Msg {}
"###;

// ------ ------
//     Update
// ------ ------

pub const _UPDATE_COMMENT: &str = r###"
// ------ ------
//     UPDATE
// ------ ------

"###;

pub const _UPDATE_TEMPLATE: &str = r###"
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg,>,)
{

}
"###;

// ------ ------
//     View
// ------ ------

pub const _VIEW_COMMENT: &str = r###"
// ------ ------
//     VIEW
// ------ ------

"###;

pub const _VIEW_TEMPLATE: &str = r###"
 pub fn view(model_: &Model) -> Node<Msg> {
 div![""]
 }
"###;

pub const _VIEW_TEMPLATE_WITH_ROUTES: &str = r###"
 pub fn view(model_: &Model, nested: &Routes) -> Node<Msg> {
 div![""]
 }
"###;

// ------ ------
//     Message
// ------ ------

pub const _ROUTES_COMMENT: &str = r###"
// ------ ------
//     Routes
// ------ ------

"###;

pub const _ROUTES_TEMPLATE: &str = r###"
#[derive(Debug, PartialEq, Clone, RoutingModules)]
 pub enum Routes {

 }
"###;
