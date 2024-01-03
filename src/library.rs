/// Loads a Vec<u8> from a file.
pub fn file_to_binary(file: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(file)
}

/// Converts an array of bytes to hex disassembly.
/// For exemple, with binary = &[0x00, 0x01, 0x02, 0x03], the function returns:
/// 00000000  00 01 02 03                                       |....|
/// 00000004
pub fn binary_to_hex(binary: &[u8]) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < binary.len() {
        out.push_str(&format!("{:08x}  ", i));
        for j in 0..16 {
            if i + j < binary.len() {
                out.push_str(&format!("{:02x} ", binary[i + j]));
            } else {
                out.push_str("   ");
            }
            if j % 4 == 3 {
                out.push_str(" ");
            }
        }
        out.push_str(" |");
        for j in 0..16 {
            if i + j < binary.len() {
                let c = binary[i + j];
                if c >= 0x20 && c <= 0x7e {
                    out.push(c as char);
                } else {
                    out.push('.');
                }
            } else {
                out.push(' ');
            }
        }
        out.push_str("|\n");
        i += 16;
    }
    out
}

/// Converts an array of bytes to binary disassembly.
/// For exemple, with binary = &[0x00, 0x01, 0x02, 0x03], the function returns:
/// 00000000  00000000 00000001 00000010 00000011                 |....|
/// 00000004
pub fn binary_to_binary(binary: &[u8]) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < binary.len() {
        out.push_str(&format!("{:08x}  ", i));
        for j in 0..16 {
            if i + j < binary.len() {
                out.push_str(&format!("{:08b} ", binary[i + j]));
            } else {
                out.push_str("         ");
            }
            if j % 4 == 3 {
                out.push_str(" ");
            }
        }
        out.push_str(" |");
        for j in 0..16 {
            if i + j < binary.len() {
                let c = binary[i + j];
                if c >= 0x20 && c <= 0x7e {
                    out.push(c as char);
                } else {
                    out.push('.');
                }
            } else {
                out.push(' ');
            }
        }
        out.push_str("|\n");
        i += 16;
    }
    out
}

/// Converts an array of bytes to a C constant.
/// For exemple, with binary = &[0x00, 0x01, 0x02, 0x03] and name = "test_txt", the function returns:
/// const unsigned char TEST_TXT[] = {
///    0x00, 0x01, 0x02, 0x03
/// };
pub fn binary_to_c_const(binary: &[u8], name: &str, tab_size: usize) -> String {
    let mut out = String::new();
    let generated_tabs = " ".repeat(tab_size);
    out.push_str(&format!(
        "const unsigned char {}[] = {{\n{}",
        name, generated_tabs
    ));
    for (i, byte) in binary.iter().enumerate() {
        out.push_str(&format!("0x{:02x}", byte));
        if i < binary.len() - 1 {
            out.push_str(", ");
        }
        if i % 16 == 15 {
            out.push_str(format!("{}\n", generated_tabs).as_str());
        }
    }
    out.push_str("\n};\n");
    out
}

/// Converts an array of bytes to a C/C++ #define.
/// For exemple, with binary = &[0x00, 0x01, 0x02, 0x03] and name = "test_txt", the function returns:
/// #define TEST_TXT_SIZE 4
/// #define TEST_TXT { 0x00, 0x01, 0x02, 0x03 }
/// It is capable of multi-line #define, for exemple:
/// #define TEST_TXT { 0x00, 0x01, 0x02, 0x03, \
///                   0x04, 0x05, 0x06, 0x07 }
pub fn binary_to_c_define(binary: &[u8], name: &str, tab_size: usize) -> String {
    let mut out = String::new();
    let generated_tabs = " ".repeat(tab_size);

    out.push_str(&format!("#define {}_SIZE {}\n", name, binary.len()));
    out.push_str(&format!("#define {} {{", name));

    let mut count = 0;
    let mut line_start = false;
    for (i, byte) in binary.iter().enumerate() {
        if !line_start {
            out.push_str(&format!("{}    ", generated_tabs));
            line_start = true;
        }

        out.push_str(&format!("0x{:02x}", byte));

        if i < binary.len() - 1 {
            out.push_str(", ");
        }

        count += 1;

        if count % 8 == 0 && i < binary.len() - 1 {
            out.push_str("\\\n");
            line_start = false;
        }
    }

    out.push_str("}\n");
    out
}

/// Converts an array of bytes to a Rust constant.
/// For exemple, with binary = &[0x00, 0x01, 0x02, 0x03] and name = "test_txt", the function returns:
/// const TEST_TXT: [u8; 4] = [0x00, 0x01, 0x02, 0x03];
pub fn binary_to_rust_const(binary: &[u8], name: &str, tab_size: usize) -> String {
    let mut out = String::new();
    let generated_tabs = " ".repeat(tab_size);
    out.push_str(&format!(
        "const {}: [u8; {}] = [\n{}",
        name,
        binary.len(),
        generated_tabs
    ));
    for (i, byte) in binary.iter().enumerate() {
        out.push_str(&format!("0x{:02x}", byte));
        if i < binary.len() - 1 {
            out.push_str(", ");
        }
        if i % 16 == 15 {
            out.push_str(format!("{}\n", generated_tabs).as_str());
        }
    }
    out.push_str("\n];\n");
    out
}

