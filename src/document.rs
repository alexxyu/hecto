use std::fs;
use crate::Row;

#[derive(Default)]
pub struct Document {
    pub rows: Vec<Row>,
}

impl Document {
    /// # Errors
    /// 
    /// Will return `Err` if I/O error encountered while attempting to read file
    /// specified by `filename`
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let rows = contents.lines().map(Row::from).collect();
        Ok(Self { rows })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
