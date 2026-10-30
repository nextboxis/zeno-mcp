// src/dsa/hnsw.rs
// Pure Native Rust Hierarchical Navigable Small World (HNSW) Graph Index.
// Sub-millisecond dense semantic vector search for 5,300+ Obsidian Notes.

use crate::edge_slm::EdgeSlmEngine;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct HnswNode {
    pub id: String,
    pub title: String,
    pub preview: String,
    pub vector: Vec<f32>,
    pub layer_neighbors: Vec<Vec<usize>>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub preview: String,
    pub similarity: f32,
}

#[derive(Debug, Clone)]
struct Candidate {
    dist: f32,
    idx: usize,
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist && self.idx == other.idx
    }
}

impl Eq for Candidate {}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap behavior
        other.dist.partial_cmp(&self.dist).unwrap_or(Ordering::Equal)
    }
}

pub struct HnswIndex {
    pub nodes: Vec<HnswNode>,
    pub enter_point: Option<usize>,
    pub max_layer: usize,
    pub m: usize,
    pub ef_construction: usize,
}

impl Default for HnswIndex {
    fn default() -> Self {
        Self::new(16, 32)
    }
}

impl HnswIndex {
    pub fn new(m: usize, ef_construction: usize) -> Self {
        Self {
            nodes: Vec::new(),
            enter_point: None,
            max_layer: 0,
            m,
            ef_construction,
        }
    }

    pub fn insert(&mut self, id: String, title: String, preview: String, vector: Vec<f32>) {
        let node_idx = self.nodes.len();
        // Deterministic pseudo-random layer assignment based on vector hash
        let target_layer = self.assign_layer(&vector);

        let mut node = HnswNode {
            id,
            title,
            preview,
            vector,
            layer_neighbors: vec![Vec::new(); target_layer + 1],
        };

        if self.nodes.is_empty() {
            self.nodes.push(node);
            self.enter_point = Some(node_idx);
            self.max_layer = target_layer;
            return;
        }

        let mut curr_entry = self.enter_point.unwrap();

        // Greedily navigate down from max_layer to target_layer + 1
        for layer in (target_layer + 1..=self.max_layer).rev() {
            curr_entry = self.greedy_closest(&node.vector, curr_entry, layer);
        }

        // On layers from target_layer down to 0, connect neighbors
        for layer in (0..=target_layer.min(self.max_layer)).rev() {
            let candidates = self.search_layer(&node.vector, curr_entry, self.ef_construction, layer);

            let selected_neighbors: Vec<usize> = candidates
                .iter()
                .take(self.m)
                .map(|c| c.idx)
                .collect();

            // Bidirectional connection
            for &n_idx in &selected_neighbors {
                node.layer_neighbors[layer].push(n_idx);
                if self.nodes[n_idx].layer_neighbors.len() > layer {
                    self.nodes[n_idx].layer_neighbors[layer].push(node_idx);
                    if self.nodes[n_idx].layer_neighbors[layer].len() > self.m {
                        self.nodes[n_idx].layer_neighbors[layer].truncate(self.m);
                    }
                }
            }

            if let Some(first) = candidates.first() {
                curr_entry = first.idx;
            }
        }

        self.nodes.push(node);

        if target_layer > self.max_layer {
            self.max_layer = target_layer;
            self.enter_point = Some(node_idx);
        }
    }

    pub fn knn_search(&self, query_vec: &[f32], k: usize) -> Vec<SearchResult> {
        if self.nodes.is_empty() {
            return Vec::new();
        }

        let mut curr_entry = self.enter_point.unwrap();

        // 1. Traverse top layers greedily
        for layer in (1..=self.max_layer).rev() {
            curr_entry = self.greedy_closest(query_vec, curr_entry, layer);
        }

        // 2. Beam search on Layer 0 with ef = max(k, 32)
        let ef = k.max(32);
        let candidates = self.search_layer(query_vec, curr_entry, ef, 0);

        candidates
            .into_iter()
            .take(k)
            .map(|c| {
                let node = &self.nodes[c.idx];
                SearchResult {
                    id: node.id.clone(),
                    title: node.title.clone(),
                    preview: node.preview.clone(),
                    similarity: (1.0 - c.dist).max(0.0),
                }
            })
            .collect()
    }

