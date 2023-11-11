use aoc_types::{Day, DayPart, ReturnWrapperType};

use proc_macro as pm;
use syn;

pub(crate) fn extract_attrs(args: pm::TokenStream) -> (syn::Ident, Option<syn::Ident>) {
    let mut idents = args.into_iter().filter_map(|a| {
        if let pm::TokenTree::Ident(_) = a {
            Some(a.into())
        } else {
            None
        }
    });

    let day = idents.next().expect("expected day as first token");
    let day: syn::Ident = syn::parse(day).expect("failed to parse day token into identifier");

    let part = idents.next().and_then(|i| syn::parse(i).ok());

    (day, part)
}

pub(crate) fn extract_return_type(ty: &syn::Type) -> Option<(ReturnWrapperType, syn::Type)> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { segments: s, .. },
        ..
    }) = ty
    {
        if let Some(p) = s.last() {
            if p.ident == "Result" {
                if let syn::PathArguments::AngleBracketed(a) = &p.arguments {
                    if let Some(arg) = a.args.first() {
                        if let syn::GenericArgument::Type(t) = arg {
                            return Some((ReturnWrapperType::Result, t.clone()));
                        }
                    }
                }
            } else if p.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(a) = &p.arguments {
                    if let Some(arg) = a.args.first() {
                        if let syn::GenericArgument::Type(t) = arg {
                            return Some((ReturnWrapperType::Option, t.clone()));
                        }
                    }
                }
            }
        }
    }

    None
}

// pub(crate) fn to_snakecase(dp: DayPart) -> syn::Ident {
//     let DayPart { day, part } = dp;
//     let name = format!("day{}_part{}", day.as_u8(), part.as_u8());
//     syn::Ident::new(&name, pm::Span::call_site().into())
// }

pub(crate) fn to_camelcase(dp: DayPart, suffix: &str) -> syn::Ident {
    let DayPart { day, part } = dp;
    let name = format!("Day{}Part{}{}", day.as_u8(), part.as_u8(), suffix);
    syn::Ident::new(&name, pm::Span::call_site().into())
}

pub(crate) fn to_input(d: Day) -> syn::Ident {
    syn::Ident::new(&format!("input_{}", d), pm::Span::call_site().into())
}
