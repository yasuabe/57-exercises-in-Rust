/*
## Ex41: Name Sorter

- Read a list of names from a file.
- Sort the names alphabetically.
- Output:
   - Total number of names.
   - A separator line.
   - The sorted names.
- Do not hard-code the number of names.
*/
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::error::Error;

static INPUT_PATH:  &str = "data/names.txt";
static OUTPUT_PATH: &str = "data/sorted_names.txt";

fn read_names(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(|line| line.to_string())
        .collect())
}
fn write_names(file_path: &str, names: &[String]) -> Result<(), Box<dyn Error>> {
    let file       = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    write_contents(&mut writer, names)
}
fn write_contents<W: Write>(writer: &mut W, names: &[String]) -> Result<(), Box<dyn Error>> {
    writeln!(writer, "Total of {} names", names.len())?;
    writeln!(writer, "-------------------------")?;

    for name in names {
        writeln!(writer, "{}", name)?
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut names = read_names(INPUT_PATH)?;
    names.sort();
    write_names(OUTPUT_PATH, &names)?;
    Ok(())
}
