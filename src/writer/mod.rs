pub mod guard;
pub mod module;
pub mod view;
//
// /// Will just write /n for making a space
// pub fn write_space(mut file: &File) {
//     let res = file.write_all("\n".to_string().as_ref());
//     if let Err(e) = res {
//         eprintln!("Could not write space  because {:?}", e);
//     }
// }
//
// /// Use to write  local views and guard
// pub fn functions_writer(
//     views: &IndexMap<String, (String, SeedRoute)>,
//     guards: &IndexMap<String, (String, Vec<SeedRoute>)>,
//     file: &mut File,
//     pb: &ProgressBar,
// ) {
//     println!("{:?}", views.iter());
//
//     let mut updates_number = 0;
//     let mut src = String::new();
//     let read = file.read_to_string(&mut src);
//     if read.is_err() {
//         // self.writer
//         //     .log_error(format!("Should read file for  {}",
//         // path).as_str()); self.writer
//         //     .log_error(format!("{:?}", read.unwrap_err()).as_str());
//     } else {
//         write_local_views(views.iter(), &mut src, &pb);
//         println!("{:?}", guards.iter());
//         // todo move to writer
//         write_guards(guards.iter(), &mut src, &pb);
//     }
//
//
// }
//
// #[cfg(test)]
// mod test {
//
//     use crate::{
//         constants::_FILE_WITH_ROUTES_AND_MODEL,
//         content::{view::get_view_function, SeedContent},
//         find_model, find_routes,
//     };
//     use indexmap::map::IndexMap;
//     use crate::constants::_FILE_WITH_GUARD;
//     use crate::writer::view::write_local_views;
//
//     const FORBIDDEN_VIEW: &str =
//         r###"fn forbidden(logged_user : &Option<LoggedData>) ->
// Node<Msg>{div!["forbidden"]}"###;
//
//     const NOT_FOUND: &str = r###"fn not_found(model : &Model) ->
// Node<Msg>{div!["not_found"]}"###;     const HOME: &str = r###"fn home(theme :
// &Theme) -> Node<Msg>{div!["home"]}"###;     #[test]
//     fn test_get_view_function_when_scope_is_good() {
//         let parsed_file = syn::parse_file( _FILE_WITH_GUARD).unwrap();
//         let model = find_model(&parsed_file);
//         let routes_enum = find_routes(&parsed_file);
//         let content = SeedContent::new(routes_enum.unwrap(), model.unwrap(),
// "", "");
//
//         write_local_views(content.local_views(), )
//         assert_eq!(result, should_have)
//     }
//
//     #[test]
//     fn test_get_view_function_when_scope_is_wrong() {
//         let parsed_file =
// syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL).unwrap();         let model =
// find_model(&parsed_file);
//
//         let result = get_view_function("my_wrong_scope_on_model",
// "forbidden", &model.unwrap());
//
//         let should_have = r###"fn forbidden(model : &Model) ->
// Node<Msg>{div!["forbidden"]}"###;
//
//         assert_eq!(result, should_have)
//     }
//
//     #[test]
//     fn test_get_view_function_when_no_scope() {
//         let parsed_file =
// syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL).unwrap();         let model =
// find_model(&parsed_file);
//
//         let result = get_view_function("", "not_found", &model.unwrap());
//
//         let should_have = NOT_FOUND;
//
//         assert_eq!(result, should_have);
//     }
// }
