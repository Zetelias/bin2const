use crate::language::Language;
use clap::builder::Str;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(index = 1)]
    pub input_file: String,

    #[arg(index = 2, value_enum)]
    pub language: Language,

    #[arg(index = 3)]
    pub output_const_name: String,

    #[arg(short, long, default_value = "4")]
    pub tab_size: usize,

    #[arg(short, long)]
    pub output_file: Option<String>,
}
