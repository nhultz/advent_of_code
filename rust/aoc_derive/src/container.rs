use crate::utils;
use aoc_types::{Day, DayPart};
use proc_macro as pm;
use proc_macro2 as pm2;
use quote::quote;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct LibInfo {
    year: u32,
}

pub fn lib_impl(input: pm::TokenStream) -> pm::TokenStream {
    let info = parse_lib_info(input).expect("failed to parse lib info");

    let year = info.year;

    let inputs = gen_inputs(year);
    let structs = gen_runner_structs();
    let run_match = gen_problem_match();

    let expanded = quote! {
        #[doc(hidden)]
        pub mod __aoc {
            use aoc_types::{Day, InputReader, InputReaderDefault, Runner, RunnerDefault};
            use std::marker::PhantomData;
            use std::time::{Duration, Instant};

            pub const YEAR : u32 = #year;

            #structs

            pub fn run_problem(day: Day) {
                println!("Advent of Code {}\n", YEAR);

                #inputs

                #run_match
            }
        }
    };

    pm::TokenStream::from(expanded)
}

fn parse_lib_info(infos: pm::TokenStream) -> Result<LibInfo, ()> {
    let tokens: Vec<_> = infos.into_iter().collect();

    if let pm::TokenTree::Ident(i) = tokens.get(0).ok_or(())? {
        if i.to_string() != "year" {
            return Err(());
        }
    } else {
        return Err(());
    }

    if let pm::TokenTree::Punct(p) = tokens.get(1).ok_or(())? {
        if p.as_char() != '=' {
            return Err(());
        }
    } else {
        return Err(());
    }

    if let pm::TokenTree::Literal(l) = tokens.get(2).ok_or(())? {
        let repr = l.to_string();

        let year = repr.parse().map_err(|_| ())?;

        Ok(LibInfo { year })
    } else {
        Err(())
    }
}

fn gen_inputs(year: u32) -> pm2::TokenStream {
    let input_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()))
        .join("input")
        .join(&year.to_string());

    let days_with_input = fs::read_dir(&input_dir)
        .map(|files| {
            files
                .filter_map(Result::ok)
                .map(|file| file.file_name())
                .filter_map(|file_name| {
                    let file_name = file_name.to_str()?;
                    if file_name.starts_with("day") && file_name.ends_with(".txt") {
                        Some(file_name[..file_name.len() - 4].parse::<Day>().ok()?)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_else(|_| HashSet::new());

    let mut days: Vec<_> = base_day_part().map(|dp| dp.day).collect();
    days.sort();
    days.dedup();

    days.into_iter()
        .map(|d| {
            let name = utils::to_input(d);
            let input = format!("../../input/{}/{}.txt", year, d);
            if days_with_input.contains(&d) {
                quote! { let #name = Some(include_str!(#input)); }
            } else {
                quote! { let #name = None; }
            }
        })
        .collect()
}

fn gen_runner_structs() -> pm2::TokenStream {
    base_day_part()
        .map(|day_part| {
            let input_reader_struct = utils::to_camelcase(day_part, "InputReader");
            let runner_struct = utils::to_camelcase(day_part, "Runner");

            quote! {
                pub struct #input_reader_struct;

                impl InputReaderDefault for #input_reader_struct {}

                pub struct #runner_struct<I>(pub PhantomData<I>);

                impl<I> RunnerDefault for #runner_struct<I> {
                    type Input = I;
                }
            }
        })
        .collect()
}

fn match_arm(day_part: DayPart) -> pm2::TokenStream {
    let (print_template, err_template) = (
        format!(
            "Day {} - Part {}: {{}}\n\tInput Reader: {{:?}},\n\tRunner: {{:?}}\n",
            day_part.day.as_u8(),
            day_part.part.as_u8()
        ),
        format!(
            "Day {} - Part {}: FAILED while {{}}:\n{{:?}}\n",
            day_part.day.as_u8(),
            day_part.part.as_u8()
        ),
    );

    let input = utils::to_input(day_part.day);
    let input_reader = utils::to_camelcase(day_part, "InputReader");
    let runner = utils::to_camelcase(day_part, "Runner");

    quote! {
        if let Some(input) = #input {
            let start_time = Instant::now();

            let input_reader = &#input_reader;
            match input_reader.read(input) {
                Ok(parsed_input) => {
                    let runner = &#runner(PhantomData);
                    if runner.is_implemented() {
                        let inter_time = Instant::now();
                        match runner.run(&parsed_input) {
                            Ok(result) => {
                                let final_time = Instant::now();
                                println!(#print_template, result, (inter_time - start_time), (final_time - inter_time));
                            },
                            Err(e) => eprintln!(#err_template, "running", e),
                        }
                    }
                },
                Err(e) => eprintln!(#err_template, "reading input", e),
            }
        }
    }
}

fn gen_problem_match() -> pm2::TokenStream {
    let mut days: Vec<_> = base_day_part().map(|dp| dp.day).collect();
    days.sort();
    days.dedup();

    let arms: pm2::TokenStream = days
        .into_iter()
        .map(|d| {
            let part1 = match_arm(DayPart {
                day: d,
                part: 1u8.try_into().unwrap(),
            });
            let part2 = match_arm(DayPart {
                day: d,
                part: 2u8.try_into().unwrap(),
            });

            let day = d.as_u8();
            quote! {
                #day => {
                    #part1
                    #part2
                }
            }
        })
        .collect();

    quote! {
        match day.as_u8() {
            #arms
            _ => eprintln!("Unknown day: {}", day)
        }
    }
}

fn base_day_part() -> impl Iterator<Item = DayPart> {
    (1..=25).into_iter().flat_map(|day| {
        (1..=2).into_iter().map(move |part| DayPart {
            day: day.try_into().unwrap(),
            part: part.try_into().unwrap(),
        })
    })
}
