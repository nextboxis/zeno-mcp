// src/dsa/mod.rs
// Core Data Structures & Algorithms module root.

pub mod trie;
pub mod dag;
pub mod ring_buffer;
pub mod graph_walker;
pub mod hnsw;

pub use hnsw::{HnswIndex, SearchResult};
