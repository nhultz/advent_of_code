extern crate proc_macro;

use proc_macro as pm;

mod container;
mod input_reader;
mod runner;
mod utils;

#[proc_macro_attribute]
pub fn aoc(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    runner::runner_impl(args, input)
}

#[proc_macro_attribute]
pub fn aoc_input(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    input_reader::input_reader_impl(args, input)
}

#[proc_macro]
pub fn aoc_lib(input: pm::TokenStream) -> pm::TokenStream {
    container::lib_impl(input)
}
