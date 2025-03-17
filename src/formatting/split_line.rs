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

use std::{borrow::Cow, str::CharIndices};

pub struct SplitLine<'a> {
    line: &'a str,
    separator: char,
    remove_quotes: bool,
    indices: CharIndices<'a>,
}

impl<'a> SplitLine<'a> {
    fn new(line: &'a str, separator: char, remove_quotes: bool) -> Self {
        Self {
            line,
            separator,
            remove_quotes,
            indices: line.char_indices(),
        }
    }

    fn is_quote(chr: char) -> bool {
        chr == '\'' || chr == '"'
    }

    fn unescape_quoted_str(
        has_escaped_quotes: bool,
        quote: char,
        s: &'a str,
    ) -> Option<Cow<'a, str>> {
        if !has_escaped_quotes {
            Some(Cow::Borrowed(s))
        } else {
            let mut outp = String::with_capacity(s.len());

            let mut escaping = false;
            for c in s.chars() {
                if c == quote {
                    if escaping {
                        escaping = false;
                    } else {
                        escaping = true;
                        outp.push(c);
                    }
                } else {
                    escaping = false;
                    outp.push(c);
                }
            }

            Some(Cow::Owned(outp))
        }
    }
}

impl<'a> Iterator for SplitLine<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        let (start_offset, opening) = self.indices.next()?;

        if Self::is_quote(opening) {
            let start_offset = if self.remove_quotes {
                self.indices.offset()
            } else {
                start_offset
            };

            let mut last_quote_index = None;
            let mut has_escaped_quotes = false;

            for (index, chr) in self.indices.by_ref() {
                if chr == opening {
                    if last_quote_index.is_none() {
                        last_quote_index = Some(index);
                    } else {
                        last_quote_index = None;
                        has_escaped_quotes = true;
                    }
                    continue;
                }

                if chr == self.separator {
                    let Some(lqi) = last_quote_index else {
                        continue;
                    };

                    if self.remove_quotes {
                        return Self::unescape_quoted_str(
                            has_escaped_quotes,
                            opening,
                            &self.line[start_offset..lqi],
                        );
                    } else {
                        return Self::unescape_quoted_str(
                            has_escaped_quotes,
                            opening,
                            &self.line[start_offset..index],
                        );
                    }
                }

                last_quote_index = None;
            }

            match last_quote_index {
                Some(lqi) if self.remove_quotes => Self::unescape_quoted_str(
                    has_escaped_quotes,
                    opening,
                    &self.line[start_offset..lqi],
                ),
                _ => Self::unescape_quoted_str(
                    has_escaped_quotes,
                    opening,
                    &self.line[start_offset..],
                ),
            }
        } else {
            for (index, chr) in self.indices.by_ref() {
                if chr == self.separator {
                    return Some(Cow::Borrowed(&self.line[start_offset..index]));
                }
            }

            Some(Cow::Borrowed(&self.line[start_offset..]))
        }
    }
}

pub fn split_line(line: &'_ str, separator: char, remove_quotes: bool) -> SplitLine<'_> {
    SplitLine::new(line, separator, remove_quotes)
}

#[cfg(test)]
mod tests {
    use super::split_line;

    fn test(line: &str, remove_quotes: bool, expected: &[&str]) {
        let split = split_line(line, ',', remove_quotes)
            .map(|s| s.into_owned())
            .collect::<Vec<_>>();

        assert_eq!(split, expected);
    }

    #[test]
    fn line_split_trivial() {
        test("Ciao, Hello,Hola", false, &["Ciao", " Hello", "Hola"]);
    }

    #[test]
    fn line_split_quoted() {
        test(
            "Ciao,\" Hello\",Hola",
            false,
            &["Ciao", "\" Hello\"", "Hola"],
        );
    }

    #[test]
    fn line_split_quoted_remove() {
        test("Ciao,\" Hello\",Hola", true, &["Ciao", " Hello", "Hola"]);
    }

    #[test]
    fn line_split_quoted_truncated() {
        test("Ciao,\" Hello,Hola", false, &["Ciao", "\" Hello,Hola"]);
    }

    #[test]
    fn line_split_quoted_separator() {
        test(
            "Ciao,\" He,llo\",Hola",
            false,
            &["Ciao", "\" He,llo\"", "Hola"],
        );
    }

    #[test]
    fn line_split_quoted_separator_remove() {
        test("Ciao,\" He,llo\",Hola", true, &["Ciao", " He,llo", "Hola"]);
    }

    #[test]
    fn line_split_quoted_quotes() {
        test(
            "Ciao,\" He\"\"llo\",Hola",
            false,
            &["Ciao", "\" He\"llo\"", "Hola"],
        );
    }

    #[test]
    fn line_split_quoted_quotes_remove() {
        test(
            "Ciao,\" He\"\"llo\",Hola",
            true,
            &["Ciao", " He\"llo", "Hola"],
        );
    }

    #[test]
    fn line_split_quoted_last_quotes() {
        test(
            "Ciao,\" He\"\"llo\",\"Hola\"",
            false,
            &["Ciao", "\" He\"llo\"", "\"Hola\""],
        );
    }

    #[test]
    fn line_split_quoted_last_quotes_remove() {
        test(
            "Ciao,\" He\"\"llo\",\"Hola\"",
            true,
            &["Ciao", " He\"llo", "Hola"],
        );
    }

    #[test]
    fn line_split_mixed_quotes() {
        test(
            "Ciao,' He\"\"llo',Hola",
            false,
            &["Ciao", "' He\"\"llo'", "Hola"],
        );
    }

    #[test]
    fn line_split_mixed_quotes_remove() {
        test(
            "Ciao,' He\"\"llo',Hola",
            true,
            &["Ciao", " He\"\"llo", "Hola"],
        );
    }

    #[test]
    fn line_split_quoted_invalid() {
        test(
            "Ciao,\" He\"llo\",Hola",
            false,
            &["Ciao", "\" He\"llo\"", "Hola"],
        );
    }

    #[test]
    fn line_split_quoted_invalid_remove() {
        test(
            "Ciao,\" He\"llo\",Hola",
            true,
            &["Ciao", " He\"llo", "Hola"],
        );
    }
}

