pub mod code_mode;
pub mod harness;
pub mod injection_classifier;  // Phase 1: HNSW semantic injection classifier
pub mod jev_mcp;

pub use code_mode::CodeModeEngine;
pub use harness::{GeminiCachePinConfig, McpHarnessEngine, McpToolSearchResult, PreToolUseDecision};
pub use injection_classifier::{InjectionVerdict, SemanticInjectionClassifier};
pub use jev_mcp::JevMcpRouter;

