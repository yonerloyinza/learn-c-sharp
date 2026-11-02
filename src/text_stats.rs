use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct TextStats {
    pub bytes: usize,
    pub lines: usize,
    pub words: usize,
    word_counts: HashMap<String, usize>,
}

impl TextStats {
    pub fn from_text(text: &str) -> Self {
        let mut word_counts = HashMap::new();
        let mut words = 0;

        for word in text
            .split(|character: char| !character.is_alphanumeric())
            .filter(|word| !word.is_empty())
        {
            words += 1;
            *word_counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }

        Self {
            bytes: text.len(),
            lines: text.lines().count(),
            words,
            word_counts,
        }
    }

    pub fn most_frequent(&self, limit: usize) -> Vec<(&str, usize)> {
        let mut words: Vec<_> = self
            .word_counts
            .iter()
            .map(|(word, count)| (word.as_str(), *count))
            .collect();
        words.sort_unstable_by(|(word_a, count_a), (word_b, count_b)| {
            count_b.cmp(count_a).then_with(|| word_a.cmp(word_b))
        });
        words.truncate(limit);
        words
    }
}

pub fn analyze_file(path: &Path) -> Result<TextStats, io::Error> {
    let text = fs::read_to_string(path)?;
    Ok(TextStats::from_text(&text))
}

#[cfg(test)]
mod tests {
    use super::TextStats;

    #[test]
    fn counts_words_case_insensitively_and_sorts_ties() {
        let stats = TextStats::from_text("Rust rust, C#!");

        assert_eq!(stats.words, 3);
        assert_eq!(stats.most_frequent(2), vec![("rust", 2), ("c", 1)]);
    }

    #[test]
    fn empty_text_has_no_words_or_lines() {
        let stats = TextStats::from_text("");

        assert_eq!(stats.words, 0);
        assert_eq!(stats.lines, 0);
        assert!(stats.most_frequent(5).is_empty());
    }
}
