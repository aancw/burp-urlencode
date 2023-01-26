// Copyright (c) 2023 Petruknisme
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate clap;
extern crate colored;

use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
#[clap(name = "burp-urlencode")]
#[clap(author = "Petruknisme <me@petruknisme.com>")]
#[clap(version = "1.0")]
#[clap(about = "Burp Suite URL encoding all characters implementation in Rust", long_about = None)]

struct Cli {
    /// Text to be encoded
    #[clap(short, long)]
    text: String,

    /// How many time text will be url encoded
    #[clap(short, long)]
    iteration: i32,

    /// Silent mode output, only show result text
    #[clap(short, long)]
    silent: bool,
}

struct Encode {
    value: String,
}

fn urlencode(text: &str) -> String {
    let mut hex = String::new();

    for c in text.chars() {
        hex.push_str(&format!(" {:x}", c as u32));
    }
    hex.replace(' ', "%")
}

fn main() {
    let cli = Cli::parse();
    let text = cli.text;
    let i = cli.iteration;
    let silent = cli.silent;

    let mut encode = Encode {
        value: text.to_string(),
    };

    for n in 0..i {
        if n == 0 {
            encode.value = urlencode(&text);
        } else {
            encode.value = urlencode(&encode.value);
        }
    }

    if silent {
        println!("{}", encode.value);
    } else {
        println!(
            "{}",
            "
        Burp Suite URL encoding all characters implementation in Rust
        by Petruknisme
        "
            .yellow()
        );
        println!("{}", "[+] Encoding...".yellow());
        println!("Text: {}", &text);
        println!("Iteration: {}", &i);

        println!("{}", "[+] Result: ".green());
        println!("{}", encode.value);
    }
}
