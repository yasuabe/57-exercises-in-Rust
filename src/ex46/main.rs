/*
## Ex46: Word Frequency Finder

- Read a text file.
- Count word frequencies.
- Display a histogram using * to show counts.
- Sort output from most frequent to least frequent.
*/
use std::error::Error;
use itertools::Itertools;

static INPUT_PATH: &str = "src/ex46/words.txt";

fn read_words(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .flat_map(|line| line.split_whitespace())
        .map(str::to_string)
        .collect())
}
fn analyze_words(words: Vec<String>) -> Vec<(String, usize)> {
    words.into_iter()
        .map(|word| word.to_lowercase())
        .sorted()
        .chunk_by(|word| word.clone())
        .into_iter()
        .map(|(word, group)| (word, group.count()))
        .sorted_by(|(_, count1), (_, count2)| count2.cmp(count1))
        .collect()
}
fn print_analysis(analysis: &[(String, usize)]) {
    let max_chars = analysis
        .iter()
        .map(|(word, _)| word.len())
        .max()
        .unwrap_or(0);
    for (word, count) in analysis {
        println!("{:<width$} {}", format!("{}:", word), "*".repeat(*count), width = max_chars+1);
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let words    = read_words(INPUT_PATH)?;
    let analysis = analyze_words(words.clone());
    print_analysis(analysis.as_slice());
    Ok(())
}
