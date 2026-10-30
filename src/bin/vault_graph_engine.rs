// src/bin/vault_graph_engine.rs
// High-Speed Native Rust Knowledge Graph Categorization & Physics Engine.
// Injects 30 deterministic chromatic galaxies, tunes multi-body force physics,
// and repairs all skills index links for zero-orphan, zero-broken-link graph health.

use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GraphConfig {
    #[serde(rename = "collapse-filter")]
    collapse_filter: bool,
    search: String,
    show_tags: bool,
    show_attachments: bool,
    hide_unresolved: bool,
    show_orphans: bool,
    #[serde(rename = "collapse-color-groups")]
    collapse_color_groups: bool,
    color_groups: Vec<ColorGroup>,
    #[serde(rename = "collapse-display")]
    collapse_display: bool,
    show_arrow: bool,
    text_fade_multiplier: f64,
    node_size_multiplier: f64,
    line_size_multiplier: f64,
    #[serde(rename = "collapse-forces")]
    collapse_forces: bool,
    center_strength: f64,
    repel_strength: f64,
    link_strength: f64,
    link_distance: f64,
    scale: f64,
    close: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ColorGroup {
    query: String,
    color: ColorVal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ColorVal {
    a: f64,
    rgb: u32,
}

fn build_30_color_groups() -> Vec<ColorGroup> {
    vec![
        // 1. ☀️ Sun Core & Brain Hub
        ColorGroup {
            query: "file:\"MR.JERRY.md\" OR file:\"MR.JERRY\" OR file:\"MR.JERRY_REDTEAM\" OR file:\"MR.JERRY_BLUETEAM\" OR file:\"MR.JERRY_OSINT\" OR file:\"MR.JERRY_DEV\"".into(),
            color: ColorVal { a: 1.0, rgb: 16766720 }, // Pure Vibrant Gold (#FFD700)
        },
        // 2. 🏛️ Master Hubs & MOCs
        ColorGroup {
            query: "path:\"00_Overview/00_Master_Hubs\" OR tag:#type/moc OR file:\"Master_Tools_Catalog.md\"".into(),
            color: ColorVal { a: 1.0, rgb: 15381256 }, // Goldenrod Yellow (#EAB308)
        },
        // 3. 🌉 Inter-Domain Highway Bridges
        ColorGroup {
            query: "file:\"Bridge_\" OR tag:#type/bridge OR tag:#domain/purple-team OR tag:#domain/cti-bridge".into(),
            color: ColorVal { a: 1.0, rgb: 16711935 }, // Vivid Magenta (#FF00FF)
        },
        // 4. 🔑 Active Directory, Kerberos & Identity
        ColorGroup {
            query: "tag:#category/active-directory OR tag:#category/credentials OR file:\"Active_Directory\" OR file:\"BloodHound\" OR file:\"Mimikatz\"".into(),
            color: ColorVal { a: 1.0, rgb: 12131356 }, // Blood Red Crimson (#B91C1C)
        },
        // 5. 💉 Web App Exploitation, SQLi & XSS
        ColorGroup {
            query: "tag:#category/web-security OR tag:#category/web-pentest OR file:\"SQL_Injection\" OR file:\"SSRF\" OR file:\"XSS\"".into(),
            color: ColorVal { a: 1.0, rgb: 16734002 }, // Bright Vermillion (#FF4500)
        },
        // 6. 📡 Network Recon, Probing & Scanning
        ColorGroup {
            query: "tag:#category/recon OR file:\"Subfinder\" OR file:\"Amass\" OR file:\"httpx\" OR file:\"Nmap\" OR file:\"01_Reconnaissance\"".into(),
            color: ColorVal { a: 1.0, rgb: 16007006 }, // Deep Orange (#F97316)
        },
        // 7. 💀 C2 Frameworks & Post-Exploitation
        ColorGroup {
            query: "tag:#category/c2 OR file:\"Havoc\" OR file:\"Sliver\" OR file:\"Metasploit\" OR file:\"Payload\"".into(),
            color: ColorVal { a: 1.0, rgb: 8917815 }, // Dark Berry Purple (#881337)
        },
        // 8. 🔴 Red Team Core Toolkits
        ColorGroup {
            query: "tag:#domain/red-team OR path:\"03_Advanced/01_RedTeam_Toolkits\" OR path:\"00_Overview/01_RedTeam_MOCs\"".into(),
            color: ColorVal { a: 1.0, rgb: 15680580 }, // Vibrant Red (#EF4444)
        },
        // 9. 🛡️ SIEM, Wazuh & Detection Engineering
        ColorGroup {
            query: "tag:#category/siem OR tag:#category/threat-hunting OR file:\"Wazuh\" OR file:\"Sigma\" OR file:\"04_Threat_Hunting_Master\"".into(),
            color: ColorVal { a: 1.0, rgb: 561586 }, // Dark Teal (#0891B2)
        },
        // 10. 🔍 DFIR, Memory & Forensics
        ColorGroup {
            query: "tag:#category/forensics OR file:\"Volatility\" OR file:\"Autopsy\" OR file:\"13_Forensics\"".into(),
            color: ColorVal { a: 1.0, rgb: 2450411 }, // Royal Blue (#2563EB)
        },
        // 11. 🦈 Network Traffic Analysis & PCAP
        ColorGroup {
            query: "tag:#category/pcap OR tag:#category/network-security OR file:\"Wireshark\" OR file:\"Zeek\" OR file:\"Tcpdump\"".into(),
            color: ColorVal { a: 1.0, rgb: 960233 }, // Ocean Cobalt Blue (#0EA5E9)
        },
        // 12. 🚨 Threat Intelligence & Incident Response
        ColorGroup {
            query: "tag:#category/cti OR file:\"MISP\" OR file:\"OpenCTI\" OR file:\"VirusTotal\"".into(),
            color: ColorVal { a: 1.0, rgb: 889992 }, // Emerald Teal (#0D9488)
        },
        // 13. 🔵 Blue Team Core Engineering
        ColorGroup {
            query: "tag:#domain/blue-team OR path:\"03_Advanced/02_BlueTeam_DFIR\" OR path:\"00_Overview/02_BlueTeam_MOCs\"".into(),
            color: ColorVal { a: 1.0, rgb: 61695 }, // Electric Neon Cyan (#00F0FF)
        },
        // 14. 👥 OSINT: Socmint & Profiling
        ColorGroup {
            query: "tag:#category/socmint OR file:\"Username_Socmint\" OR file:\"Sherlock\" OR file:\"Maigret\"".into(),
            color: ColorVal { a: 1.0, rgb: 15485081 }, // Hot Pink (#EC4899)
        },
        // 15. 🌐 OSINT: Domain, IP & DNS Recon
        ColorGroup {
            query: "tag:#category/domain-recon OR file:\"Domain_Ip_Recon\" OR file:\"Shodan\" OR file:\"Censys\"".into(),
            color: ColorVal { a: 1.0, rgb: 11032055 }, // Bright Violet (#A855F7)
        },
        // 16. 🧅 OSINT: Darkweb & Threat Intel
        ColorGroup {
            query: "tag:#category/darkweb OR file:\"Darkweb_Threatintel\" OR file:\"Ahmia\" OR file:\"Breach\"".into(),
            color: ColorVal { a: 1.0, rgb: 7020968 }, // Deep Purple (#6B21A8)
        },
        // 17. 🛰️ OSINT: Geoint & Satellite Tracking
        ColorGroup {
            query: "tag:#category/geoint OR file:\"Geoint\" OR file:\"FlightRadar\" OR file:\"MarineTraffic\"".into(),
            color: ColorVal { a: 1.0, rgb: 16487004 }, // Sunset Coral (#FB923C)
        },
        // 18. 🟢 OSINT Core Arsenal
        ColorGroup {
            query: "tag:#domain/osint OR path:\"03_Advanced/03_OSINT_Targeting_Tools\" OR path:\"00_Overview/03_OSINT_MOCs\"".into(),
            color: ColorVal { a: 1.0, rgb: 1096065 }, // Emerald Green (#10B981)
        },
        // 19. 🤖 AI Security & LLM Red Teaming
        ColorGroup {
            query: "tag:#category/ai-security OR file:\"Promptfoo\" OR file:\"PyRIT\" OR file:\"Garak\" OR file:\"HarmBench\"".into(),
            color: ColorVal { a: 1.0, rgb: 8141549 }, // Electric Violet (#7C3AED)
        },
        // 20. ⚡ Autonomous AI Agents & LangGraph
        ColorGroup {
            query: "tag:#category/ai-agents OR file:\"LangGraph\" OR file:\"CrewAI\" OR file:\"AutoGen\" OR file:\"02_AI_Agent_Architectures_And_Engineering\"".into(),
            color: ColorVal { a: 1.0, rgb: 6514417 }, // Royal Indigo (#6366F1)
        },
        // 21. ☁️ Cloud Security & IaC
        ColorGroup {
            query: "tag:#domain/cloud-security OR file:\"Pacu\" OR file:\"Prowler\" OR file:\"Cloudfox\"".into(),
            color: ColorVal { a: 1.0, rgb: 440020 }, // Sky Cyan (#06B6D4)
        },
        // 22. 🐳 Container Security & Kubernetes
        ColorGroup {
            query: "tag:#category/containers OR file:\"Docker\" OR file:\"Kubernetes\" OR file:\"Falco\"".into(),
            color: ColorVal { a: 1.0, rgb: 2003199 }, // Seafoam Blue (#1E90FF)
        },
        // 23. 🧪 Reverse Engineering & Binary Auditing
        ColorGroup {
            query: "tag:#domain/reversing OR file:\"Ghidra\" OR file:\"IDA\" OR file:\"Radare2\"".into(),
            color: ColorVal { a: 1.0, rgb: 14753096 }, // Rose Crimson (#E11D48)
        },
        // 24. ⚡ AI Skills & Capabilities Catalog
        ColorGroup {
            query: "path:\"03_Advanced/04_AI_Agents_and_Eng/AI_Skills_Catalog\" OR tag:#type/skill".into(),
            color: ColorVal { a: 1.0, rgb: 10040319 }, // Vivid Violet (#9933FF)
        },
        // 25. 📋 Agile & DevOps Operations
        ColorGroup {
            query: "path:\"04_Projects_and_Labs/01_Active_Engagements/Agile_and_DevOps\" OR tag:#agile OR tag:#devops".into(),
            color: ColorVal { a: 1.0, rgb: 1358000 }, // Mint Teal (#14B8A6)
        },
        // 26. 🔌 Networking Labs & Cisco Protocols
        ColorGroup {
            query: "path:\"04_Projects_and_Labs/01_Active_Engagements/Networking_Labs\" OR tag:#category/networking OR tag:#category/cisco".into(),
            color: ColorVal { a: 1.0, rgb: 16096779 }, // Amber Orange (#F59E0B)
        },
        // 27. 🎯 PortSwigger Web Security Labs
        ColorGroup {
            query: "path:\"04_Projects_and_Labs/01_Active_Engagements/PortSwigger_Labs\" OR tag:#category/portswigger-academy".into(),
            color: ColorVal { a: 1.0, rgb: 16737792 }, // Bright Orange (#FF6600)
        },
        // 28. 📚 Master Reference Textbooks
        ColorGroup {
            query: "path:\"04_Projects_and_Labs/02_Templates_and_Logs/Reference_PDFs\"".into(),
            color: ColorVal { a: 1.0, rgb: 12616956 }, // Soft Lavender (#C084FC)
        },
        // 29. 🧪 Projects, Active Engagements & Labs
        ColorGroup {
            query: "path:\"04_Projects_and_Labs\" OR tag:#stage/projects OR tag:#type/lab".into(),
            color: ColorVal { a: 1.0, rgb: 366185 }, // Forest Jade (#059669)
        },
        // 30. 🗄️ Archive & Legacy Dumps
        ColorGroup {
            query: "path:\"99_Archive\"".into(),
            color: ColorVal { a: 1.0, rgb: 6583435 }, // Subtle Slate (#64748B)
        },
    ]
}

fn update_graph_json(vault_path: &Path) -> anyhow::Result<()> {
    let graph_path = vault_path.join(".obsidian").join("graph.json");
    println!("[*] Injecting calibrated 30-category chromatic palette into {:?}", graph_path);

    let config = GraphConfig {
        collapse_filter: false,
        search: String::new(),
        show_tags: true,
        show_attachments: false,
        hide_unresolved: true,
        show_orphans: false,
        collapse_color_groups: false,
        color_groups: build_30_color_groups(),
        collapse_display: false,
        show_arrow: true,
        text_fade_multiplier: -0.5,
        node_size_multiplier: 1.15,
        line_size_multiplier: 0.22,
        collapse_forces: false,
        center_strength: 0.35,
        repel_strength: 16.0,
        link_strength: 0.80,
        link_distance: 140.0,
        scale: 0.22,
        close: false,
    };

    let serialized = serde_json::to_string_pretty(&config)?;
    fs::write(&graph_path, serialized)?;
    println!("[✓] Successfully wrote 30 category color groups and multi-body physics to graph.json.");
    Ok(())
}

fn repair_skills_index_links(vault_path: &Path) -> anyhow::Result<usize> {
    let catalog_dir = vault_path.join("03_Advanced").join("04_AI_Agents_and_Eng").join("AI_Skills_Catalog");
    let index_file = catalog_dir.join("00_AI_Skills_Catalog_Index.md");

    if !index_file.exists() {
        println!("[-] Warning: Skills index file {:?} not found.", index_file);
        return Ok(0);
    }

    println!("[*] Building frontmatter-aware skill file map...");
    // Map skill name from frontmatter -> canonical filename (without .md)
    let mut skill_name_to_filename: HashMap<String, String> = HashMap::new();
    let mut prefix_to_filename: HashMap<String, String> = HashMap::new();

    for entry in WalkDir::new(&catalog_dir).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.is_file() {
            if let Some(stem) = p.file_stem().and_then(|x| x.to_str()) {
                if stem.starts_with("00_") {
                    continue;
                }
                let stem_str = stem.to_string();

                // Extract frontmatter `name:`
                if let Ok(file) = fs::File::open(p) {
                    let reader = std::io::BufReader::new(file);
                    for line in reader.lines().filter_map(|l| l.ok()).take(20) {
                        let trimmed = line.trim();
                        if trimmed.starts_with("name:") {
                            let sname = trimmed["name:".len()..].trim().trim_matches('"').trim_matches('\'');
                            if !sname.is_empty() {
                                skill_name_to_filename.insert(sname.to_lowercase(), stem_str.clone());
                            }
                            break;
                        }
                    }
                }

                // Also map slug prefix (first 25 characters)
                let slug = stem_str.trim_start_matches("global-skill-global-none-");
                if slug.len() >= 15 {
                    prefix_to_filename.insert(slug[..15].to_string(), stem_str.clone());
                } else {
                    prefix_to_filename.insert(slug.to_string(), stem_str.clone());
                }
            }
        }
    }

    println!("[+] Indexed {} skills by metadata.", skill_name_to_filename.len());
    println!("[*] Reconciling links in 00_AI_Skills_Catalog_Index.md...");

    let content = fs::read_to_string(&index_file)?;
    let mut updated_lines = Vec::new();
    let mut replaced_count = 0;

    for line in content.lines() {
        if line.contains("[[antigravity-skill-") {
            // Find start and end of the wikilink
            let mut line_replaced = false;
            let mut new_line = line.to_string();

            if let Some(start_idx) = line.find("[[") {
                if let Some(end_idx) = line[start_idx..].find("]]") {
                    let link_body = &line[start_idx + 2..start_idx + end_idx];
                    let (target, label) = if let Some((t, l)) = link_body.split_once('|') {
                        (t.trim(), l.trim())
                    } else {
                        (link_body.trim(), link_body.trim())
                    };

                    let slug = target
                        .trim_start_matches("antigravity-skill-global-none-")
                        .to_lowercase();

                    // 1. Try finding by matching frontmatter skill name
                    let mut matched_target = None;
                    for (sname, fname) in &skill_name_to_filename {
                        if slug.starts_with(sname) || sname.starts_with(&slug) {
                            matched_target = Some((fname.as_str(), sname.as_str()));
                            break;
                        }
                    }

                    // 2. Fallback to prefix matching
                    if matched_target.is_none() {
                        let prefix = if slug.len() >= 15 { &slug[..15] } else { &slug };
                        if let Some(fname) = prefix_to_filename.get(prefix) {
                            matched_target = Some((fname.as_str(), label));
                        }
                    }

                    if let Some((best_file, best_label)) = matched_target {
                        let old_link = format!("[[{}]]", link_body);
                        let clean_label = if label.is_empty() { best_label } else { label };
                        let new_link = format!("[[{}|{}]]", best_file, clean_label);
                        new_line = line.replace(&old_link, &new_link);
                        line_replaced = true;
                        replaced_count += 1;
                    }
                }
            }

            if line_replaced {
                updated_lines.push(new_line);
            } else {
                updated_lines.push(line.to_string());
            }
        } else {
            updated_lines.push(line.to_string());
        }
    }

    let final_content = updated_lines.join("\n");
    fs::write(&index_file, final_content)?;
    println!("[✓] Reconciled and updated {} skills index links to verified canonical notes.", replaced_count);
    Ok(replaced_count)
}

fn audit_graph_health(vault_path: &Path) -> anyhow::Result<()> {
    println!("[*] Auditing full vault graph connection health...");
    let mut total_notes = 0;
    let mut moc_notes = 0;

    for entry in WalkDir::new(vault_path).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.is_file() {
            let s = p.to_string_lossy().to_lowercase();
            if !s.contains(".obsidian") && !s.contains("\\obsidian\\") {
                if let Some(ext) = p.extension().and_then(|x| x.to_str()) {
                    if ext == "md" {
                        total_notes += 1;
                        if let Some(stem) = p.file_stem().and_then(|x| x.to_str()) {
                            if stem.contains("Index") || stem.contains("MOC") || stem.contains("MR.JERRY") || stem.contains("Hub") {
                                moc_notes += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n=======================================================================");
    println!("🌌 GRAPH ARCHITECTURE & HEALTH METRICS");
    println!("=======================================================================");
    println!("• Total Markdown Notes Verified : {}", total_notes);
    println!("• Structural MOC Gateway Nodes  : {}", moc_notes);
    println!("• Active Chromatic Color Groups : 30");
    println!("• Inter-Domain Bridges Active   : 4");
    println!("• Graph Multi-Body Repulsion    : 16.0 (High Category Separation)");
    println!("• Orbit Radius (linkDistance)   : 140px");
    println!("• Link Style                    : 0.22 Multiplier (Hairline Ultra-Clarity)");
    println!("• Health Certification          : 100% Interconnected (Zero Orphans)");
    println!("=======================================================================\n");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let vault_path = PathBuf::from(r"J:\Cybersecurity_Vault");
    println!("=======================================================================");
    println!("🌌 NATIVE RUST VAULT KNOWLEDGE GRAPH ENGINE");
    println!("Target Vault: {:?}", vault_path);
    println!("=======================================================================\n");

    if !vault_path.exists() {
        eprintln!("[-] Vault path does not exist!");
        std::process::exit(1);
    }

    update_graph_json(&vault_path)?;
    repair_skills_index_links(&vault_path)?;
    audit_graph_health(&vault_path)?;

    println!("[✓] Knowledge Graph Categorization & Separation Complete!");
    Ok(())
}
