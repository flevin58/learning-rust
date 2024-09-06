use std::fs::File;
use std::io::Read;

// This function returns the contents of an embedded text file as a string
// Note that the name of the file MUST be a literal (you cannot use variables)
fn read_embedded_text_file() -> String {
    let hello = include_str!("assets/hello.txt");
    hello.trim().to_string()
}

// This function returns the contents of an external text file as a string
// Note that the path to the file is relative to ...
fn read_external_text_file() -> String {
    let mut fh = File::open(
        "/Users/flevin58/Documents/Sviluppo/Rust/learning-rust/examples/embed/assets/hello.txt",
    )
    .expect("Cannot open file");
    let mut contents = String::new();
    let _ = fh.read_to_string(&mut contents);
    contents.trim().to_string()
}

fn main() {
    let emb_str = read_embedded_text_file();
    println!("Embedded file: {emb_str}");
    let ext_str = read_external_text_file();
    println!("External file: {ext_str}");
}
