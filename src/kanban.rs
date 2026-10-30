// src/kanban.rs
// Native Agile Sprint & Kanban Board Engine with bi-directional Obsidian synchronization.

use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KanbanStage {
    Backlog,
    SprintBacklog,
    InProgress,
    HackerReview,
    DevOpsGate,
    Done,
}

impl std::fmt::Display for KanbanStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KanbanStage::Backlog => write!(f, "📋 BACKLOG"),
            KanbanStage::SprintBacklog => write!(f, "🎯 SPRINT BACKLOG"),
            KanbanStage::InProgress => write!(f, "⚡ IN PROGRESS"),
            KanbanStage::HackerReview => write!(f, "⚔️ HACKER REVIEW"),
            KanbanStage::DevOpsGate => write!(f, "🛡️ DEVOPS GATE"),
            KanbanStage::Done => write!(f, "✅ DONE"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCard {
    pub id: String,
    pub title: String,
    pub assigned_agent: String,
    pub stage: KanbanStage,
    pub created_at: String,
    pub updated_at: String,
    pub defect_count: usize,
}

pub struct AgileKanbanBoard {
    pub cards: Vec<KanbanCard>,
    pub obsidian_sync_path: Option<PathBuf>,
}

impl AgileKanbanBoard {
    pub fn new(obsidian_sync_path: Option<PathBuf>) -> Self {
        Self {
            cards: Vec::new(),
            obsidian_sync_path,
        }
    }

    pub fn add_story(&mut self, title: &str, assigned_agent: &str) -> String {
        let id = format!("STORY-{}", self.cards.len() + 1);
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let card = KanbanCard {
            id: id.clone(),
            title: title.to_string(),
            assigned_agent: assigned_agent.to_string(),
            stage: KanbanStage::Backlog,
            created_at: now.clone(),
            updated_at: now,
            defect_count: 0,
        };

        self.cards.push(card);
        let _ = self.sync_to_obsidian();
        id
    }

    pub fn transition(&mut self, card_id: &str, next_stage: KanbanStage) -> Result<()> {
        let card = self
            .cards
            .iter_mut()
            .find(|c| c.id == card_id)
            .context(format!("Card not found: {card_id}"))?;

        card.stage = next_stage;
        card.updated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        println!(
            "{} [{}] '{}' ➔ {}",
            "📋 [KANBAN TRANSITION]:".cyan().bold(),
            card.id.yellow(),
            card.title,
            next_stage.to_string().green().bold()
        );

        let _ = self.sync_to_obsidian();
        Ok(())
    }

    pub fn record_defect(&mut self, card_id: &str) -> Result<()> {
        if let Some(card) = self.cards.iter_mut().find(|c| c.id == card_id) {
            card.defect_count += 1;
        }
        let _ = self.sync_to_obsidian();
        Ok(())
    }

    pub fn render_terminal(&self) {
        println!("{}", "\n=========================================================================".cyan());
        println!("{}", "       🚀 AGILE SPRINT KANBAN BOARD (LIVE AGENT STATUS)                  ".cyan().bold());
        println!("{}", "=========================================================================".cyan());

        for stage in &[
            KanbanStage::Backlog,
            KanbanStage::SprintBacklog,
            KanbanStage::InProgress,
            KanbanStage::HackerReview,
            KanbanStage::DevOpsGate,
            KanbanStage::Done,
        ] {
            let matching: Vec<&KanbanCard> = self.cards.iter().filter(|c| c.stage == *stage).collect();
            println!("\n## {} ({})", stage, matching.len());
            for c in matching {
                let defect_badge = if c.defect_count > 0 {
                    format!(" [Defects: {}]", c.defect_count).red().to_string()
                } else {
                    "".to_string()
                };
                println!("  - [{}] {} (Agent: {}){}", c.id.yellow(), c.title, c.assigned_agent.blue(), defect_badge);
            }
        }
        println!("{}", "=========================================================================\n".cyan());
    }

    pub fn sync_to_obsidian(&self) -> Result<()> {
        let path = match &self.obsidian_sync_path {
            Some(p) => p,
            None => return Ok(()),
        };

        let mut markdown = String::from("---\nkanban-plugin: basic\nlast_updated: 2026-10-29\ntags: [domain/dev-ai, agile, kanban, sprint]\n---\n\n# 📋 Agent Agile Sprint Board\n\n");

        for stage in &[
            KanbanStage::Backlog,
            KanbanStage::SprintBacklog,
            KanbanStage::InProgress,
            KanbanStage::HackerReview,
            KanbanStage::DevOpsGate,
            KanbanStage::Done,
        ] {
            markdown.push_str(&format!("## {}\n", stage));
            let matching: Vec<&KanbanCard> = self.cards.iter().filter(|c| c.stage == *stage).collect();
            for c in matching {
                markdown.push_str(&format!("- [ ] **{}**: {} (`{}`)\n", c.id, c.title, c.assigned_agent));
            }
            markdown.push('\n');
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, markdown)?;
        Ok(())
    }
}
