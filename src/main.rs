use clap::Parser;
use std::fs;

/// Simple Text Cleaning Tool
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    input: String,
    output: String,

    #[arg(long)]
    remove_empty: bool,

    #[arg(long)]
    trim: bool,

    #[arg(long)]
    lowercase: bool,
}

fn main() {
    let args = Args::parse();

    let content =
        fs::read_to_string(&args.input).expect("Failed to read file");

    let cleaned = clean_text(
        &content,
        args.remove_empty,
        args.trim,
        args.lowercase,
    );

    fs::write(&args.output, cleaned)
        .expect("Failed to write file");

    println!("File cleaned successfully.");
}

fn clean_text(
    text: &str,
    remove_empty: bool,
    trim: bool,
    lowercase: bool,
) -> String {
    let mut lines: Vec<String> =
        text.lines().map(|l| l.to_string()).collect();

    if trim {
        lines = lines
            .into_iter()
            .map(|l| l.trim_end().to_string())
            .collect();
    }

    if remove_empty {
        lines = lines
            .into_iter()
            .filter(|l| !l.trim().is_empty())
            .collect();
    }

    let mut result = lines.join("\n");

    if lowercase {
        result = result.to_lowercase();
    }

    result
}

