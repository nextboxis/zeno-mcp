// src/bin/vault_dedup.rs
// Blazing-fast Native Rust Deduplication Engine for Obsidian Second-Brain Vault.
// Identifies and permanently eliminates duplicate nodes, identical content, and redundant skill exports.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct NoteMeta {
    path: PathBuf,
    filename: String,
    size: u64,
    content: String,
    skill_name: Option<String>,
    is_missing: bool,
    real_path_exists: bool,
    tier_priority: u32, // Lower number = higher priority (00_Overview=1, 99_Archive=99)
}

impl NoteMeta {
    fn from_path(path: &Path) -> Option<Self> {
        let filename = path.file_name()?.to_string_lossy().to_string();
        let metadata = fs::metadata(path).ok()?;
        let size = metadata.len();

        let mut file = fs::File::open(path).ok()?;
        let mut content = String::new();
        file.read_to_string(&mut content).ok()?;

        let mut skill_name = None;
        let mut is_missing = false;
        let mut real_path_exists = false;

        // Parse frontmatter if present
        if content.starts_with("---") {
            if let Some(end_idx) = content[3..].find("---") {
                let fm_str = &content[3..3 + end_idx];
                for line in fm_str.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("name:") {
                        let val = trimmed["name:".len()..].trim().trim_matches('"').trim_matches('\'');
                        if !val.is_empty() {
                            skill_name = Some(val.to_string());
                        }
                    } else if trimmed.starts_with("missingSince:") {
                        is_missing = true;
                    } else if trimmed.starts_with("realPath:") || trimmed.starts_with("sourcePath:") {
                        let p = trimmed.split_once(':').map(|x| x.1.trim()).unwrap_or("");
                        let cleaned = p.trim_matches('"').trim_matches('\'');
                        if !cleaned.is_empty() && Path::new(cleaned).exists() {
                            real_path_exists = true;
                        }
                    }
                }
            }
        }

        // Determine tier priority
        let p_str = path.to_string_lossy().to_lowercase();
        let tier_priority = if p_str.contains("00_overview") {
            1
        } else if p_str.contains("01_fundamentals") {
            2
        } else if p_str.contains("02_concepts") {
            3
        } else if p_str.contains("03_advanced") {
            4
        } else if p_str.contains("04_projects_and_labs") {
            5
        } else if p_str.contains("ai skills manager") {
            95 // Obsolete staging directory, lower priority than 03_Advanced
        } else if p_str.contains("99_archive") {
            99
        } else {
            50
        };

        Some(NoteMeta {
            path: path.to_path_buf(),
            filename,
            size,
            content,
            skill_name,
            is_missing,
            real_path_exists,
            tier_priority,
        })
    }
}

