use crate::utils;
use aoc_types::{DayPart, ReturnWrapperType};

use proc_macro as pm;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, FnArg, Ident, ItemFn, Lifetime, ReturnType, Type, TypeReference};

pub fn runner_impl(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let (day, part) = utils::extract_attrs(args);

    let day = day
        .to_string()
        .parse()
        .expect("runner must have a defined day");

    let part = part
        .expect("runner must have a defined part")
        .to_string()
        .parse()
        .expect("runner must have a defined part");

    let input = parse_macro_input!(input as ItemFn);
    let original_fn = input.clone();

    let runner_fn_name = &input.sig.ident;

    let input_type = match input.sig.inputs.first().expect("runner must take an input") {
        FnArg::Typed(pat) => pat.ty.clone(),
        _ => panic!("runner functions can't have a self parameter"),
    };

    let input_type = match &*input_type {
        Type::Reference(ty) => {
            let ty = ty.clone();
            Box::new(Type::Reference(TypeReference {
                lifetime: Some(Lifetime::new("'a", Span::call_site())),
                ..ty
            }))
        }
        _ => input_type,
    };

    let out_type = if let ReturnType::Type(_, p) = input.sig.output {
        p
    } else {
        panic!("cannot find output type for {}", runner_fn_name)
    };

    let (ret_wrapper_type, out_type) =
        if let Some((ty, inner)) = utils::extract_return_type(&*out_type) {
            (Some(ty), Box::new(inner))
        } else {
            (None, out_type)
        };

    let mod_name = Ident::new(
        &format!("__{}_aoc_runner", runner_fn_name),
        Span::call_site(),
    );

    let runner_body = match ret_wrapper_type {
        Some(ReturnWrapperType::Result) => {
            quote! { #runner_fn_name(input).map_err(|err| err.into()) }
        }
        Some(ReturnWrapperType::Option) => {
            quote! { #runner_fn_name(input).ok_or_else(|| aoc_types::InputReaderFailed.into()) }
        }
        None => {
            quote! { Ok(#runner_fn_name(input)) }
        }
    };

    let runner_struct = utils::to_camelcase(DayPart { day, part }, "Runner");

    (quote! {
        #original_fn

        #[doc(hidden)]
        mod #mod_name {
            use super::*;

            impl<'a> aoc_types::Runner<'a, #input_type> for super::super::__aoc::#runner_struct<#input_type> {
                type Output = #out_type;

                fn run(&self, input: #input_type) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                    #runner_body
                }
            }
        }

    }).into()
}
