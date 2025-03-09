pub struct Table<'a> {
    head: Vec<&'a str>,
    rows: Vec<Vec<&'a str>>,
}

impl<'a> Table<'a> {
    pub fn with_text_and_separator(text: &'a str, separator: char, has_head: bool) -> Self {
        let lines = text.split('\n');
        let mut head = Vec::new();
        let mut rows = Vec::new();

        log::info!("Generating table with separator '{separator}' and titles={has_head}");

        for l in lines {
            if l.is_empty() {
                continue;
            }

            let cells = l.split(separator);

            if has_head && head.is_empty() {
                head = cells.collect();
            } else {
                rows.push(cells.collect());
            }
        }

        Self { head, rows }
    }

    pub fn titles(&self) -> Option<&[&str]> {
        if self.head.is_empty() {
            None
        } else {
            Some(&self.head)
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &[&str]> {
        self.rows.iter().map(|v| v.as_slice())
    }
    /*
    pub fn row(&self, row: usize) -> &[&str] {
        &self.data[row]
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    */
}
