/* MIT License
 *
 * Copyright (c) 2025 Marco Mastropaolo
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * SPDX-License-Identifier: MIT
 */

use super::split_line;
use std::borrow::Cow;

pub struct Table<'a> {
    head: Vec<Cow<'a, str>>,
    rows: Vec<Vec<Cow<'a, str>>>,
}

impl<'a> Table<'a> {
    pub fn with_text_and_separator(
        text: &'a str,
        separator: char,
        has_head: bool,
        remove_quotes: bool,
    ) -> Self {
        let lines = text.split('\n');
        let mut head = Vec::new();
        let mut rows = Vec::new();

        gtk::glib::g_info!(
            "tabela",
            "Generating table with separator '{separator}' and titles={has_head}"
        );

        for l in lines {
            if l.is_empty() {
                continue;
            }

            let cells = split_line(l, separator, remove_quotes);

            if has_head && head.is_empty() {
                head = cells.collect();
            } else {
                rows.push(cells.collect());
            }
        }

        Self { head, rows }
    }

    pub fn titles(&self) -> Option<&[Cow<'a, str>]> {
        if self.head.is_empty() {
            None
        } else {
            Some(&self.head)
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &[Cow<'a, str>]> {
        self.rows.iter().map(|v| v.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare(table: Table<'_>, titles: &[&str], expected: &[&[&str]]) {
        if let Some(tit) = table.titles() {
            assert_eq!(tit, titles);
        }

        let mut exp_idx = 0;
        for row in table.rows() {
            assert_eq!(row, expected[exp_idx]);
            exp_idx += 1;
        }

        assert_eq!(exp_idx, expected.len());
    }

    #[test]
    fn table_happy_case() {
        let t = Table::with_text_and_separator("t1,t2,t3\na1,a2,a3\nb1,b2,b3", ',', true, true);

        compare(
            t,
            &["t1", "t2", "t3"],
            &[&["a1", "a2", "a3"], &["b1", "b2", "b3"]],
        );
    }

    #[test]
    fn table_happy_case_no_titles() {
        let t = Table::with_text_and_separator("t1,t2,t3\na1,a2,a3\nb1,b2,b3", ',', false, true);

        compare(
            t,
            &[],
            &[
                &["t1", "t2", "t3"],
                &["a1", "a2", "a3"],
                &["b1", "b2", "b3"],
            ],
        );
    }

    #[test]
    fn table_empty_field() {
        let t = Table::with_text_and_separator("t1,t2,t3\na1,,a3\nb1,b2,b3", ',', false, true);

        compare(
            t,
            &[],
            &[&["t1", "t2", "t3"], &["a1", "", "a3"], &["b1", "b2", "b3"]],
        );
    }
}
