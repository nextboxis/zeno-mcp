// src/agents/cyber_operator.rs
// Autonomous Cyber Warfare & Red Team Operator in Pure Native Rust.

use crate::client::AntigravityClient;
use crate::skills::SkillsRegistry;
use anyhow::Result;

pub const CYBER_OPERATOR_SYSTEM: &str = r#"
You are the Lead Autonomous AI Red Team & Offensive Cybersecurity Architect.
You execute professional, rigorous evaluations of AI systems, APIs, and cloud infrastructure.

INVARIANTS:
1. Apply OWASP Top 10 for LLMs (Prompt Injection, Insecure Output Handling, SSRF via Tools).
2. All reported findings must feature CVSS v3.1 / SSVC metrics, PoC payload, and code remediation.
3. Strict defensive adherence: No destructive operations outside authorized targets.
"#;

pub struct CyberOperatorAgent {
    client: AntigravityClient,
    skills: SkillsRegistry,
}

impl CyberOperatorAgent {
    pub fn new(client: AntigravityClient, skills: SkillsRegistry) -> Self {
        Self { client, skills }
    }

    pub async fn execute_assessment(&self, mission_prompt: &str) -> Result<String> {
        let skill_count = self.skills.skills.len();
        
        // Real-Time Live Internet Target Probing if a URL is provided
        let mut live_telemetry = String::new();
        if let Some(target_url) = extract_url(mission_prompt) {
            if let Some(audit) = probe_live_target(target_url).await {
                live_telemetry = audit;
            }
        }

        let enriched_prompt = format!(
            "Mission Objective: {mission_prompt}\n\n\
             Refer to your loaded {skill_count} Cybersecurity Skills to produce an executive-ready \
             threat modeling and remediation report with CVSS ratings and guardrail rules."
        );

        let mut base_report = self.client.chat(CYBER_OPERATOR_SYSTEM, &enriched_prompt).await?;
        if !live_telemetry.is_empty() {
            base_report.push_str(&live_telemetry);
        }
        Ok(base_report)
    }
}

fn extract_url(text: &str) -> Option<&str> {
    for word in text.split_whitespace() {
        if word.starts_with("http://") || word.starts_with("https://") {
            return Some(word.trim_matches(|c| c == '"' || c == '\'' || c == ',' || c == ';' || c == ')'));
        }
    }
    None
}

async fn probe_live_target(url: &str) -> Option<String> {
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .ok()?;

    let start = std::time::Instant::now();
    let resp = http_client.get(url).send().await.ok()?;
    let elapsed = start.elapsed();

    let status = resp.status();
    let headers = resp.headers();

    let has_hsts = headers.contains_key("strict-transport-security");
    let has_csp = headers.contains_key("content-security-policy");
    let has_xfo = headers.contains_key("x-frame-options");
    let has_xcto = headers.contains_key("x-content-type-options");
    let has_referrer = headers.contains_key("referrer-policy");
    let has_permissions = headers.contains_key("permissions-policy");

    let server = headers
        .get("server")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Protected / Undisclosed");

    let is_outdated_server = server.to_lowercase().contains("centos") || server.to_lowercase().contains("2.4.6");

    let mut audit = format!(
        "\n\n---\n### 🌐 REAL-TIME LIVE INTERNET TARGET AUDIT (OWASP SECURE HEADERS)\n\
         - **Target URL**: `{}`\n\
         - **HTTP Status**: `{}`\n\
         - **Round-Trip Latency**: `{:.2} ms`\n\
         - **Server Banner**: `{}` {}\n\
         - **HSTS (Strict-Transport-Security)**: {}\n\
         - **CSP (Content-Security-Policy)**: {}\n\
         - **X-Frame-Options (Clickjacking)**: {}\n\
         - **X-Content-Type-Options (MIME Sniffing)**: {}\n\
         - **Referrer-Policy**: {}\n\
         - **Permissions-Policy**: {}\n",
        url,
        status,
        elapsed.as_secs_f64() * 1000.0,
        server,
        if is_outdated_server { "⚠️ (Outdated / EOL OS & Server Detected)" } else { "" },
        if has_hsts { "✅ Present" } else { "❌ MISSING (Risk: SSL Stripping)" },
        if has_csp { "✅ Present" } else { "❌ MISSING (Risk: XSS / Injection)" },
        if has_xfo { "✅ Present" } else { "❌ MISSING (Risk: Clickjacking)" },
        if has_xcto { "✅ Present" } else { "❌ MISSING (Risk: MIME-type attacks)" },
        if has_referrer { "✅ Present" } else { "⚠️ MISSING (Default: No-Referrer)" },
        if has_permissions { "✅ Present" } else { "⚠️ MISSING (Hardware APIs active)" }
    );

    // Auto-generate remediation code if missing critical headers are detected
    if !has_csp || !has_xfo || is_outdated_server {
        let is_apache = server.to_lowercase().contains("apache");
        audit.push_str("\n#### 🛠️ AUTOMATED REMEDIATION PATCH DIRECTIVE:\n");
        if is_apache {
            audit.push_str(
                "```apache\n\
                 # Insert into Apache httpd.conf or .htaccess\n\
                 Header always set X-Frame-Options \"DENY\"\n\
                 Header always set Content-Security-Policy \"default-src 'self'; frame-ancestors 'none';\"\n\
                 Header always set X-Content-Type-Options \"nosniff\"\n\
                 ServerTokens Prod\n\
                 ServerSignature Off\n\
                 ```\n"
            );
        } else {
            audit.push_str(
                "```nginx\n\
                 # Insert into Nginx server block\n\
                 add_header X-Frame-Options \"DENY\" always;\n\
                 add_header Content-Security-Policy \"default-src 'self'; frame-ancestors 'none';\" always;\n\
                 add_header X-Content-Type-Options \"nosniff\" always;\n\
                 server_tokens off;\n\
                 ```\n"
            );
        }
    }

    Some(audit)
}


