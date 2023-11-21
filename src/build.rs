use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let file_path = Path::new("./src/environment.rs");
    let mut output_file = File::create(&file_path).unwrap();

    let content = format!(
        "pub fn get_decryption_key() -> String {{ \
            return String::from(obfstr::obfstr!(\"{}\")); \
        }}",
        get_encryption_key(),
    );

    output_file.write_all(content.as_bytes()).unwrap();
}

fn get_encryption_key() -> String {
    return env::var("ENCRYPTION_KEY").unwrap_or_else(|_| String::from("my static key"));
}
