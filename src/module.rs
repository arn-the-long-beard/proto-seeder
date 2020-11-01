use proc_macro_error::{abort, Diagnostic, Level};

use crate::get_string_from_attribute;

use syn::Attribute;

pub fn modules_path(attrs: &std::slice::Iter<'_, Attribute>) -> Option<String> {
    let mut attrs =
        attrs.clone().filter_map(
            |attr| match get_string_from_attribute("modules_path", attr) {
                Ok(op) => op,
                Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
            },
        );
    let name = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple path names defined.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        "".to_string()
    };
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}
