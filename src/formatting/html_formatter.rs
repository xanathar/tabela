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

use super::{Formatter, Table};
use std::borrow::Cow;

pub struct HtmlFormatter;

impl HtmlFormatter {
    fn escape(s: &str) -> Cow<'_, str> {
        let needs_escaping = s
            .bytes()
            .any(|c| c == b'"' || c == b'\'' || c == b'&' || c == b'<' || c == b'>');

        if needs_escaping {
            let mut res = String::with_capacity(s.len());

            for c in s.chars() {
                match c {
                    '&' => res.push_str("&amp;"),
                    '"' => res.push_str("&quot;"),
                    '<' => res.push_str("&lt;"),
                    '>' => res.push_str("&gt;"),
                    '\'' => res.push_str("&#39;"),
                    c => res.push(c),
                }
            }

            res.into()
        } else {
            s.into()
        }
    }

    fn format_row(result: &mut String, row: &[Cow<'_, str>], tag: &str) {
        result.push_str("\t<tr>\n");
        for cell in row.iter().map(|c| c.trim()) {
            let text = Self::escape(cell);
            result.push_str(&format!("\t\t<{tag}>{text}</{tag}>\n"));
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

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::formatting::HtmlFormatter;

    #[test]
    fn html_escapes() {
        assert_eq!(HtmlFormatter::escape("\"<>&\'"), "&quot;&lt;&gt;&amp;&#39;")
    }

    #[test]
    fn html_unnecessary_escapes() {
        let s = HtmlFormatter::escape("dfsfdsdfùù**ùù§");
        if let Cow::Owned(_) = s {
            panic!();
        }
    }
}
