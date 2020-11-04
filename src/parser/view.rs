use proc_macro_error::{abort, Diagnostic, Level};

use crate::parser::get_string_from_attribute;
use syn::{Attribute, Ident};

/// Get the content of #[view = model_prop => view_function]
pub fn get_view_attribute(
    _ident: Ident,
    attrs: std::slice::Iter<'_, Attribute>,
) -> Option<(String, String)> {
    let mut attrs = attrs.filter_map(|attr| match get_string_from_attribute("view", attr) {
        Ok(op) => op,
        Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
    });
    let view_scope = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple state path defined.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        "".to_string()
    };
    if view_scope.is_empty() {
        None
    } else {
        let string_to_parse = view_scope;
        let view_scope_string: Vec<&str> = string_to_parse.split("=>").collect();
        let mut view_scope_string_iter = view_scope_string.iter();
        let view_path = view_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[view_path = MODEL_PROP => VIEW] but got this {:?}",
                string_to_parse
            )
        });
        let view_init = view_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect view for  #[view_path = MODEL_PROP => VIEW] but got this {:?}",
                string_to_parse
            )
        });
        Some((view_path.trim().to_string(), view_init.trim().to_string()))
    }
}
pub fn get_guard_attribute(
    _ident: Ident,
    attrs: std::slice::Iter<'_, Attribute>,
) -> Option<(String, String, String)> {
    let mut attrs = attrs.filter_map(|attr| match get_string_from_attribute("guard", attr) {
        Ok(op) => op,
        Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
    });
    let guard_scope = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple guard defined are not supported.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        "".to_string()
    };
    if guard_scope.is_empty() {
        None
    } else {
        let string_to_parse = guard_scope;
        let guard_scope_string: Vec<&str> = string_to_parse.split("=>").collect();
        let mut guard_scope_string_iter = guard_scope_string.iter();
        let guard_path = guard_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[guard_path = PATH => GUARD_FUNCTION => REDIRECT_VIEW] but got \
                 this {:?}",
                string_to_parse
            )
        });
        let guard_function = guard_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[guard_path = PATH => GUARD_FUNCTION => REDIRECT_VIEW] but got \
                 this {:?}",
                string_to_parse
            )
        });
        let guard_redirect = guard_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[guard_path = PATH => GUARD_FUNCTION => REDIRECT_VIEW] but got \
                 this {:?}",
                string_to_parse
            )
        });
        Some((
            guard_path.trim().to_string(),
            guard_function.trim().to_string(),
            guard_redirect.trim().to_string(),
        ))
    }
}
