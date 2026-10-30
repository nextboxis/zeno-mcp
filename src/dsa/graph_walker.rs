// src/dsa/graph_walker.rs
// BFS/DFS Graph Walker for Obsidian Vault Wikilink & Security Attack Path Traversal.

use std::collections::{HashMap, HashSet, VecDeque};

pub struct VaultGraphWalker {
    adj: HashMap<String, Vec<String>>,
}

impl VaultGraphWalker {
    pub fn new() -> Self {
        Self { adj: HashMap::new() }
    }

    pub fn add_link(&mut self, from: &str, to: &str) {
        self.adj.entry(from.to_string()).or_default().push(to.to_string());
    }

    pub fn find_shortest_path(&self, start: &str, target: &str) -> Option<Vec<String>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(vec![start.to_string()]);
        visited.insert(start.to_string());

        while let Some(path) = queue.pop_front() {
            let last = path.last().unwrap();
            if last == target {
                return Some(path);
            }

            if let Some(neighbors) = self.adj.get(last) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        let mut next_path = path.clone();
                        next_path.push(neighbor.clone());
                        queue.push_back(next_path);
                    }
                }
            }
        }

        None
    }
}
