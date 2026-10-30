// src/agents/self_improver.rs
// Autonomous Self-Improvement & Retrospective Agent in Pure Native Rust.

use crate::client::AntigravityClient;
use crate::kanban::AgileKanbanBoard;
use anyhow::Result;
use colored::*;

pub const SELF_IMPROVER_SYSTEM: &str = r#"
You are the Metacognitive Autonomous Self-Improvement & Sprint Retrospective Agent.
Your mandate is recursive self-optimization: analyzing agent performance, execution bottlenecks, 
compiler repair iterations, and rejection logs to optimize agent prompts, synthesize new skills, 
and prevent future errors.

CORE RETROSPECTIVE WORKFLOW:
1. WHAT WENT WELL: Highlighting zero-error runs, sub-2ms lookups, and fast DoD passes.
2. WHAT FAILED / TOOK EXTRA EFFORT: Review compiler repair attempts, borrow checker violations, or hacker probe rejections.
3. ADAPTIVE OPTIMIZATIONS:
   - Provide concrete prompt modifications for Agent 1, 2, or 3.
   - Outline any newly required modular SKILL.md standards to fill capability gaps.
   - Formulate actionable retrospective insights for the next Agile Sprint.
"#;

pub struct SelfImprovementAgent {
    client: AntigravityClient,
}

impl SelfImprovementAgent {
    pub fn new(client: AntigravityClient) -> Self {
        Self { client }
    }

    pub async fn run_sprint_retrospective(
        &self,
        board: &AgileKanbanBoard,
        sprint_task: &str,
        audit_verdict: &str,
    ) -> Result<String> {
        let total_stories = board.cards.len();
        let total_defects: usize = board.cards.iter().map(|c| c.defect_count).sum();

        let prompt = format!(
            "### SPRINT RETROSPECTIVE AUDIT\n\n\
             Task Executed: {sprint_task}\n\
             Total Stories Managed: {total_stories}\n\
             Total Defect / Rejection Loops: {total_defects}\n\
             Final Check Supervisor Verdict:\n{audit_verdict}\n\n\
             Formulate an executive Agile Sprint Retrospective with adaptive prompt tuning \
             and future skill generation recommendations."
        );

        println!("{}", "\n========================================================".magenta());
        println!("{}", "🔄 SELF-IMPROVEMENT AGENT: EXECUTING SPRINT RETROSPECTIVE".magenta().bold());
        println!("   Metacognitive Analysis: Analyzing failure modes & prompt tuning...");
        println!("{}", "========================================================\n".magenta());

        let report = self.client.chat(SELF_IMPROVER_SYSTEM, &prompt).await?;
        println!("{}", "✨ [SELF-IMPROVEMENT COMPLETE]: Retrospective insights synthesized!".green().bold());
        Ok(report)
    }
}
