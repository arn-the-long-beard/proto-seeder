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
// /// Will write a function with empty body but write args
// pub fn write_guards(
//     guards: Iter<String, (String, Vec<SeedRoute>)>,
//     src: &mut str,
//     mut pb: &ProgressBar,
// ) -> u32 {
//     let mut updates_number = 0;
//
//     for (guard_name, (guard_content, _)) in guards {
//         let check = Checker::check_local_function_exist(guard_name, src);
//
//         if check {
//             pb.println(format!(
//                 "! no need to create guard [ => ] as fn {} ()",
//                 guard_name,
//             ));
//         } else {
//             let res = file.write_all(format!("{}\n",
// guard_content.to_string()).as_ref());
//
//             if let Err(e) = res {
//                 eprintln!(
//                     "Could not update file for guard {} because {}",
//                     guard_name, e
//                 );
//             } else {
//                 pb.println(format!(
//                     "[+] created guard {} [ => ] as fn {} ()",
//                     guard_name, guard_name
//                 ));
//                 updates_number += 1;
//                 write_space(file);
//             }
//         }
//     }
//     updates_number
// }
