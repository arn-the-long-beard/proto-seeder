use crate::content::SeedRoute;
use indexmap::map::Iter;
use indicatif::ProgressBar;
use std::{fs::File, io::Write};

pub fn write_local_views(
    local_views: Iter<String, (String, SeedRoute)>,
    mut file: &File,
    mut pb: &ProgressBar,
) {
    for (view_name, (view_content, seed_route)) in local_views {
        let res = file.write_all(format!("{}\n", view_content.to_string()).as_ref());

        if let Err(e) = res {
            eprintln!(
                "Could not update file for route {} because {}",
                seed_route.name, e
            );
        } else {
            pb.println(format!(
                "[+] created view for route {} [ => ] as fn {} ()",
                seed_route.name, view_name,
            ));
        }
        write_space(file);
    }
}
pub fn write_guards(
    guards: Iter<String, (String, Vec<SeedRoute>)>,
    mut file: &File,
    mut pb: &ProgressBar,
) {
    for (guard_name, (guard_content, seed_routes)) in guards {
        let res = file.write_all(format!("{}\n", guard_content.to_string()).as_ref());

        if let Err(e) = res {
            eprintln!(
                "Could not update file for guard {} because {}",
                guard_name, e
            );
        } else {
            pb.println(format!(
                "[+] created guard {} [ => ] as fn {} ()",
                guard_name, guard_name
            ));
        }
        write_space(file);
    }
}
pub fn write_space(mut file: &File) {
    let res = file.write_all("\n".to_string().as_ref());
    if let Err(e) = res {
        eprintln!("Could not write space  because {:?}", e);
    }
}
