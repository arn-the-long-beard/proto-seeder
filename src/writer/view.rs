// use crate::{
//     content::SeedRoute,
//     writer::{module::checker::Checker, write_space},
// };
// use indexmap::map::Iter;
// use indicatif::ProgressBar;
// use std::{
//     fs::File,
//     io::{Read, Write},
// };
//
// /// Will write a function with dummy div inside
// pub fn write_local_views(
//     local_views: Iter<String, (String, SeedRoute)>,
//     src: &mut str,
//     mut pb: &ProgressBar,
// ) -> u32 {
//     let mut updates_number = 0;
//     for (view_name, (view_content, seed_route)) in local_views {
//         let check = Checker::check_local_function_exist(view_name, src);
//         if check {
//             pb.println(format!(
//                 "! no need to create view for route {} [ => ] as fn {} ()",
//                 seed_route.name, view_name,
//             ));
//         } else {
//             let res = file.write_all(format!("{}\n",
// view_content.to_string()).as_ref());
//
//             if let Err(e) = res {
//                 eprintln!(
//                     "Could not update file for route {} because {}",
//                     seed_route.name, e
//                 );
//             } else {
//                 pb.println(format!(
//                     "[+] created view for route {} [ => ] as fn {} ()",
//                     seed_route.name, view_name,
//                 ));
//             }
//             write_space(file);
//             updates_number += 1;
//         }
//     }
//     updates_number
// }
