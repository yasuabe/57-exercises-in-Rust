/*
## Ex45: Word Finder

- Read a text file.
- Replace every instance of "utilize" with "use".
- Write the result to a new file.
- Constraint: Prompt the user for the output file name.
*/
use clap::Parser;
use anyhow::{Result, Context};

const INPUT_PATH: &str = "data/ex45_input.txt";

#[derive(Parser)]
struct Cli {
    #[arg(short = 'o', long = "output")]
    output: String,
}

fn mk_output_path() -> String {
    format!("output/{}", Cli::parse().output)
}
fn replace_words(text: &str) -> String {
    text.replace("utilize", "use")
}

fn write_output(output: &str, text: &str) -> Result<()> {
    std::fs::write(output, text)
        .with_context(|| format!("Failed to write to {}", output))
}

fn read_original() -> Result<String> {
    std::fs::read_to_string(INPUT_PATH)
        .with_context(|| format!("Failed to read {}", INPUT_PATH))
}

fn main() -> Result<()> {
    let out_path = mk_output_path();
    let original = read_original()?;
    let modified = replace_words(&original);

    write_output(&out_path, &modified)
}
