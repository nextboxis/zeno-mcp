// src/agents/final_checker.rs
// Autonomous Human-Brain Agile DevOps Hacker Supervisor
// Enforces Definition of Done, CI/CD gates, and adversarial attack scrutiny.

use crate::client::AntigravityClient;
use anyhow::Result;
use colored::*;

pub const FINAL_CHECKER_SYSTEM_INSTRUCTION: &str = r#"
You are the SUPREME FINAL CHECK SUPERVISOR.
Your cognitive architecture emulates an expert human brain (Prefrontal Cortex Executive Control, 
Working Memory Buffer, and Adversarial Red-Team Critic) working in an Agile/DevOps environment.

YOUR MISSION:
Perform a ruthless, adversarial, and uncompromising audit of any code, note, or security report 
produced by Agent 1 (Cyber Warfare), Agent 2 (Vault Architect), or Agent 3 (Rust Engineer).

THE 4 COGNITIVE QUALITY GATES:
1. PREFRONTAL GOAL ARBITRATION: Does the deliverable solve the user's EXACT core request without 
   scope creep, hallucinated references, or incomplete stubs?
2. AGILE DEFINITION OF DONE (DoD):
   - 100% complete and self-contained (Zero placeholders, zero TODOs).
   - Fully backward-compatible.
   - Comprehensive test cases / verification criteria documented.
3. DEVOPS CI/CD INTEGRITY:
   - Zero unhandled panics, zero unwrap() in production paths.
   - Zero unpinned external dependencies.
   - Zero credential / PII leakage.
4. HACKER RED-TEAM SCRUTINY (DEVIL'S ADVOCATE ATTACK):
   - Actively attempt to bypass or break the output.
   - Check edge cases: empty strings, integer overflows, concurrent race conditions, injection payloads.
   - Check graph integrity: broken wikilinks, ghost nodes, dangling references.

DECISION PROTOCOL:
You must issue either:
[STATUS: APPROVED] with an executive cryptographic-style Sign-off Ledger.
OR
[STATUS: REJECTED] with a prioritized list of Defect Attack Vectors and concrete code remedies.
"#;

pub struct FinalCheckSupervisor {
    client: AntigravityClient,
}

impl FinalCheckSupervisor {
    pub fn new(client: AntigravityClient) -> Self {
        Self { client }
    }

    pub async fn review_deliverable(
        &self,
        origin_agent: &str,
        original_goal: &str,
        deliverable: &str,
    ) -> Result<String> {
        let review_prompt = format!(
            "### AUDIT TARGET FROM: {origin_agent}\n\n\
             ### ORIGINAL USER GOAL:\n{original_goal}\n\n\
             ### DELIVERABLE TO AUDIT:\n{deliverable}\n\n\
             Execute the 4 Cognitive Quality Gates (Prefrontal Goal Check, Agile DoD, DevOps Integrity, and Hacker Red-Team Attack). \
             Issue your final verdict with [STATUS: APPROVED] or [STATUS: REJECTED]."
        );

        println!("{}", "\n========================================================".cyan());
        println!("{}", "🧠 FINAL CHECK SUPERVISOR: INITIATING COGNITIVE AUDIT".cyan().bold());
        println!("   Origin Agent: {}", origin_agent.yellow());
        println!("   Quality Gates: Prefrontal Brain • Agile DoD • DevOps • Hacker Red-Team");
        println!("{}", "========================================================\n".cyan());

        let review_result = self
            .client
            .chat(FINAL_CHECKER_SYSTEM_INSTRUCTION, &review_prompt)
            .await?;

        if review_result.contains("[STATUS: APPROVED]") {
            println!("{}", "✅ [GATE PASSED]: Deliverable certified by Human-Brain Agile DevOps Hacker Supervisor!".green().bold());
        } else {
            println!("{}", "❌ [GATE REJECTED]: Defects uncovered during Hacker Red-Team Probe!".red().bold());
        }

        Ok(review_result)
    }
}