    fn assign_layer(&self, vec: &[f32]) -> usize {
        // Geometric hash distribution
        let hash: u64 = vec.iter().enumerate().fold(14695981039346656037, |acc, (i, &v)| {
            acc ^ ((v.to_bits() as u64) << (i % 32))
        });
        let val = (hash % 100) as f32 / 100.0;
        let l = (-val.ln() * 0.7) as usize;
        l.min(4) // Max 4 layers
    }

    fn distance(&self, a: &[f32], b: &[f32]) -> f32 {
        let sim = EdgeSlmEngine::cosine_similarity(a, b);
        (1.0 - sim).max(0.0)
    }

    fn greedy_closest(&self, query: &[f32], mut curr: usize, layer: usize) -> usize {
        let mut curr_dist = self.distance(query, &self.nodes[curr].vector);
        loop {
            let mut changed = false;
            if self.nodes[curr].layer_neighbors.len() > layer {
                for &neighbor in &self.nodes[curr].layer_neighbors[layer] {
                    let d = self.distance(query, &self.nodes[neighbor].vector);
                    if d < curr_dist {
                        curr_dist = d;
                        curr = neighbor;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        curr
    }

    fn search_layer(&self, query: &[f32], entry: usize, ef: usize, layer: usize) -> Vec<Candidate> {
        let mut visited = HashSet::new();
        let mut candidates = BinaryHeap::new();
        let mut results = BinaryHeap::new();

        let initial_dist = self.distance(query, &self.nodes[entry].vector);
        visited.insert(entry);
        candidates.push(Candidate { dist: initial_dist, idx: entry });
        results.push(Candidate { dist: -initial_dist, idx: entry }); // Max-heap for results

        while let Some(current) = candidates.pop() {
            let furthest_result_dist = -results.peek().map(|c| c.dist).unwrap_or(f32::NEG_INFINITY);
            if current.dist > furthest_result_dist && results.len() >= ef {
                break;
            }

            if self.nodes[current.idx].layer_neighbors.len() > layer {
                for &neighbor in &self.nodes[current.idx].layer_neighbors[layer] {
                    if visited.insert(neighbor) {
                        let d = self.distance(query, &self.nodes[neighbor].vector);
                        let furthest_dist = -results.peek().map(|c| c.dist).unwrap_or(f32::INFINITY);

                        if d < furthest_dist || results.len() < ef {
                            candidates.push(Candidate { dist: d, idx: neighbor });
                            results.push(Candidate { dist: -d, idx: neighbor });
                            if results.len() > ef {
                                results.pop();
                            }
                        }
                    }
                }
            }
        }

        let mut res: Vec<Candidate> = results
            .into_iter()
            .map(|c| Candidate { dist: -c.dist, idx: c.idx })
            .collect();
        res.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap_or(Ordering::Equal));
        res
    }

    pub fn index_vault(&mut self, vault_path: &Path, max_notes: usize) -> usize {
        let mut indexed = 0;
        for entry in WalkDir::new(vault_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            if indexed >= max_notes {
                break;
            }

            let path = entry.path();
            let title = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Untitled").to_string();
            
            // Fast read first 250 characters for embedding preview
            if let Ok(content) = std::fs::read_to_string(path) {
                let preview = content.chars().take(200).collect::<String>().replace('\n', " ");
                let text_to_embed = format!("{title} {preview}");
                let vector = EdgeSlmEngine::embed_text(&text_to_embed);
                let id = path.strip_prefix(vault_path).unwrap_or(path).to_string_lossy().to_string();

                self.insert(id, title, preview, vector);
                indexed += 1;
            }
        }
        indexed
    }
}
