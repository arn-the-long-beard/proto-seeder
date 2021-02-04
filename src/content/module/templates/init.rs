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
