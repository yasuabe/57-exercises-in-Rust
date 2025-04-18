/*
## Ex45: Word Finder

- Read a text file.
- Replace every instance of "utilize" with "use".
- Write the result to a new file.
- Constraint: Prompt the user for the output file name.
*/
use std::error::Error;
use clap::Parser;

const INPUT_PATH: &str = "data/ex45_input.txt";

#[derive(Parser)]
struct Cli {
    #[arg(short = 'o', long = "output")]
    output: String,
}

fn replace_utilize(text: &str) -> String {
    text.replace("utilize", "use")
}

fn write_output(output: &str, text: &str) -> Result<(), std::io::Error> {
    std::fs::write(output, text).map_err(|e|
        std::io::Error::new(e.kind(), "Write error"))
}

fn read_input() -> Result<String, std::io::Error> {
    std::fs::read_to_string(INPUT_PATH)
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli           = Cli::parse();
    let text          = read_input()?;
    let modified_text = replace_utilize(&text);
    write_output(&format!("output/{}", cli.output), &modified_text)?;

    Ok(())
}
