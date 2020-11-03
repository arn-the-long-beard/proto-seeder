use crate::{content::SeedRoute, writer::write_space};
use indexmap::map::Iter;
use indicatif::ProgressBar;
use std::{fs::File, io::Write};

/// Will write a function with empty body but write args
pub fn write_guards(
    guards: Iter<String, (String, Vec<SeedRoute,>,),>,
    mut file: &File,
    mut pb: &ProgressBar,
) {
    for (guard_name, (guard_content, seed_routes,),) in guards {
        let res = file.write_all(format!("{}\n", guard_content.to_string()).as_ref(),);

        if let Err(e,) = res {
            eprintln!(
                "Could not update file for guard {} because {}",
                guard_name, e
            );
        } else {
            pb.println(format!(
                "[+] created guard {} [ => ] as fn {} ()",
                guard_name, guard_name
            ),);
        }
        write_space(file,);
    }
}