fn is_ignored_path(p: &Path) -> bool {
    let s = p.to_string_lossy().to_lowercase();
    s.contains(".obsidian")
        || s.contains(".smart-env")
        || s.contains(".claudian")
        || s.contains(".git")
        || s.contains("\\obsidian\\")
        || s.ends_with("\\obsidian")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let execute = args.iter().any(|a| a == "--execute" || a == "-e");
    let vault_path = PathBuf::from(r"J:\Cybersecurity_Vault");

    println!("=======================================================================");
    println!("🦀 NATIVE RUST SECOND-BRAIN VAULT NODE DEDUPLICATOR");
    println!("Target Vault   : {:?}", vault_path);
    println!("Execution Mode : {}", if execute { "🚀 LIVE EXECUTION (PERMANENT DELETION)" } else { "🔍 DRY RUN / AUDIT ONLY (NO FILES DELETED)" });
    println!("=======================================================================\n");

    if !vault_path.exists() {
        eprintln!("[-] Error: Vault path {:?} does not exist!", vault_path);
        std::process::exit(1);
    }

    // 1. Traverse and index all markdown notes
    println!("[*] Scanning Obsidian vault nodes...");
    let mut notes: Vec<NoteMeta> = Vec::new();

    for entry in WalkDir::new(&vault_path).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.is_file() && !is_ignored_path(p) {
            let ext = p.extension().and_then(|x| x.to_str()).unwrap_or("").to_lowercase();
            if ext == "md" || ext == "canvas" {
                if let Some(meta) = NoteMeta::from_path(p) {
                    notes.push(meta);
                }
            }
        }
    }

    println!("[+] Indexed {} notes and canvas nodes across vault.\n", notes.len());

    let mut to_delete: HashSet<PathBuf> = HashSet::new();
    let mut kept_records: Vec<(String, PathBuf, Vec<PathBuf>)> = Vec::new();

    // -------------------------------------------------------------------------
    // VECTOR 1: Skill Deduplication by `name:` in Frontmatter
    // -------------------------------------------------------------------------
    println!("-----------------------------------------------------------------------");
    println!("🔍 VECTOR 1: Deduplicating AI & Antigravity Skill Nodes");
    println!("-----------------------------------------------------------------------");

    let mut skill_groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, note) in notes.iter().enumerate() {
        if let Some(ref sname) = note.skill_name {
            skill_groups.entry(sname.clone()).or_default().push(idx);
        }
    }

    let mut skill_dup_count = 0;
    for (sname, indices) in skill_groups {
        if indices.len() > 1 {
            // Sort by:
            // 1. Lower tier_priority (prefer 03_Advanced over AI Skills Manager)
            // 2. Not missing (false comes before true)
            // 3. Real path exists on host filesystem (true comes before false)
            // 4. Prefer "global-skill-" or active prefix over obsolete
            // 5. Higher size
            let mut sorted = indices.clone();
            sorted.sort_by(|&a, &b| {
                let na = &notes[a];
                let nb = &notes[b];

                na.tier_priority.cmp(&nb.tier_priority)
                    .then_with(|| na.is_missing.cmp(&nb.is_missing))
                    .then_with(|| nb.real_path_exists.cmp(&na.real_path_exists))
                    .then_with(|| {
                        let a_is_global = na.filename.starts_with("global-skill-");
                        let b_is_global = nb.filename.starts_with("global-skill-");
                        b_is_global.cmp(&a_is_global)
                    })
                    .then_with(|| nb.size.cmp(&na.size))
            });

            let best_idx = sorted[0];
            let best_note = &notes[best_idx];
            let mut removed_paths = Vec::new();

            for &other_idx in &sorted[1..] {
                let bad_note = &notes[other_idx];
                to_delete.insert(bad_note.path.clone());
                removed_paths.push(bad_note.path.clone());
                skill_dup_count += 1;
            }

            kept_records.push((
                format!("Skill Node: {}", sname),
                best_note.path.clone(),
                removed_paths,
            ));
        }
    }

    println!("[+] Found {} duplicate skill notes across {} skill clusters.", skill_dup_count, kept_records.len());

    // -------------------------------------------------------------------------
    // VECTOR 2: Exact Full Content Deduplication (Byte-for-byte Hash)
    // -------------------------------------------------------------------------
    println!("\n-----------------------------------------------------------------------");
    println!("🔍 VECTOR 2: Exact Content Deduplication");
    println!("-----------------------------------------------------------------------");

    let mut content_groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, note) in notes.iter().enumerate() {
        if note.size > 0 && !to_delete.contains(&note.path) {
            content_groups.entry(note.content.clone()).or_default().push(idx);
        }
    }

    let mut content_dup_count = 0;
    for (_content, indices) in content_groups {
        if indices.len() > 1 {
            let mut sorted = indices.clone();
            sorted.sort_by(|&a, &b| {
                let na = &notes[a];
                let nb = &notes[b];
                na.tier_priority.cmp(&nb.tier_priority)
                    .then_with(|| na.path.to_string_lossy().len().cmp(&nb.path.to_string_lossy().len()))
            });

            let best_idx = sorted[0];
            let best_note = &notes[best_idx];
            let mut removed_paths = Vec::new();

            for &other_idx in &sorted[1..] {
                let bad_note = &notes[other_idx];
                if to_delete.insert(bad_note.path.clone()) {
                    removed_paths.push(bad_note.path.clone());
                    content_dup_count += 1;
                }
            }

            if !removed_paths.is_empty() {
                kept_records.push((
                    format!("Identical Content: {}", best_note.filename),
                    best_note.path.clone(),
                    removed_paths,
                ));
            }
        }
    }

    println!("[+] Found {} additional exact duplicate content notes.", content_dup_count);

    // -------------------------------------------------------------------------
    // VECTOR 3: Duplicate Filenames with Redundant Archives
    // -------------------------------------------------------------------------
    println!("\n-----------------------------------------------------------------------");
    println!("🔍 VECTOR 3: Duplicate Filenames Across Tiers");
    println!("-----------------------------------------------------------------------");

    let mut filename_groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, note) in notes.iter().enumerate() {
        if !to_delete.contains(&note.path) {
            filename_groups.entry(note.filename.to_lowercase()).or_default().push(idx);
        }
    }

    let mut filename_dup_count = 0;
    for (fname, indices) in filename_groups {
        if indices.len() > 1 {
            let mut active_indices = Vec::new();
            let mut archive_indices = Vec::new();

            for &idx in &indices {
                if notes[idx].tier_priority < 99 {
                    active_indices.push(idx);
                } else {
                    archive_indices.push(idx);
                }
            }

            if !active_indices.is_empty() && !archive_indices.is_empty() {
                let best_note = &notes[active_indices[0]];
                let mut removed_paths = Vec::new();

                for &arch_idx in &archive_indices {
                    let arch_note = &notes[arch_idx];
                    if to_delete.insert(arch_note.path.clone()) {
                        removed_paths.push(arch_note.path.clone());
                        filename_dup_count += 1;
                    }
                }

                if !removed_paths.is_empty() {
                    kept_records.push((
                        format!("Filename Collision: {}", fname),
                        best_note.path.clone(),
                        removed_paths,
                    ));
                }
            }
        }
    }

    println!("[+] Found {} archived collision notes matching active canonical notes.", filename_dup_count);

    // -------------------------------------------------------------------------
    // SUMMARY & EXECUTION
    // -------------------------------------------------------------------------
    println!("\n=======================================================================");
    println!("📊 DEDUPLICATION SUMMARY");
    println!("=======================================================================");
    println!("Total Unique Duplicate Files Marked for Removal: {}", to_delete.len());

    let mut bytes_saved: u64 = 0;
    for p in &to_delete {
        if let Ok(meta) = fs::metadata(p) {
            bytes_saved += meta.len();
        }
    }
    println!("Total Disk Space to Reclaim: {:.2} KB ({:.2} MB)", bytes_saved as f64 / 1024.0, bytes_saved as f64 / (1024.0 * 1024.0));

    // Sample details
    println!("\nSample Duplicate Clusters (First 10):");
    for (i, (desc, kept, removed)) in kept_records.iter().take(10).enumerate() {
        println!("  Cluster #{}: {}", i + 1, desc);
        println!("    [KEEP] -> {}", kept.display());
        for r in removed {
            println!("    [DROP] -> {}", r.display());
        }
    }

    if execute {
        println!("\n🚀 EXECUTING PERMANENT DELETION OF {} DUPLICATE NODES...", to_delete.len());
        let mut deleted_count = 0;
        let mut error_count = 0;

        for p in &to_delete {
            match fs::remove_file(p) {
                Ok(_) => {
                    deleted_count += 1;
                }
                Err(e) => {
                    eprintln!("  [!] Failed to remove {:?}: {}", p, e);
                    error_count += 1;
                }
            }
        }

        // Clean up empty directories
        println!("[*] Cleaning up empty folders in vault...");
        let mut empty_dirs_removed = 0;
        for entry in WalkDir::new(&vault_path).contents_first(true).into_iter().filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_dir() && !is_ignored_path(p) && p != vault_path {
                if let Ok(mut read_dir) = fs::read_dir(p) {
                    if read_dir.next().is_none() {
                        if fs::remove_dir(p).is_ok() {
                            empty_dirs_removed += 1;
                        }
                    }
                }
            }
        }

        println!("[✓] Cleaned {} empty directories.", empty_dirs_removed);
        println!("[✓] Successfully deleted {} duplicate nodes (Errors: {}).", deleted_count, error_count);
        println!("[✓] Final Verified Vault Note Count: {}", notes.len() - deleted_count);
    } else {
        println!("\n[i] Dry run complete. To execute permanent deduplication, re-run with: --execute");
    }
}
