use crate::args::Args;
use crate::language::{read_file_or_die_trying, Language};
use crate::library::{
    binary_to_binary, binary_to_c_const, binary_to_c_define, binary_to_csharp_const,
    binary_to_go_const, binary_to_hex, binary_to_java_const, binary_to_javascript_const,
    binary_to_python_const, binary_to_rust_const,
};
use clap::Parser;

mod args;
mod language;
mod library;

fn main() {
    let args = Args::parse();

    let file = read_file_or_die_trying(&args.input_file);

    let output = {
        match args.language {
            Language::Binary => binary_to_binary(&file),
            Language::Hex => binary_to_hex(&file),
            Language::C => binary_to_c_const(&file, &args.output_const_name, args.tab_size),
            Language::CDef => binary_to_c_define(&file, &args.output_const_name, args.tab_size),
            Language::Rust => binary_to_rust_const(&file, &args.output_const_name, args.tab_size),
            Language::CSharp => {
                binary_to_csharp_const(&file, &args.output_const_name, args.tab_size)
            }
            Language::Python => {
                binary_to_python_const(&file, &args.output_const_name, args.tab_size)
            }
            Language::Javascript => {
                binary_to_javascript_const(&file, &args.output_const_name, args.tab_size)
            }
            Language::Go => binary_to_go_const(&file, &args.output_const_name, args.tab_size),
            Language::Java => binary_to_java_const(&file, &args.output_const_name, args.tab_size),
        }
    };

    if let Some(output_file) = args.output_file {
        std::fs::write(&output_file, output);
    } else {
        println!("{output}");
    }
}
