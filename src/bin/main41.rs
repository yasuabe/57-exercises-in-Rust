/*
## Ex40: Filtering Records

- Read a list of names from a file.
- Sort the names alphabetically.
- Output:
   - Total number of names.
   - A separator line.
   - The sorted names.
- Do not hard-code the number of names.
*/
use std::io::{BufWriter, Write};
use std::fs;
use std::fs::File;
use std::error::Error;

static FILE_PATH: &str        = "data/names.txt";
static SORTED_FILE_PATH: &str = "data/sorted_names.txt";

fn read_names_from_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents
        .lines()
        .map(|line| line.to_string())
        .collect())
}
fn write_names<W: Write>(writer: &mut W, names: &[String]) -> Result<(), Box<dyn Error>> {
    writeln!(writer, "Total of {} names", names.len())?;
    writeln!(writer, "-------------------------")?;

    for name in names {
        writeln!(writer, "{}", name)?
    }
    Ok(())
}
fn write_sorted_names_to_file(file_path: &str, names: &[String]) -> Result<(), Box<dyn Error>> {
    let file        = File::create(file_path)?;
    let mut writer  = BufWriter::new(file);

    write_names(&mut writer, names)?;
    writer.flush()?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut names = read_names_from_file(FILE_PATH)?;
    names.sort();
    write_sorted_names_to_file(SORTED_FILE_PATH, &names)?;
    Ok(())
}
