// src/agents/vault_architect.rs
// Sub-millisecond Second-Brain Vault Architect in Pure Native Rust.

use crate::client::AntigravityClient;
use anyhow::Result;
use std::path::PathBuf;

pub const VAULT_ARCHITECT_SYSTEM: &str = r#"
You are the Principal Second-Brain Knowledge Architect governing an Obsidian vault (>5,300 notes).
You maintain graph topology, eliminate broken links, auto-generate dynamic MOCs, and ensure zero overload.
Follow the standards of ADHD_OCD_Calm_Graph_Guide and 4-Vector Tool Inter-Connection Architecture.
"#;

pub struct VaultArchitectAgent {
    client: AntigravityClient,
    vault_path: PathBuf,
}

impl VaultArchitectAgent {
    pub fn new(client: AntigravityClient, vault_path: PathBuf) -> Self {
        Self { client, vault_path }
    }

    pub async fn curate_vault(&self, curation_prompt: &str) -> Result<String> {
        let task = format!(
            "Target Vault Path: {:?}\n\nAction Request: {curation_prompt}",
            self.vault_path
        );

        self.client.chat(VAULT_ARCHITECT_SYSTEM, &task).await
    }
}
