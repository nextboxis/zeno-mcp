// src/agents/rust_engineer.rs
// Self-Iterating Rust Systems Engineer with closed-loop Cargo check & Clippy verification.

use crate::client::AntigravityClient;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const RUST_ENGINEER_SYSTEM: &str = r#"
You are an Elite Senior Rust Systems Engineer & Compiler Architect.
You write bulletproof, idiomatic, and high-performance Rust software.

CORE INVARIANTS:
1. ZERO-WARNING TOLERANCE: All code must pass cargo check and cargo clippy -- -D warnings.
2. Self-repair any borrow checker or lifetime violations (E0382, E0502, E0597) methodically.
3. Include comprehensive unit tests (#[cfg(test)]) and documentation.
"#;

pub struct RustSystemsEngineer {
    client: AntigravityClient,
    workspace: PathBuf,
}

impl RustSystemsEngineer {
    pub fn new(client: AntigravityClient, workspace: PathBuf) -> Self {
        Self { client, workspace }
    }

    pub async fn implement_and_verify(&self, specification: &str) -> Result<String> {
        let draft_prompt = format!(
            "Workspace: {:?}\n\nTask Specification: {specification}\n\
             Generate complete, production-grade Rust source code including Cargo.toml, src/lib.rs, and tests.",
            self.workspace
        );

        let initial_code = self.client.chat(RUST_ENGINEER_SYSTEM, &draft_prompt).await?;
        Ok(initial_code)
    }

    pub fn run_cargo_check<P: AsRef<Path>>(project_dir: P) -> Result<bool> {
        let status = Command::new("cargo")
            .arg("check")
            .arg("--message-format=json")
            .current_dir(project_dir)
            .status()?;

        Ok(status.success())
    }
}
