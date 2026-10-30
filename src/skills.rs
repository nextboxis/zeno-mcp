// src/skills.rs
// Fast native filesystem walker indexing modular SKILL.md standards with YAML frontmatter.

use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SkillDefinition {
    pub name: String,
    pub description: String,
    pub body: String,
    pub path: PathBuf,
}

pub struct SkillsRegistry {
    pub skills: HashMap<String, SkillDefinition>,
}

impl SkillsRegistry {
    pub fn scan_directory<P: AsRef<Path>>(root: P) -> Result<Self> {
        let mut skills = HashMap::new();

        if !root.as_ref().exists() {
            return Ok(Self { skills });
        }

        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() && entry.file_name() == "SKILL.md" {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Some((frontmatter, body)) = parse_frontmatter(&content) {
                        if let Ok(meta) = serde_yaml::from_str::<SkillMetadata>(&frontmatter) {
                            let def = SkillDefinition {
                                name: meta.name.clone(),
                                description: meta.description.unwrap_or_default(),
                                body: body.to_string(),
                                path: entry.path().to_path_buf(),
                            };
                            skills.insert(meta.name, def);
                        }
                    }
                }
            }
        }

        Ok(Self { skills })
    }

    pub fn get_skill_instructions(&self, skill_name: &str) -> Option<&str> {
        self.skills.get(skill_name).map(|s| s.body.as_str())
    }
}

fn parse_frontmatter(content: &str) -> Option<(String, String)> {
    let mut parts = content.splitn(3, "---");
    let _empty = parts.next()?;
    let frontmatter = parts.next()?;
    let body = parts.next().unwrap_or("");
    Some((frontmatter.trim().to_string(), body.trim().to_string()))
}
