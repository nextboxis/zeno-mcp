// src/dsa/trie.rs
// Prefix Trie for O(k) Skill Matching across 980 modular skills.

use std::collections::HashMap;

#[derive(Default)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub skill_ids: Vec<String>,
}

pub struct SkillTrie {
    root: TrieNode,
}

impl SkillTrie {
    pub fn new() -> Self {
        Self { root: TrieNode::default() }
    }

    pub fn insert(&mut self, keyword: &str, skill_id: String) {
        let mut curr = &mut self.root;
        for ch in keyword.chars() {
            curr = curr.children.entry(ch).or_default();
        }
        curr.skill_ids.push(skill_id);
    }

    pub fn search(&self, keyword: &str) -> Vec<String> {
        let mut curr = &self.root;
        for ch in keyword.chars() {
            match curr.children.get(&ch) {
                Some(next) => curr = next,
                None => return Vec::new(),
            }
        }
        curr.skill_ids.clone()
    }
}
