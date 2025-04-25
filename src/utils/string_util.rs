pub trait StripMargin {
    fn strip_margin(&self) -> String;
}

impl StripMargin for String {
    fn strip_margin(&self) -> String {
        self.lines()
            .map(|line| {
                line.trim_start().strip_prefix('|').unwrap_or(line)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
