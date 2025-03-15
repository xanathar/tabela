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
use crate::table::Table;

pub trait Formatter {
    fn format(&self, table: Table) -> String;
}

pub struct HtmlFormatter;

impl HtmlFormatter {
    fn format_row(result: &mut String, row: &[&str], tag: &str) {
        result.push_str("\t<tr>\n");
        for cell in row.iter() {
            result.push_str(&format!("\t\t<{tag}>{cell}</{tag}>\n"));
        }
        result.push_str("\t</tr>\n");
    }
}

impl Formatter for HtmlFormatter {
    fn format(&self, table: Table) -> String {
        let mut result = String::new();

        result.push_str("<table>\n");

        if let Some(head) = table.titles() {
            Self::format_row(&mut result, head, "th");
        }

        for row in table.rows() {
            Self::format_row(&mut result, row, "td");
        }

        result.push_str("</table>\n");

        result
    }
}

pub struct MarkdownFormatter;

impl MarkdownFormatter {
    fn precalc_widths(widths: &mut Vec<usize>, row: &[&str]) {
        for (idx, cell) in row.iter().enumerate() {
            #[allow(clippy::comparison_chain)]
            if idx == widths.len() {
                widths.push(cell.len());
            } else if idx < widths.len() {
                widths[idx] = std::cmp::max(widths[idx], cell.len());
            } else {
                gtk::glib::g_error!("tabela", "Width index out of range");
            }
        }
    }

    fn add_cells(result: &mut String, widths: &[usize], row: &[&str]) {
        for (index, mut width) in widths.iter().copied().enumerate() {
            if index != 0 {
                result.push('|');
            }

            result.push(' ');

            if index < row.len() {
                result.push_str(row[index]);
                width = width.saturating_sub(row[index].len());
            }

            for _ in 0..=width {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    fn add_dashes(result: &mut String, widths: &[usize]) {
        for (index, width) in widths.iter().copied().enumerate() {
            if index != 0 {
                result.push('|');
            }

            for _ in 0..=width + 1 {
                result.push('-');
            }
        }
        result.push('\n');
    }
}

impl Formatter for MarkdownFormatter {
    fn format(&self, table: Table) -> String {
        let mut widths = Vec::new();

        if let Some(head) = table.titles() {
            Self::precalc_widths(&mut widths, head);
        }

        for row in table.rows() {
            Self::precalc_widths(&mut widths, row);
        }

        let mut result = String::new();

        if let Some(head) = table.titles() {
            Self::add_cells(&mut result, &widths, head);
            Self::add_dashes(&mut result, &widths);
        }

        for row in table.rows() {
            Self::add_cells(&mut result, &widths, row);
        }

        result
    }
}
