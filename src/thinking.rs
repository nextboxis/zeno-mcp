// src/thinking.rs
// Configures deep chain-of-thought extended thinking budgets for Gemini models.

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ThinkingConfig {
    pub thinking_budget_tokens: u32,
    pub include_thoughts_in_response: bool,
}

impl ThinkingConfig {
    pub fn complex_reasoning() -> Self {
        Self {
            thinking_budget_tokens: 16384, // Deep multi-step architectural planning
            include_thoughts_in_response: true,
        }
    }

    pub fn fast_execution() -> Self {
        Self {
            thinking_budget_tokens: 2048,  // Low-latency tool dispatch
            include_thoughts_in_response: false,
        }
    }
}
