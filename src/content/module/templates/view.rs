// ------ ------
//     View
// ------ ------

pub const _VIEW_COMMENT: &str = r###"
// ------ ------
//     VIEW
// ------ ------

"###;

pub const _VIEW_TEMPLATE: &str = r###"
 pub fn view(model: &Model) -> Node<Msg> {
 div![""]
 }
"###;

pub const _VIEW_TEMPLATE_WITH_ROUTES: &str = r###"
 pub fn view(nested: &Routes, model: &Model) -> Node<Msg> {
 div![""]
 }
"###;
