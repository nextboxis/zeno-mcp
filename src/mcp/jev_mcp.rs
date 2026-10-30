// src/mcp/jev_mcp.rs
// Jev-Powered Calibrated MCP Router in Pure Native Rust.
// Implements Victor Dibia's Jev architecture directly for Model Context Protocol dispatching.

use crate::jev_engine::{JevDecisionEngine, JevMultiHeadDecision};
use colored::*;

pub struct JevMcpRouter {
    engine: JevDecisionEngine,
    registered_tools: Vec<String>,
}

impl Default for JevMcpRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl JevMcpRouter {
    pub fn new() -> Self {
        let tools = vec![
            "cyber_operator".to_string(),
            "vault_architect".to_string(),
            "rust_engineer".to_string(),
            "headless_browser_cdp".to_string(),
            "hnsw_semantic_search".to_string(),
            "agile_sprint_coordinator".to_string(),
        ];

        Self {
            engine: JevDecisionEngine::new(0.95, 0.85),
            registered_tools: tools,
        }
    }

    /// Dispatches an incoming MCP request using Jev's 4-step calibrated decision model:
    /// State -> Sentence Completion -> Parallel Scoring -> Temperature-Scaled Normalization.
    pub fn route_mcp_request(&self, user_state: &str) -> JevMultiHeadDecision {
        self.engine.evaluate_mcp_request(user_state, &self.registered_tools)
    }

    /// Formats an executive Jev MCP telemetry audit report
    pub fn format_decision_report(&self, decision: &JevMultiHeadDecision, input_state: &str) -> String {
        // Phase 1: injection_verdict is set by the SemanticInjectionClassifier (contains icon)
        let injection_badge = if decision.is_prompt_injection {
            format!("{} (BLOCKED)", decision.injection_verdict).red().bold().to_string()
        } else {
            format!("{} (PASSED)", decision.injection_verdict).green().bold().to_string()
        };

        let automation_badge = if decision.can_automate {
            "✅ CONFIDENCE CLEARED (Automated Execution)".green().bold().to_string()
        } else {
            "⚠️ LOW CONFIDENCE (<85%) -> ESCALATE TO HUMAN / SYSTEM 2".yellow().bold().to_string()
        };

        format!(
            "\n========================================================================================\n\
             🧠 JEV CALIBRATED MCP ROUTER TELEMETRY (SYSTEM ONE DECISION)\n\
             ========================================================================================\n\
             - **Input State**: \"{}\"\n\
             - **Target Tool Selected (Choice)**: `{}` (Confidence: {:.1}%)\n\
             - **Prompt Injection Guardrail (Noul)**: {} (P_inj: {:.2}%)\n\
             - **Task Complexity & Risk (Score)**: Level {} - `{}`\n\
             - **Decision Latency**: `{:.2} ms` on native CPU\n\
             - **Output Tokens Generated**: `{}` tokens (Zero Generation Paradigm)\n\
             - **Expected Calibration Error (ECE)**: `{:.4}`\n\
             - **Autonomous Clearance**: {}\n\
             ========================================================================================\n",
            input_state,
            decision.target_tool,
            decision.tool_confidence * 100.0,
            injection_badge,
            decision.injection_probability * 100.0,
            decision.risk_rating,
            decision.complexity_level,
            decision.latency_ms,
            decision.output_tokens_generated,
            decision.expected_calibration_error,
            automation_badge
        )
    }
}
