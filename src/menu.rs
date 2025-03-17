use std::collections::HashMap;

use crate::numeric;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub body: String,
    pub prefix_len: usize,
    pub auto_accept: bool,
}

impl Entry {
    pub fn prefix(&self) -> &str {
        &self.body[..self.prefix_len]
    }

    pub fn is_selectable(&self, input: &str) -> bool {
        self.body.starts_with(input)
    }

    pub fn is_acceptable(&self, input: &str) -> bool {
        self.auto_accept && input == self.prefix()
    }
}

fn generate_entries(lines: &[String]) -> Vec<Entry> {
    let mut char_iters: Vec<_> = lines.iter().map(|ln| ln.chars()).collect();
    let mut groups: Vec<(usize, Vec<usize>)> = vec![(0, (0..lines.len()).collect())];
    let mut entries = HashMap::<usize, Entry>::new();

    while let Some((prefix_len, indices)) = groups.pop() {
        if let [idx] = indices[..] {
            // Just one line with this prefix, it's over.
            entries.insert(
                idx,
                Entry {
                    body: lines[idx].clone(),
                    prefix_len,
                    auto_accept: true,
                },
            );
        } else {
            // More than one line with the prefix.
            let mut lines_by_next_char = HashMap::<char, Vec<usize>>::new();

            for idx in indices {
                // This will always be the next character after the current prefix,
                // since chars are consumed precisely one-by-one as they are required.
                // That is:
                // charss[idx].next() = lines[idx].chars().nth(prefix.len())
                match char_iters[idx].next() {
                    None => {
                        // This line is a prefix of some other line, so it can't be
                        // auto-accepted.
                        entries.insert(
                            idx,
                            Entry {
                                body: lines[idx].clone(),
                                prefix_len,
                                auto_accept: false,
                            },
                        );
                        continue;
                    }
                    Some(next_ch) => {
                        lines_by_next_char.entry(next_ch).or_default().push(idx);
                    }
                }
            }

            for group_indices in lines_by_next_char.into_values() {
                groups.push((prefix_len + 1, group_indices))
            }
        }
    }

    (0..lines.len())
        .map(|i| entries.remove(&i).unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub enum SearchDirection {
    Forwards,
    Backwards,
}

#[derive(Debug, Clone)]
pub struct Menu {
    entries: Vec<Entry>,
    selection: Option<usize>,
}

impl Menu {
    pub fn from_lines(lines: &[String]) -> Self {
        assert!(!lines.is_empty(), "lines must have at least one element");

        let entries = generate_entries(lines);
        Self {
            entries,
            selection: Some(0),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn selection(&self) -> Option<&usize> {
        self.selection.as_ref()
    }

    pub fn has_selectable(&self, input: &str) -> bool {
        self.entries.iter().any(|entry| entry.is_selectable(input))
    }

    pub fn find_acceptable(&self, input: &str) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.is_acceptable(input))
    }

    pub fn manual_accept(&self) -> Option<&Entry> {
        self.selection.map(|sel| &self.entries[sel])
    }

    pub fn update_selection(&mut self, new_input: &str) -> bool {
        let start = self.selection.unwrap_or(0);

        let search_result = self.entries[start..]
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.is_selectable(new_input))
            .map(|(i, _)| start + i)
            .or_else(|| {
                self.entries[..start]
                    .iter()
                    .enumerate()
                    .rfind(|(_, entry)| entry.is_selectable(new_input))
                    .map(|(i, _)| i)
            });

        if self.selection != search_result {
            self.selection = search_result;
            true
        } else {
            false
        }
    }

    pub fn move_selection(&mut self, input: &str, direction: SearchDirection, wrap: bool) -> bool {
        let mut candidate = self.selection.unwrap_or(0);
        let mut did_wrap;

        loop {
            (candidate, did_wrap) = match direction {
                SearchDirection::Forwards => numeric::wrapping_inc(candidate, self.entries.len()),
                SearchDirection::Backwards => numeric::wrapping_dec(candidate, self.entries.len()),
            };

            if did_wrap && !wrap {
                break false;
            }

            if self.entries[candidate].is_selectable(input) {
                self.selection = Some(candidate);
                break true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_entries_empty() {
        assert_eq!(generate_entries(&[]), vec![]);
    }

    #[test]
    fn test_generate_entries_simple() {
        let lines = ["aaa", "bbb", "ccc"].map(str::to_string);
        assert_eq!(
            generate_entries(&lines),
            vec![
                Entry {
                    body: "aaa".to_string(),
                    prefix_len: 1,
                    auto_accept: true
                },
                Entry {
                    body: "bbb".to_string(),
                    prefix_len: 1,
                    auto_accept: true
                },
                Entry {
                    body: "ccc".to_string(),
                    prefix_len: 1,
                    auto_accept: true
                },
            ]
        );
    }

    #[test]
    fn test_generate_entries_complex() {
        let lines = ["abcddd", "abce", "abb", "cc", "ccd"].map(str::to_string);
        assert_eq!(
            generate_entries(&lines),
            vec![
                Entry {
                    body: "abcddd".to_string(),
                    prefix_len: 4,
                    auto_accept: true
                },
                Entry {
                    body: "abce".to_string(),
                    prefix_len: 4,
                    auto_accept: true
                },
                Entry {
                    body: "abb".to_string(),
                    prefix_len: 3,
                    auto_accept: true
                },
                Entry {
                    body: "cc".to_string(),
                    prefix_len: 2,
                    auto_accept: false
                },
                Entry {
                    body: "ccd".to_string(),
                    prefix_len: 3,
                    auto_accept: true
                },
            ]
        );
    }

    #[test]
    fn test_generate_entries_repeated() {
        let lines = ["aaa", "aaa", "bbb", "cc", "cc"].map(str::to_string);
        assert_eq!(
            generate_entries(&lines),
            vec![
                Entry {
                    body: "aaa".to_string(),
                    prefix_len: 3,
                    auto_accept: false
                },
                Entry {
                    body: "aaa".to_string(),
                    prefix_len: 3,
                    auto_accept: false
                },
                Entry {
                    body: "bbb".to_string(),
                    prefix_len: 1,
                    auto_accept: true
                },
                Entry {
                    body: "cc".to_string(),
                    prefix_len: 2,
                    auto_accept: false
                },
                Entry {
                    body: "cc".to_string(),
                    prefix_len: 2,
                    auto_accept: false
                },
            ]
        );
    }
}
