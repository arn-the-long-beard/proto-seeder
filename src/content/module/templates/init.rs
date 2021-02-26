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
 pub fn init(url:Url, previous_state: &mut Model, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for TEMPLATE");
       Model {}
       }
"###;

pub const _NESTED_INIT_TEMPLATE: &str = r###"
 pub fn init(url:Url, previous_state: &mut Model, nested: &Routes, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for TEMPLATE");
       Model {}
       }
"###;

pub const _PAYLOAD_INIT_TEMPLATE: &str = r###"
 pub fn init(url:Url, previous_state: &mut Model, PAYLOAD, orders: &mut impl Orders<Msg,> ) -> Model {
       log!("init for TEMPLATE");
       Model {}
       }
"###;
