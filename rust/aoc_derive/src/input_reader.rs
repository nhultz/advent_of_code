use crate::utils;
use aoc_types::{DayPart, Part, ReturnWrapperType};

use proc_macro as pm;
use proc_macro2::Span;
use quote::quote;
use std::convert::TryInto;
use syn::{parse_macro_input, Ident, ItemFn, Lifetime, ReturnType, Type, TypeReference};

pub(crate) fn input_reader_impl(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let (day, part) = utils::extract_attrs(args);

    let day = day
        .to_string()
        .parse()
        .expect("input fn must have a defined day");

    let part = part.and_then(|p| p.to_string().parse().ok());

    let input = parse_macro_input!(input as ItemFn);
    let original_fn = input.clone();

    let input_fn_name = &input.sig.ident;

    let out_type = if let ReturnType::Type(_, p) = input.sig.output {
        p
    } else {
        panic!("cannot find output type for {}", input_fn_name)
    };

    let (ret_wrapper_type, out_type) =
        if let Some((ty, inner)) = utils::extract_return_type(&*out_type) {
            (Some(ty), Box::new(inner))
        } else {
            (None, out_type)
        };

    let out_type = match &*out_type {
        Type::Reference(ty) => {
            let ty = ty.clone();
            Box::new(Type::Reference(TypeReference {
                lifetime: Some(Lifetime::new("'a", Span::call_site())),
                ..ty
            }))
        }
        _ => out_type,
    };

    let mod_name = Ident::new(
        &format!("__{}_aoc_input_reader", input_fn_name),
        Span::call_site(),
    );

    let input_reader_body = match ret_wrapper_type {
        Some(ReturnWrapperType::Result) => {
            quote! { #input_fn_name(input).map_err(|err| err.into()) }
        }
        Some(ReturnWrapperType::Option) => {
            quote! { #input_fn_name(input).ok_or_else(|| aoc_types::InputReaderFailed.into()) }
        }
        None => {
            quote! { Ok(#input_fn_name(input)) }
        }
    };

    let reader_impl = |part: Part| {
        let reader_struct = utils::to_camelcase(DayPart { day, part }, "InputReader");

        quote! {
            impl<'a> aoc_types::InputReader<'a> for super::super::__aoc::#reader_struct {
                type Output = #out_type;

                fn read(&self, input: &'a str) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                    #input_reader_body
                }
            }
        }
    };

    let impls = match part {
        Some(part) => reader_impl(part),
        None => (1..=2)
            .into_iter()
            .map(|part| reader_impl(part.try_into().unwrap()))
            .collect(),
    };

    (quote! {
        #original_fn

        #[doc(hidden)]
        mod #mod_name {
            use super::*;

            #impls
        }
    })
    .into()
}
