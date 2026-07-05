mod core;

use clap::Parser;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    size: String,

    mode: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.size == "version" {
        println!("rustcrypt 0.1.0");
        println!("Safe, Fast, Cryptographic.");
        return;
    }

    let size: usize = cli.size.parse().unwrap_or(32);
    let output = core::generate(size);

    let mode = cli.mode.unwrap_or_else(|| "text".to_string());

    if mode == "json" {
        let json = serde_json::json!({
            "output": output,
            "length": size
        });

        let mut file = File::create("output.json").unwrap();
        file.write_all(json.to_string().as_bytes()).unwrap();

        println!("{}", json);
    } else {
        println!("{}", output);
    }
}