/// Converts an array of bytes to a python constant.
/// For exemple, with binary = &[0x00, 0x01, 0x02, 0x03] and name = "test_txt", the function returns:
/// TEST_TXT = bytes([0x00, 0x01, 0x02, 0x03])
pub fn binary_to_python_const(binary: &[u8], name: &str, tab_size: usize) -> String {
    let mut out = String::new();
    let generated_tabs = " ".repeat(tab_size);
    out.push_str(&format!("{} = bytes([\n{}", name, generated_tabs));
    for (i, byte) in binary.iter().enumerate() {
        out.push_str(&format!("0x{:02x}", byte));
        if i < binary.len() - 1 {
            out.push_str(", ");
        }
        if i % 16 == 15 {
            out.push_str(format!("{}\n", generated_tabs).as_str());
        }
    }
    out.push_str("\n])\n");
    out
}

/// Converts an array of bytes to a C# constant.
/// For exemple, with binary = &[0x00, 0x01, 0x02, 0x03] and name = "test_txt", the function returns:
/// public static readonly byte[] TEST_TXT = new byte[] {
///    0x00, 0x01, 0x02, 0x03
/// };
pub fn binary_to_csharp_const(binary: &[u8], name: &str, tab_size: usize) -> String {
    let mut out = String::new();
    let generated_tabs = " ".repeat(tab_size);
    out.push_str(&format!(
        "public static readonly byte[] {} = new byte[] {{\n{}",
        name, generated_tabs
    ));
    for (i, byte) in binary.iter().enumerate() {
        out.push_str(&format!("0x{:02x}", byte));
        if i < binary.len() - 1 {
            out.push_str(", ");
        }
        if i % 16 == 15 {
            out.push_str(format!("{}\n", generated_tabs).as_str());
        }
    }
    out.push_str("\n};\n");
    out
}

/// Converts an array of bytes to a Javascript constant.
/// For exemple, with binary = &[0x00, 0x01, 0x02, 0x03] and name = "test_txt", the function returns:
/// const TEST_TXT = new Uint8Array([
///    0x00, 0x01, 0x02, 0x03
/// ]);
pub fn binary_to_javascript_const(binary: &[u8], name: &str, tab_size: usize) -> String {
    let mut out = String::new();
    let generated_tabs = " ".repeat(tab_size);
    out.push_str(&format!(
        "const {} = new Uint8Array([\n{}",
        name, generated_tabs
    ));
    for (i, byte) in binary.iter().enumerate() {
        out.push_str(&format!("0x{:02x}", byte));
        if i < binary.len() - 1 {
            out.push_str(", ");
        }
        if i % 16 == 15 {
            out.push_str(format!("{}\n", generated_tabs).as_str());
        }
    }
    out.push_str("\n]);\n");
    out
}

/// Converts an array of bytes to a Go constant.
/// For exemple, with binary = &[0x00, 0x01, 0x02, 0x03] and name = "test_txt", the function returns:
/// var TEST_TXT = []byte{
///   0x00, 0x01, 0x02, 0x03
/// }
pub fn binary_to_go_const(binary: &[u8], name: &str, tab_size: usize) -> String {
    let mut out = String::new();
    let generated_tabs = " ".repeat(tab_size);
    out.push_str(&format!("var {} = []byte{{\n{}", name, generated_tabs));
    for (i, byte) in binary.iter().enumerate() {
        out.push_str(&format!("0x{:02x}", byte));
        if i < binary.len() - 1 {
            out.push_str(", ");
        }
        if i % 16 == 15 {
            out.push_str(format!("{}\n", generated_tabs).as_str());
        }
    }
    out.push_str("\n}\n");
    out
}

/// Converts an array of bytes to a Java constant.
/// For exemple, with binary = &[0x00, 0x01, 0x02, 0x03] and name = "test_txt", the function returns:
/// public static final byte[] TEST_TXT = new byte[] {
///   0x00, 0x01, 0x02, 0x03
/// };
pub fn binary_to_java_const(binary: &[u8], name: &str, tab_size: usize) -> String {
    let mut out = String::new();
    let generated_tabs = " ".repeat(tab_size);
    out.push_str(&format!(
        "public static final byte[] {} = new byte[] {{\n{}",
        name, generated_tabs
    ));
    for (i, byte) in binary.iter().enumerate() {
        out.push_str(&format!("0x{:02x}", byte));
        if i < binary.len() - 1 {
            out.push_str(", ");
        }
        if i % 16 == 15 {
            out.push_str(format!("{}\n", generated_tabs).as_str());
        }
    }
    out.push_str("\n};\n");
    out
}
