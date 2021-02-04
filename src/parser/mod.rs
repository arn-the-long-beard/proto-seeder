//! Parser that share similarities with code in [seed_routing](https://github.com/arn-the-long-beard/seed-routing).
//! It will extract variants and attributes from Routes enum.

use syn::{
    Attribute, Error, Item, ItemEnum, ItemFn, ItemMod, ItemStruct, Lit, LitStr, Meta,
    MetaNameValue, Result,
};

pub mod module;
pub mod view;

pub fn get_string_from_attribute(
    attribute_name: &str,
    attr: &Attribute,
) -> Result<Option<LitStr,>,> {
    if !attr.path.is_ident(attribute_name,) {
        return Ok(None,); // not our attribute
    }
    match attr.parse_meta()? {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(name,),
            ..
        },) => Some(Some(name,),),
        _ => None,
    }
    .ok_or_else(|| Error::new_spanned(attr, &format!("expected #[{} = \"...\"]", attribute_name),),)
}

pub fn find_routes(file: &syn::File,) -> Option<ItemEnum,> {
    file.items.iter().find_map(|item| -> Option<ItemEnum,> {
        if let Item::Enum(found_enum,) = item {
            if found_enum.ident == "Routes" {
                return Some(found_enum.clone(),);
            }
        }
        None
    },)
}

pub fn find_model(file: &syn::File,) -> Option<ItemStruct,> {
    file.items.iter().find_map(|item| -> Option<ItemStruct,> {
        if let Item::Struct(model_struct,) = item {
            if model_struct.ident == "Model" {
                return Some(model_struct.clone(),);
            }
        }
        None
    },)
}

pub fn find_function(file: &syn::File, fn_name: &str,) -> Option<ItemFn,> {
    file.items.iter().find_map(|item| -> Option<ItemFn,> {
        if let Item::Fn(found_fn,) = item {
            if found_fn.sig.ident == fn_name {
                return Some(found_fn.clone(),);
            }
        }
        None
    },)
}

pub fn find_message(file: &syn::File,) -> Option<ItemEnum,> {
    file.items.iter().find_map(|item| -> Option<ItemEnum,> {
        if let Item::Enum(found_enum,) = item {
            if found_enum.ident == "Msg" {
                return Some(found_enum.clone(),);
            }
        }
        None
    },)
}

pub fn find_mod(file: &syn::File, name: &str,) -> Option<ItemMod,> {
    file.items
        .clone()
        .iter()
        .find_map(|item| -> Option<ItemMod,> {
            if let Item::Mod(found_mod,) = item {
                if found_mod.ident == name {
                    return Some(found_mod.clone(),);
                }
            }
            None
        },)
}

#[cfg(test)]
mod test {
    use crate::{
        constants::{_FILE_WITHOUT_ROUTES_NOR_MODEL, _FILE_WITH_ROUTES_AND_MODEL},
        parser::{find_model, find_routes},
    };

    #[test]
    fn test_find_routes() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL,).unwrap();
        let route = find_routes(&parsed_file,);

        assert_eq!(route.is_some(), true);

        let parsed_file = syn::parse_file(_FILE_WITHOUT_ROUTES_NOR_MODEL,).unwrap();
        let route = find_routes(&parsed_file,);

        assert_eq!(route.is_some(), false)
    }

    #[test]
    fn test_find_model() {
        let parsed_file = syn::parse_file(_FILE_WITH_ROUTES_AND_MODEL,).unwrap();
        let model = find_model(&parsed_file,);

        assert_eq!(model.is_some(), true);

        let parsed_file = syn::parse_file(_FILE_WITHOUT_ROUTES_NOR_MODEL,).unwrap();
        let model = find_model(&parsed_file,);

        assert_eq!(model.is_some(), false)
    }
}
