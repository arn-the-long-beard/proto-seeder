use crate::seed_content::SeedRoute;
use indexmap::map::Iter;
use indicatif::ProgressBar;
use std::{fs::File, io::Write};

pub fn write_local_views(
    local_views: Iter<String, (String, SeedRoute)>,
    mut file: File,
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
    }
}
