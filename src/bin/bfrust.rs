use clap::clap_app;

use std::env::temp_dir;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::process::Command;

use brainrust::{lex, run, compile};

fn main() {
    let matches = clap_app!(bfrust =>
        (version: "0.1")
        (author: "Ethan Tang <ethanyidong@gmail.com>")
        (about: "A brainf*ck interpreter written in Rust")
        (@arg FILE: * "The brainf*ck file to run")
        (@arg compile: -c --compile +takes_value "Compile to a binary rather than run")
        
    )
    .get_matches();
    let file_name = matches.value_of("FILE")
        .expect("No file provided");
    let file = File::open(&file_name)
        .expect("Error opening file");
    let tokens = lex(file);
    if let Some(f) = matches.value_of("compile") {
        let mut output = temp_dir();
        output.push("bfrust.rs");
        let mut output_file = File::create(&output)
            .expect("Error opening output file");
        output_file.write_all(compile(tokens).as_bytes())
            .expect("Error writing to output file");
        Command::new("rustc")
            .args(&["-C", "opt-level=3", "-o", &f, &output.to_str().unwrap()])
            .output()
            .expect("Error running rustc.");
    }
    else {
        run(tokens, stdin(), stdout());
    }
}
