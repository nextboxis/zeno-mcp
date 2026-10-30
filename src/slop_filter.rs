// src/slop_filter.rs
// Removes conversational padding, sycophancy, and boilerplate fluff in pure Rust.

use regex::Regex;

pub struct SlopFilter {
    slop_patterns: Vec<Regex>,
}

impl SlopFilter {
    pub fn new() -> Self {
        let patterns = vec![
            Regex::new(r"(?i)^sure,? here is the (code|solution|plan|report):?\s*").unwrap(),
            Regex::new(r"(?i)^certainly!? let('s| us) (dive in|get started|take a look):?\s*").unwrap(),
            Regex::new(r"(?i)as an ai language model,?\s*").unwrap(),
            Regex::new(r"(?i)i hope this (helps|meets your needs)!?.*$").unwrap(),
            Regex::new(r"(?i)feel free to ask if you have any (more )?questions!?.*$").unwrap(),
            Regex::new(r"(?i)in conclusion,?\s*").unwrap(),
        ];
        Self { slop_patterns: patterns }
    }

    pub fn purify(&self, raw_content: &str) -> String {
        let mut cleaned = raw_content.to_string();
        for pattern in &self.slop_patterns {
            cleaned = pattern.replace_all(&cleaned, "").to_string();
        }
        cleaned.trim().to_string()
    }
}
