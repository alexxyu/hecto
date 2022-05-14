pub struct Row {
    pub string: String,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = end.min(self.string.len());
        let start = start.min(end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
        }
    }
}
