use crate::args::Args;
use std::path::Path;
use std::str::FromStr;

// Define the enum
#[derive(Debug, Copy, Clone)]
pub enum Language {
    Binary,
    Hex,
    C,
    CDef,
    Rust,
    CSharp,
    Python,
    Javascript,
    Go,
    Java,
}

// Implement FromStr for the enum
impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bin" | "binary" => Ok(Language::Binary),
            "hex" | "hexadecimal" => Ok(Language::Hex),
            "c" | "c++" => Ok(Language::C),
            "cdef" | "cdefine" => Ok(Language::CDef),
            "rs" | "rust" => Ok(Language::Rust),
            "c#" | "csharp" => Ok(Language::CSharp),
            "py" | "python" => Ok(Language::Python),
            "js" | "javascript" => Ok(Language::Javascript),
            "go" | "golang" => Ok(Language::Go),
            "java" => Ok(Language::Java),
            _ => Err(format!("Invalid language: {}", s)),
        }
    }
}

pub fn read_file_or_die_trying(path: &str) -> Vec<u8> {
    let res = std::fs::read(path);
    match res {
        Ok(vec) => vec,
        Err(err) => {
            panic!("Error reading file '{path}': {err:?}");
        }
    }
}
