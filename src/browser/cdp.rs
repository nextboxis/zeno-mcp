// src/browser/cdp.rs
// Native Headless Chrome / Edge CDP Engine in Pure Rust.
// Zero Node.js / Zero Python Puppeteer dependencies.

use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::time::Instant;
use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct RenderedPage {
    pub url: String,
    pub title: String,
    pub latency_ms: f64,
    pub html_size: usize,
    pub meta_csp: Option<String>,
    pub inline_script_count: usize,
    pub form_count: usize,
    pub input_fields: Vec<String>,
    pub text_snippet: String,
}

pub struct HeadlessBrowser {
    executable_path: PathBuf,
}

impl HeadlessBrowser {
    pub fn new() -> Result<Self> {
        let mut candidates = vec![
            PathBuf::from(r"C:\Program Files\Google\Chrome\Application\chrome.exe"),
            PathBuf::from(r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"),
            PathBuf::from(r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe"),
            PathBuf::from(r"C:\Program Files\Microsoft\Edge\Application\msedge.exe"),
        ];

        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            candidates.push(PathBuf::from(&local_app_data).join(r"Google\Chrome\Application\chrome.exe"));
            candidates.push(PathBuf::from(&local_app_data).join(r"Microsoft\Edge\Application\msedge.exe"));
        }

        for path in &candidates {
            if path.exists() {
                return Ok(Self {
                    executable_path: path.clone(),
                });
            }
        }

        bail!("No compatible Chrome or Edge headless browser found on system");
    }

    pub async fn fetch_rendered_dom(&self, url: &str) -> Result<RenderedPage> {
        let start = Instant::now();

        let output = Command::new(&self.executable_path)
            .args([
                "--headless=new",
                "--dump-dom",
                "--disable-gpu",
                "--no-sandbox",
                "--virtual-time-budget=6000",
                url,
            ])
            .output()
            .await
            .context("Failed to spawn headless browser process")?;

        let elapsed = start.elapsed();
        let latency_ms = elapsed.as_secs_f64() * 1000.0;

        let dom_content = String::from_utf8_lossy(&output.stdout).to_string();
        let html_size = dom_content.len();

        let title = extract_tag_content(&dom_content, "title").unwrap_or_else(|| "No Title".to_string());
        let meta_csp = extract_meta_csp(&dom_content);
        let inline_script_count = dom_content.matches("<script").count();
        let form_count = dom_content.matches("<form").count();
        let input_fields = extract_inputs(&dom_content);
        let text_snippet = extract_readable_text(&dom_content, 400);

        Ok(RenderedPage {
            url: url.to_string(),
            title,
            latency_ms,
            html_size,
            meta_csp,
            inline_script_count,
            form_count,
            input_fields,
            text_snippet,
        })
    }

    pub fn format_audit_report(&self, page: &RenderedPage) -> String {
        format!(
            "\n### 🌐 NATIVE HEADLESS CHROME DOM AUDIT\n\
             - **Target URL**: `{}`\n\
             - **Rendered Title**: `{}`\n\
             - **Render Latency**: `{:.2} ms`\n\
             - **Rendered DOM Size**: `{:.2} KB`\n\
             - **Forms Detected**: `{}`\n\
             - **Input Field Fingerprint**: `{:?}`\n\
             - **Inline Scripts Count**: `{}` (Risk: High attack surface for XSS if untrusted)\n\
             - **Meta Content-Security-Policy**: {}\n\n\
             #### 📄 Rendered Text Preview:\n\
             ```text\n{}\n```",
            page.url,
            page.title,
            page.latency_ms,
            page.html_size as f64 / 1024.0,
            page.form_count,
            page.input_fields,
            page.inline_script_count,
            match &page.meta_csp {
                Some(csp) => format!("`{csp}`"),
                None => "❌ None detected in DOM `<meta>` headers".to_string(),
            },
            page.text_snippet
        )
    }
}

fn extract_tag_content(html: &str, tag: &str) -> Option<String> {
    let open_tag = format!("<{tag}>");
    let close_tag = format!("</{tag}>");
    let start = html.find(&open_tag)? + open_tag.len();
    let end = html[start..].find(&close_tag)?;
    Some(html[start..start + end].trim().to_string())
}

fn extract_meta_csp(html: &str) -> Option<String> {
    if let Some(pos) = html.to_lowercase().find("content-security-policy") {
        let snippet = &html[pos..pos + 120.min(html.len() - pos)];
        return Some(snippet.replace(['"', '\''], ""));
    }
    None
}

fn extract_inputs(html: &str) -> Vec<String> {
    let mut inputs = Vec::new();
    let lower = html.to_lowercase();
    let mut search_pos = 0;
    while let Some(pos) = lower[search_pos..].find("<input") {
        let start = search_pos + pos;
        let end = lower[start..].find('>').unwrap_or(30);
        let tag = &html[start..start + end + 1];
        if let Some(name_pos) = tag.to_lowercase().find("name=") {
            let name_part = &tag[name_pos + 5..];
            let name = name_part.trim_matches(|c| c == '"' || c == '\'' || c == ' ');
            let clean = name.split_whitespace().next().unwrap_or("unnamed");
            inputs.push(clean.to_string());
        }
        search_pos = start + end + 1;
        if inputs.len() >= 8 {
            break;
        }
    }
    inputs
}

fn extract_readable_text(html: &str, max_len: usize) -> String {
    let mut in_tag = false;
    let mut text = String::new();

    for ch in html.chars() {
        if ch == '<' {
            in_tag = true;
        } else if ch == '>' {
            in_tag = false;
            text.push(' ');
        } else if !in_tag && !ch.is_control() {
            text.push(ch);
            if text.len() >= max_len {
                break;
            }
        }
    }

    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
