use std::{fs::File, io::Write};

pub mod guard;
pub mod module;
pub mod view;

/// Will just write /n for making a space
pub fn write_space(mut file: &File) {
    let res = file.write_all("\n".to_string().as_ref());
    if let Err(e) = res {
        eprintln!("Could not write space  because {:?}", e);
    }
}
