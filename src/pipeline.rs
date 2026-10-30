// src/pipeline.rs
// Fluid, end-to-end reactive orchestrator connecting user goals to smooth multi-agent execution.

use crate::agents::cyber_operator::CyberOperatorAgent;
use crate::agents::final_checker::FinalCheckSupervisor;
use crate::agents::rust_engineer::RustSystemsEngineer;
use crate::agents::self_improver::SelfImprovementAgent;
use crate::agents::vault_architect::VaultArchitectAgent;
use crate::client::AntigravityClient;
use crate::kanban::{AgileKanbanBoard, KanbanStage};
use crate::skills::SkillsRegistry;
use crate::slop_filter::SlopFilter;
use anyhow::Result;
use colored::*;
use std::path::PathBuf;

pub struct FluidPipeline {
    client: AntigravityClient,
    board: AgileKanbanBoard,
    slop_filter: SlopFilter,
    skills_path: PathBuf,
    vault_path: PathBuf,
    workspace_path: PathBuf,
}

impl FluidPipeline {
    pub fn new(api_key: String, obsidian_kanban_path: Option<PathBuf>) -> Self {
        let client = AntigravityClient::new(api_key, None);
        let board = AgileKanbanBoard::new(obsidian_kanban_path);
        let slop_filter = SlopFilter::new();
        let home_dir = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .unwrap_or_else(|_| ".".to_string());

        Self {
            client,
            board,
            slop_filter,
            skills_path: std::env::var("ZENO_SKILLS_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from(&home_dir).join(".zeno").join("skills")),
            vault_path: std::env::var("ZENO_VAULT_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from(&home_dir).join(".zeno").join("vault")),
            workspace_path: std::env::var("ZENO_WORKSPACE_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from(&home_dir).join(".zeno").join("workspace")),
        }
    }

    pub async fn execute_sprint(&mut self, agent_type: &str, raw_user_goal: &str) -> Result<()> {
        println!("{}", "\n🚀 INITIATING FLUID AGILE SPRINT PIPELINE".green().bold());

        // Step 1: Slop Purifier cleans input
        let purified_goal = self.slop_filter.purify(raw_user_goal);

        // Step 2: Backlog Story Creation
        let card_id = self.board.add_story(&purified_goal, agent_type);
        self.board.transition(&card_id, KanbanStage::SprintBacklog)?;

        // Step 3: Sprint Commitment & Active Execution
        self.board.transition(&card_id, KanbanStage::InProgress)?;

        let raw_deliverable = match agent_type {
            "cyber" => {
                let registry = SkillsRegistry::scan_directory(&self.skills_path).unwrap_or_else(|_| {
                    SkillsRegistry { skills: std::collections::HashMap::new() }
                });
                let agent = CyberOperatorAgent::new(self.client.clone(), registry);
                agent.execute_assessment(&purified_goal).await?
            }
            "vault" => {
                let agent = VaultArchitectAgent::new(self.client.clone(), self.vault_path.clone());
                agent.curate_vault(&purified_goal).await?
            }
            "rust" => {
                let agent = RustSystemsEngineer::new(self.client.clone(), self.workspace_path.clone());
                agent.implement_and_verify(&purified_goal).await?
            }
            _ => anyhow::bail!("Invalid agent: {agent_type}"),
        };

        // Purify output deliverable
        let deliverable = self.slop_filter.purify(&raw_deliverable);

        // Step 4: Hacker Review & Final Check Gate
        self.board.transition(&card_id, KanbanStage::HackerReview)?;
        let final_checker = FinalCheckSupervisor::new(self.client.clone());
        let audit_verdict = final_checker
            .review_deliverable(agent_type, &purified_goal, &deliverable)
            .await?;

        // Step 5: DevOps Gate & DoD Verification
        if audit_verdict.contains("[STATUS: APPROVED]") {
            self.board.transition(&card_id, KanbanStage::DevOpsGate)?;
            self.board.transition(&card_id, KanbanStage::Done)?;
        } else {
            self.board.record_defect(&card_id)?;
            println!("{}", "⚠️  Defect detected: Recycling deliverable for repair!".yellow());
        }

        // Step 6: Autonomous Retrospective & Self-Improvement
        let self_improver = SelfImprovementAgent::new(self.client.clone());
        let retro = self_improver
            .run_sprint_retrospective(&self.board, &purified_goal, &audit_verdict)
            .await?;

        println!("\n{}", "=========================================================".cyan());
        println!("{}", "               🎉 SPRINT SUMMARY & DELIVERABLE           ".cyan().bold());
        println!("{}", "=========================================================".cyan());
        println!("{}\n", deliverable);

        println!("{}", "=========================================================".magenta());
        println!("{}", "           🔄 SPRINT RETROSPECTIVE & OPTIMIZATIONS       ".magenta().bold());
        println!("{}", "=========================================================".magenta());
        println!("{}\n", retro);

        self.board.render_terminal();
        Ok(())
    }
}
